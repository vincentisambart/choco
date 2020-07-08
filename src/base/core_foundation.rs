// CoreFoundation's handling is very similator to Objective-C's handling.
// (nothing suprising as they have been made to be used in concert)
// FFI is simpler as we're just calling C functions.

use std::ptr::NonNull;

pub type CFIndex = isize;
pub type CFHashCode = usize;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Boolean(u8);

impl From<Boolean> for bool {
    fn from(b: Boolean) -> Self {
        b.0 != 0
    }
}

impl From<bool> for Boolean {
    fn from(b: bool) -> Self {
        Self(if b { 1 } else { 0 })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFTypeID(usize);

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(cf: RawCFTypeRef);
    fn CFRetain(cf: RawCFTypeRef) -> RawNullableCFTypeRef;
    fn CFShow(cf: RawCFTypeRef);
    fn CFGetRetainCount(cf: RawCFTypeRef) -> CFIndex;
    fn CFHash(cf: RawCFTypeRef) -> CFHashCode;
    fn CFEqual(cf1: RawCFTypeRef, cf2: RawCFTypeRef) -> Boolean;
    fn CFGetTypeID(cf: RawCFTypeRef) -> CFTypeID;
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct RawCFTypeRef {
    ptr: NonNull<std::ffi::c_void>,
}

// I would like to use Option<CFTypeRef> instead but I'm not sure its memory layout is the same.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct RawNullableCFTypeRef {
    ptr: Option<NonNull<std::ffi::c_void>>,
}

impl RawNullableCFTypeRef {
    pub fn into_opt(self) -> Option<RawCFTypeRef> {
        self.ptr.map(|ptr| RawCFTypeRef { ptr })
    }
}

impl From<RawCFTypeRef> for RawNullableCFTypeRef {
    fn from(ptr: RawCFTypeRef) -> Self {
        Self { ptr: Some(ptr.ptr) }
    }
}

#[repr(transparent)]
pub struct OwnedCFTypeRef {
    raw: RawCFTypeRef,
}

impl OwnedCFTypeRef {
    /// # Safety
    /// You must be sure that you own the pointer.
    /// The pointer will be released when we go out of scope.
    pub unsafe fn from_raw_unchecked(raw: RawCFTypeRef) -> Self {
        Self { raw }
    }

    pub fn as_raw(&self) -> RawCFTypeRef {
        self.raw
    }
}

impl Drop for OwnedCFTypeRef {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_raw());
        }
    }
}

impl Clone for OwnedCFTypeRef {
    fn clone(&self) -> Self {
        let raw = unsafe { CFRetain(self.as_raw()) }
            .into_opt()
            .expect("expecting CFRetain() to return a non null pointer");
        unsafe { Self::from_raw_unchecked(raw) }
    }
}

pub trait CFTypeRef
where
    // to be able to have default implementations of methods returning Self
    Self: Sized,
    // all objects should be clonable (here cloning just means increasing the refcount)
    Self: Clone,
{
    fn as_raw(&self) -> RawCFTypeRef;

    fn equal(&self, other: &impl CFTypeRef) -> bool {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();
        let ret = unsafe { CFEqual(self_raw, other_raw) };
        ret.into()
    }

    fn show(&self) {
        let self_raw = self.as_raw();
        unsafe { CFShow(self_raw) };
    }

    fn retain_count(&self) -> isize {
        let self_raw = self.as_raw();
        unsafe { CFGetRetainCount(self_raw) }
    }

    fn hash(&self) -> usize {
        let self_raw = self.as_raw();
        unsafe { CFHash(self_raw) }
    }

    fn type_id(&self) -> CFTypeID {
        let self_raw = self.as_raw();
        unsafe { CFGetTypeID(self_raw) }
    }
}
