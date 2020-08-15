pub trait Retain {
    type Owned;
    fn retain(&self) -> Self::Owned;
}

/// At least one of the two methods must be implemented (or else you'll end up with infinite recursion).
pub trait AsRaw {
    fn as_raw_ptr(&self) -> objc::RawPtr {
        objc::RawPtr {
            ptr: self.as_raw_ref().ptr.cast(),
        }
    }

    fn as_raw_ref(&self) -> cf::RawRef {
        cf::RawRef {
            ptr: self.as_raw_ptr().ptr.cast(),
        }
    }
}

/// At least one of the two methods must be implemented (or else you'll end up with infinite recursion).
pub trait FromOwned: Sized {
    unsafe fn from_owned_ptr_unchecked(owned_ptr: objc::OwnedPtr) -> Self {
        Self::from_owned_ref_unchecked(owned_ptr.into())
    }

    unsafe fn from_owned_ref_unchecked(owned_ref: cf::OwnedRef) -> Self {
        Self::from_owned_ptr_unchecked(owned_ref.into())
    }
}

pub(crate) mod objc {
    use std::ptr::NonNull;

    #[link(name = "objc", kind = "dylib")]
    extern "C" {
        fn objc_release(value: RawPtr);
        fn objc_retain(value: RawPtr) -> Option<RawPtr>;
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

    // No explicit repr() because not expected to be used on FFI boundaries.
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

    impl super::AsRaw for OwnedPtr {
        fn as_raw_ptr(&self) -> RawPtr {
            self.raw
        }
    }

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct StaticPtr {
        raw: RawPtr,
    }

    impl super::AsRaw for StaticPtr {
        fn as_raw_ptr(&self) -> RawPtr {
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

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ClassPtr {
        ptr: NonNull<OpaqueClass>,
    }
}

pub(crate) mod cf {
    use std::ptr::NonNull;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFRelease(cf: RawRef);
        fn CFRetain(cf: RawRef) -> Option<RawRef>;
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

    pub struct OwnedRef {
        pub(super) raw: RawRef,
    }

    impl OwnedRef {
        pub(crate) unsafe fn from_owned_raw(raw: RawRef) -> Self {
            Self { raw }
        }
    }

    impl super::AsRaw for OwnedRef {
        fn as_raw_ref(&self) -> RawRef {
            self.raw
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

impl From<cf::OwnedRef> for objc::OwnedPtr {
    fn from(ptr: cf::OwnedRef) -> Self {
        let raw = ptr.as_raw_ptr();
        std::mem::forget(ptr);
        unsafe { Self::from_owned_raw(raw) }
    }
}

impl From<objc::OwnedPtr> for cf::OwnedRef {
    fn from(ptr: objc::OwnedPtr) -> Self {
        let raw = ptr.as_raw_ref();
        std::mem::forget(ptr);
        unsafe { Self::from_owned_raw(raw) }
    }
}
