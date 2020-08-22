use crate::base::ptr;
use std::ptr::NonNull;

pub type NSInteger = isize;
pub type NSUInteger = usize;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BOOL(i8);

impl From<BOOL> for bool {
    fn from(b: BOOL) -> Self {
        b.0 != 0
    }
}

impl From<bool> for BOOL {
    fn from(b: bool) -> Self {
        Self(if b { 1 } else { 0 })
    }
}

// TODO: Move NSObject to foundation, even though technically it's part of the runtime.
extern "C" {
    fn choco_base_NSObjectProtocol_instance_hash(self_: ptr::RawPtr) -> NSUInteger;
    fn choco_base_NSObjectProtocol_instance_isEqual(
        self_: ptr::RawPtr,
        other: Option<ptr::RawPtr>,
    ) -> BOOL;
    fn choco_base_NSObjectProtocol_instance_isKindOfClass(
        self_: ptr::RawPtr,
        class: ptr::ClassPtr,
    ) -> BOOL;
    fn choco_base_NSObjectProtocol_instance_description(self_: ptr::RawPtr) -> Option<ptr::RawPtr>;
    fn choco_base_NSObjectProtocol_instance_debugDescription(
        self_: ptr::RawPtr,
    ) -> Option<ptr::RawPtr>;

    fn choco_base_NSObject_class() -> ptr::ClassPtr;
    fn choco_base_NSObjectInterface_class_new(class: ptr::ClassPtr) -> Option<ptr::RawPtr>;
}

/// Marker trait used for handling of type parameters in NSArray and NSDictionary.
pub unsafe trait IsKindOf<T: ptr::Type>: ptr::Type {}
unsafe impl<T: ptr::Type> IsKindOf<T> for T {}

pub trait NSObjectProtocol
where
    Self: ptr::AsRaw,
    Self: Sized,
{
    type Class: ptr::ObjCClass;

    fn hash(&self) -> usize {
        unsafe { choco_base_NSObjectProtocol_instance_hash(self.as_raw()) }
    }

    // In Objective-C, the parameter to -[NSObject isEqual:] is nullable,
    // we consider it non-nullable to makes things simpler.
    fn is_equal(&self, obj: &impl NSObjectProtocol) -> bool {
        let self_raw = self.as_raw();
        let obj_raw = obj.as_raw();
        let ret = unsafe { choco_base_NSObjectProtocol_instance_isEqual(self_raw, obj_raw.into()) };
        ret.into()
    }

    fn is_kind_of(&self, class: ptr::ClassPtr) -> bool {
        let self_raw = self.as_raw();
        let ret = unsafe { choco_base_NSObjectProtocol_instance_isKindOfClass(self_raw, class) };
        ret.into()
    }

    fn description(&self) -> ptr::OwnedPtr<crate::foundation::NSString> {
        let self_raw = self.as_raw();
        unsafe {
            let raw = choco_base_NSObjectProtocol_instance_description(self_raw).unwrap();
            ptr::OwnedPtr::from_owned_raw_unchecked(raw)
        }
    }

    fn debug_description(&self) -> ptr::OwnedPtr<crate::foundation::NSString> {
        let self_raw = self.as_raw();
        unsafe {
            let raw = choco_base_NSObjectProtocol_instance_debugDescription(self_raw).unwrap();
            ptr::OwnedPtr::from_owned_raw_unchecked(raw)
        }
    }
}

pub trait NSObjectInterface: NSObjectProtocol {}

pub trait NSObjectInterfaceClassMethods: ptr::ObjCClass
where
    Self: Sized,
{
    fn new() -> ptr::OwnedPtr<Self> {
        unsafe {
            let raw =
                choco_base_NSObjectInterface_class_new(<Self as ptr::ObjCClass>::class()).unwrap();
            ptr::OwnedPtr::from_owned_raw_unchecked(raw)
        }
    }
}

pub struct NSObject {}

impl ptr::Type for NSObject {
    const KIND: ptr::TypeKind = ptr::TypeKind::ObjC;
}

impl ptr::ObjCClass for NSObject {
    fn class() -> ptr::ClassPtr {
        unsafe { choco_base_NSObject_class() }
    }
}

impl NSObjectInterfaceClassMethods for NSObject {}

impl NSObjectProtocol for ptr::OwnedPtr<NSObject> {
    type Class = NSObject;
}
impl NSObjectInterface for ptr::OwnedPtr<NSObject> {}

impl<LhsClass: ptr::ObjCClass, Rhs: NSObjectInterface> std::cmp::PartialEq<Rhs>
    for ptr::OwnedPtr<LhsClass>
where
    ptr::OwnedPtr<LhsClass>: NSObjectProtocol,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.is_equal(other)
    }
}

#[cfg(test)]
#[allow(clippy::eq_op)]
mod tests {
    use super::*;

    #[test]
    fn object_comparison() {
        let obj1 = NSObject::new();
        let obj2 = NSObject::new();
        assert!(obj1.is_equal(&obj1));
        assert!(obj2.is_equal(&obj2));
        assert!(!obj1.is_equal(&obj2));
        assert!(obj1 == obj1);
        assert!(obj2 == obj2);
        assert!(obj1 != obj2);
    }

    #[test]
    fn retain() {
        let obj1 = NSObject::new();
        let obj2 = obj1.retain();
        assert!(obj1.is_equal(&obj2));
        assert!(obj1 == obj2);
    }
}

#[repr(C)]
struct OpaqueAutoreleasePool {
    _private: [u8; 0],
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_autoreleasePoolPush() -> Option<NonNull<OpaqueAutoreleasePool>>;
    fn objc_autoreleasePoolPop(pool: NonNull<OpaqueAutoreleasePool>);
}

struct AutoreleasePoolGuard {
    pool: NonNull<OpaqueAutoreleasePool>,
}

impl AutoreleasePoolGuard {
    fn push() -> AutoreleasePoolGuard {
        let pool = unsafe { objc_autoreleasePoolPush() }
            .expect("expecting objc_autoreleasePoolPush() to return a non-null value");
        AutoreleasePoolGuard { pool }
    }
}

impl Drop for AutoreleasePoolGuard {
    fn drop(&mut self) {
        unsafe { objc_autoreleasePoolPop(self.pool) }
    }
}

pub fn autorelease_pool<Ret, F: FnOnce() -> Ret>(f: F) -> Ret {
    let _pool = AutoreleasePoolGuard::push();
    f()
}
