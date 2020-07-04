use choco_macro::NSObjectProtocol;
use std::ptr::NonNull;

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

pub type NSInteger = isize;
pub type NSUInteger = usize;

#[repr(C)]
struct OpaqueObjCObject {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct OpaqueObjCClass {
    _private: [u8; 0],
}

extern "C" {
    fn choco_base_NSObjectProtocol_instance_hash(self_: RawObjCPtr) -> NSUInteger;
    fn choco_base_NSObjectProtocol_instance_isEqual(
        self_: RawObjCPtr,
        other: RawNullableObjCPtr,
    ) -> BOOL;
    fn choco_base_NSObjectProtocol_instance_isKindOfClass(
        self_: RawObjCPtr,
        class: ObjCClassPtr,
    ) -> BOOL;
    fn choco_base_NSObjectProtocol_instance_description(self_: RawObjCPtr) -> RawNullableObjCPtr;
    fn choco_base_NSObjectProtocol_instance_debugDescription(
        self_: RawObjCPtr,
    ) -> RawNullableObjCPtr;

    fn choco_base_NSObject_class() -> NullableObjCClassPtr;
    fn choco_base_NSObjectInterface_class_new(class: ObjCClassPtr) -> RawNullableObjCPtr;
}

// ARC runtime support - https://clang.llvm.org/docs/AutomaticReferenceCounting.html#runtime-support
#[link(name = "objc", kind = "dylib")]
extern "C" {
    // fn objc_autoreleasePoolPush() -> *const std::ffi::c_void;
    // fn objc_autoreleasePoolPop(pool: *const std::ffi::c_void);
    fn objc_release(value: RawObjCPtr);
    fn objc_retain(value: RawObjCPtr) -> RawNullableObjCPtr;
    fn class_getName(class: ObjCClassPtr) -> *const i8;
}

// I would like to use Option<ObjCClassPtr> instead but I'm not sure its memory layout is the same.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct NullableObjCClassPtr {
    ptr: Option<NonNull<OpaqueObjCClass>>,
}

impl NullableObjCClassPtr {
    pub fn into_opt(self) -> Option<ObjCClassPtr> {
        self.ptr.map(|ptr| ObjCClassPtr { ptr })
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ObjCClassPtr {
    ptr: NonNull<OpaqueObjCClass>,
}

impl From<ObjCClassPtr> for NullableObjCClassPtr {
    fn from(ptr: ObjCClassPtr) -> Self {
        Self { ptr: Some(ptr.ptr) }
    }
}

impl ObjCClassPtr {
    pub fn class_name(&self) -> &str {
        unsafe {
            let ptr = class_getName(*self);
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("expecting class_getName() to return a non null pointer")
        }
    }
}

// I would like to use Option<RawObjCPtr> instead but I'm not sure its memory layout is the same.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RawNullableObjCPtr {
    ptr: Option<NonNull<OpaqueObjCObject>>,
}

impl RawNullableObjCPtr {
    pub fn empty() -> Self {
        RawNullableObjCPtr { ptr: None }
    }

