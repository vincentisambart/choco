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
    fn choco_base_NSObjectProtocol_instance_hash(self_: ptr::objc::RawPtr) -> NSUInteger;
    fn choco_base_NSObjectProtocol_instance_isEqual(
        self_: ptr::objc::RawPtr,
        other: ptr::objc::NullableRawPtr,
    ) -> BOOL;
    fn choco_base_NSObjectProtocol_instance_isKindOfClass(
        self_: ptr::objc::RawPtr,
        class: ptr::objc::ClassPtr,
    ) -> BOOL;
    fn choco_base_NSObjectProtocol_instance_description(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_base_NSObjectProtocol_instance_debugDescription(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;

    fn choco_base_NSObject_class() -> ptr::objc::ClassPtr;
    fn choco_base_NSObjectInterface_class_new(
        class: ptr::objc::ClassPtr,
    ) -> ptr::objc::NullableRawPtr;
}

// TODO: Move to ptr module
pub trait FromOwnedPtr {
    unsafe fn from_owned_ptr_unchecked(owned_ptr: ptr::objc::OwnedPtr) -> Self;
}

// TODO: Is that really safe?
impl<T> ptr::Retain for T
where
    T: ptr::objc::AsRawPtr + FromOwnedPtr,
{
    type Owned = Self;

    fn retain(&self) -> Self::Owned {
        unsafe { Self::from_owned_ptr_unchecked(self.as_raw().retain()) }
    }
}

/// Indicates that the type can be used as type parameter for Objective-C classes like NSArray.
/// That does not include special types like StaticNSString or ImmutableNSString.
pub trait ValidObjCGeneric: ptr::objc::AsRawPtr + FromOwnedPtr {}

// TODO: IsKinfOf should maybe be unsafe
/// Marker trait used for handling of type parameters in NSArray and NSDictionary.
pub trait IsKindOf<T: ValidObjCGeneric>: ptr::objc::AsRawPtr {}
impl<T: ValidObjCGeneric> IsKindOf<T> for T {}

pub trait NSObjectProtocol
where
    Self: ptr::objc::AsRawPtr,
    Self: Sized,
{
    type Owned: FromOwnedPtr;

    /// Objective-C class represented by the struct implementing this trait..
    fn class() -> ptr::objc::ClassPtr;

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

    fn is_kind_of(&self, class: ptr::objc::ClassPtr) -> bool {
        let self_raw = self.as_raw();
        let ret = unsafe { choco_base_NSObjectProtocol_instance_isKindOfClass(self_raw, class) };
        ret.into()
    }

    fn description(&self) -> crate::foundation::NSString {
        let self_raw = self.as_raw();
        unsafe {
            let owned_ptr = choco_base_NSObjectProtocol_instance_description(self_raw)
                .unwrap()
                .consider_owned();
            crate::foundation::NSString::from_owned_ptr_unchecked(owned_ptr)
        }
    }

    fn debug_description(&self) -> crate::foundation::NSString {
        let self_raw = self.as_raw();
        unsafe {
            let owned_ptr = choco_base_NSObjectProtocol_instance_debugDescription(self_raw)
                .unwrap()
                .consider_owned();
            crate::foundation::NSString::from_owned_ptr_unchecked(owned_ptr)
        }
    }
}

pub trait NSObjectInterface: NSObjectProtocol {
    fn new() -> Self::Owned {
        unsafe {
            let owned_ptr = choco_base_NSObjectInterface_class_new(Self::class())
                .unwrap()
                .consider_owned();
            Self::Owned::from_owned_ptr_unchecked(owned_ptr)
        }
    }
}

pub struct NSObject {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::objc::AsRawPtr for NSObject {
    fn as_raw(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw()
    }
}

impl FromOwnedPtr for NSObject {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for NSObject {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_base_NSObject_class() }
    }
}

impl NSObjectInterface for NSObject {}
impl ValidObjCGeneric for NSObject {}

impl<Rhs: NSObjectInterface> std::cmp::PartialEq<Rhs> for NSObject {
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
        use ptr::Retain;

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
