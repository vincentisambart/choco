// CoreFoundation's handling is very similator to Objective-C's handling.
// (nothing suprising as they have been made to be used in concert)
// FFI is simpler as we're just calling C functions.

use crate::base::ptr;

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

#[repr(C)]
pub(crate) struct OpaqueCFType {
    _private: [u8; 0],
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFTypeID(usize);

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(cf: ptr::cf::RawRef);
    fn CFRetain(cf: ptr::cf::RawRef) -> ptr::cf::NullableRawRef;
    fn CFShow(cf: ptr::cf::RawRef);
    fn CFGetRetainCount(cf: ptr::cf::RawRef) -> CFIndex;
    fn CFHash(cf: ptr::cf::RawRef) -> CFHashCode;
    fn CFEqual(cf1: ptr::cf::RawRef, cf2: ptr::cf::RawRef) -> Boolean;
    fn CFGetTypeID(cf: ptr::cf::RawRef) -> CFTypeID;
}

pub trait CFTypeInterface
where
    // to be able to have default implementations of methods returning Self
    Self: Sized,
    // all objects should be clonable (here cloning just means increasing the reference count)
    Self: Clone,
{
    fn as_raw(&self) -> ptr::cf::RawRef;

    fn equal(&self, other: &impl CFTypeInterface) -> bool {
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
