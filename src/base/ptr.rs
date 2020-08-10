pub trait Retain {
    type Owned;
    fn retain(&self) -> Self::Owned;
}

pub(crate) mod objc {
    use std::ptr::NonNull;

    #[link(name = "objc", kind = "dylib")]
    extern "C" {
        fn objc_release(value: RawPtr);
        fn objc_retain(value: RawPtr) -> NullableRawPtr;
    }

    #[repr(C)]
    pub(super) struct OpaqueObject {
        _private: [u8; 0],
    }

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct RawPtr {
        pub(super) ptr: NonNull<OpaqueObject>,
    }

    impl RawPtr {
        pub unsafe fn consider_owned(self) -> OwnedPtr {
            OwnedPtr::from_owned_raw(self)
        }
    }

    impl super::Retain for RawPtr {
        type Owned = OwnedPtr;

        fn retain(&self) -> Self::Owned {
            let raw = unsafe { objc_retain(*self) }.unwrap();
            Self::Owned { raw }
        }
    }

    // I would like to use Option<RawPtr> instead but I'm not sure
    // if its memory layout is guaranted to be the same.
    #[derive(Copy, Clone, Default)]
    #[repr(transparent)]
    pub struct NullableRawPtr {
        ptr: Option<NonNull<OpaqueObject>>,
    }

    impl NullableRawPtr {
        pub fn into_opt(self) -> Option<RawPtr> {
            self.ptr.map(|ptr| RawPtr { ptr })
        }

        pub fn unwrap(self) -> RawPtr {
            self.into_opt().expect("expecting a non null pointer")
        }
    }

    impl From<RawPtr> for NullableRawPtr {
        fn from(ptr: RawPtr) -> Self {
            Self { ptr: Some(ptr.ptr) }
        }
    }

    pub trait AsRawPtr {
        fn as_raw(&self) -> RawPtr;
    }

    pub struct OwnedPtr {
        raw: RawPtr,
    }

    impl OwnedPtr {
        pub(crate) unsafe fn from_owned_raw(raw: RawPtr) -> Self {
            Self { raw }
        }
    }

    impl super::Retain for OwnedPtr {
        type Owned = Self;

        fn retain(&self) -> Self::Owned {
            let raw = unsafe { objc_retain(self.raw) }.unwrap();
            Self::Owned { raw }
        }
    }

    impl Drop for OwnedPtr {
        fn drop(&mut self) {
            unsafe {
                objc_release(self.raw);
            }
        }
    }

    impl AsRawPtr for OwnedPtr {
        fn as_raw(&self) -> RawPtr {
            self.raw
        }
    }

    #[derive(Copy, Clone)]
    pub struct StaticPtr {
        raw: RawPtr,
    }

    impl AsRawPtr for StaticPtr {
        fn as_raw(&self) -> RawPtr {
            self.raw
        }
    }

    unsafe impl Send for StaticPtr {}
    unsafe impl Sync for StaticPtr {}

    pub struct BorrowedPtr {
        raw: RawPtr,
    }

    impl super::Retain for BorrowedPtr {
        type Owned = OwnedPtr;
        fn retain(&self) -> Self::Owned {
            let raw = unsafe { objc_retain(self.raw) }.unwrap();
            Self::Owned { raw }
        }
    }

    #[repr(C)]
    pub(crate) struct OpaqueClass {
        _private: [u8; 0],
    }

    // I would like to use Option<ClassPtr> instead but I'm not sure
    // if its memory layout is guaranted to be the same.
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct NullableClassPtr {
        ptr: Option<NonNull<OpaqueClass>>,
    }

    impl NullableClassPtr {
        pub fn into_opt(self) -> Option<ClassPtr> {
            self.ptr.map(|ptr| ClassPtr { ptr })
        }

        pub fn unwrap(self) -> ClassPtr {
            self.into_opt().expect("expecting a non-null class")
        }
    }

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ClassPtr {
        ptr: NonNull<OpaqueClass>,
    }

    impl From<ClassPtr> for NullableClassPtr {
        fn from(ptr: ClassPtr) -> Self {
            Self { ptr: Some(ptr.ptr) }
        }
    }
}

pub(crate) mod cf {
    use std::ptr::NonNull;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFRelease(cf: RawRef);
        fn CFRetain(cf: RawRef) -> NullableRawRef;
    }

    #[repr(C)]
    pub(super) struct OpaqueCFType {
        _private: [u8; 0],
    }

    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct RawRef {
        pub(super) ptr: NonNull<OpaqueCFType>,
    }

    // I would like to use Option<RawRef> instead but I'm not sure
    // if its memory layout is guaranted to be the same.
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct NullableRawRef {
        ptr: Option<NonNull<OpaqueCFType>>,
    }

    impl NullableRawRef {
        fn into_opt(self) -> Option<RawRef> {
            self.ptr.map(|ptr| RawRef { ptr })
        }

        pub fn unwrap(self) -> RawRef {
            self.into_opt().expect("expecting a non-null pointer")
        }
    }

    pub struct OwnedRef {
        pub(super) raw: RawRef,
    }

    impl OwnedRef {
        pub(crate) unsafe fn from_owned_raw(raw: RawRef) -> Self {
            Self { raw }
        }
    }

    impl super::Retain for OwnedRef {
        type Owned = Self;
        fn retain(&self) -> Self::Owned {
            let raw = unsafe { CFRetain(self.raw) }.unwrap();
            Self::Owned { raw }
        }
    }

    impl Drop for OwnedRef {
        fn drop(&mut self) {
            unsafe {
                CFRelease(self.raw);
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct StaticRef {
        raw: RawRef,
    }

    unsafe impl Send for StaticRef {}
    unsafe impl Sync for StaticRef {}

    pub struct BorrowedRef {
        raw: RawRef,
    }

    impl super::Retain for BorrowedRef {
        type Owned = OwnedRef;

        fn retain(&self) -> Self::Owned {
            let raw = unsafe { CFRetain(self.raw) }.unwrap();
            Self::Owned { raw }
        }
    }
}

impl objc::AsRawPtr for cf::OwnedRef {
    fn as_raw(&self) -> objc::RawPtr {
        objc::RawPtr {
            ptr: self.raw.ptr.cast(),
        }
    }
}