    pub fn into_opt(self) -> Option<RawObjCPtr> {
        self.ptr.map(|ptr| RawObjCPtr { ptr })
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RawObjCPtr {
    ptr: NonNull<OpaqueObjCObject>,
}

impl From<RawObjCPtr> for RawNullableObjCPtr {
    fn from(ptr: RawObjCPtr) -> Self {
        Self { ptr: Some(ptr.ptr) }
    }
}

#[repr(transparent)]
pub struct OwnedObjCPtr {
    raw: RawObjCPtr,
}
// TODO: Do we really need that...?
unsafe impl Send for OwnedObjCPtr {}
unsafe impl Sync for OwnedObjCPtr {}

impl OwnedObjCPtr {
    /// # Safety
    /// You must be sure that you own the pointer.
    /// The pointer will be released when we go out of scope.
    pub unsafe fn from_raw_unchecked(raw: RawObjCPtr) -> Self {
        Self { raw }
    }

    pub fn as_raw(&self) -> RawObjCPtr {
        self.raw
    }
}

impl Drop for OwnedObjCPtr {
    fn drop(&mut self) {
        unsafe {
            objc_release(self.as_raw());
        }
    }
}

impl Clone for OwnedObjCPtr {
    fn clone(&self) -> Self {
        let raw = unsafe { objc_retain(self.as_raw()) }
            .into_opt()
            .expect("expecting objc_retain() to return a non null pointer");
        unsafe { Self::from_raw_unchecked(raw) }
    }
}

pub trait NSObjectProtocol
where
    // to be able to have default implementations of methods returning Self
    Self: Sized,
    // all objects should be clonable (here cloning just means increasing the refcount)
    Self: Clone,
{
    /// Owned version of the type. Most of the time it will be Self.
    type Owned: NSObjectProtocol;

    /// Objective-C class this struct represents.
    fn class() -> ObjCClassPtr;

    /// Create a new struct owning its Objective-C pointer, without doing any check.
    ///
    /// # Safety
    /// You must be sure that Objective-C pointer is of the correct type, and that you own it.
    /// The pointer will be released this struct goes out of scope.
    unsafe fn from_owned_raw_unchecked(raw: RawObjCPtr) -> Self::Owned {
        Self::from_owned_unchecked(OwnedObjCPtr::from_raw_unchecked(raw))
    }

    /// Create a new struct owning its Objective-C pointer, from a non-owning pointer, without doing any check.
    ///
    /// # Safety
    /// You must be sure that Objective-C pointer is of the correct type, and that you do not own it.
    unsafe fn retain_unowned_raw_unchecked(unowned_raw: RawObjCPtr) -> Self::Owned {
        let owned_raw = objc_retain(unowned_raw)
            .into_opt()
            .expect("expecting objc_retain() to return a non null pointer");
        Self::from_owned_raw_unchecked(owned_raw)
    }

    /// Create a new struct owning its Objective-C pointer, without doing any check.
    ///
    /// # Safety
    /// You must be sure that Objective-C pointer is of the correct type.
    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned;

    fn as_raw(&self) -> RawObjCPtr;

    fn hash(&self) -> usize {
        unsafe { choco_base_NSObjectProtocol_instance_hash(self.as_raw()) }
    }
    // In Objective-C, the parameter to -[NSObject isEqual:] is nullable,
    // but that's not very useful and makes things hard to use in Rust so here we consider it non-nullable.
    fn is_equal(&self, obj: &impl NSObjectProtocol) -> bool {
        let self_raw = self.as_raw();
        let obj_raw = obj.as_raw();
        let ret = unsafe { choco_base_NSObjectProtocol_instance_isEqual(self_raw, obj_raw.into()) };
        ret.into()
    }

    fn is_kind_of(&self, class: ObjCClassPtr) -> bool {
        let self_raw = self.as_raw();
        let ret = unsafe { choco_base_NSObjectProtocol_instance_isKindOfClass(self_raw, class) };
        ret.into()
    }

    fn description(&self) -> crate::foundation::NSString {
        let self_raw = self.as_raw();
        let raw_ptr = unsafe { choco_base_NSObjectProtocol_instance_description(self_raw) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[NSObject description] to return a non null pointer");
        unsafe { crate::foundation::NSString::from_owned_raw_unchecked(raw) }
    }

    fn debug_description(&self) -> crate::foundation::NSString {
        let self_raw = self.as_raw();
        let raw_ptr = unsafe { choco_base_NSObjectProtocol_instance_debugDescription(self_raw) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[NSObject debugDescription] to return a non null pointer");
        unsafe { crate::foundation::NSString::from_owned_raw_unchecked(raw) }
    }
}

pub trait NSObjectInterface: NSObjectProtocol {
    fn new() -> Self::Owned {
        let raw_ptr = unsafe { choco_base_NSObjectInterface_class_new(Self::class()) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting +[<class_name> new] to return a non null pointer");
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(base)]
pub struct NSObject {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for NSObject {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_comparison() {
        let obj1 = NSObject::new();
        let obj2 = NSObject::new();
        assert!(obj1.is_equal(&obj1));
        assert!(obj2.is_equal(&obj2));
        assert!(!obj1.is_equal(&obj2));
    }

    #[test]
    fn clone() {
        let obj1 = NSObject::new();
        let obj2 = obj1.clone();
        assert!(obj1.is_equal(&obj2));
    }
}

/// Marker trait used for handling of type parameters in NSArray and NSDictionary.
pub trait IsKindOf<T: NSObjectProtocol>: NSObjectProtocol {}
impl<T: NSObjectProtocol> IsKindOf<T> for T {}
