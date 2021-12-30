// CoreFoundation's handling is very similator to Objective-C's handling.
// (nothing suprising as they have been made to be used in concert)
// FFI is simpler as we're just calling C functions.

use super::{AsRaw, RawObjPtr};

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
    fn CFShow(cf: RawObjPtr);
    fn CFGetRetainCount(cf: RawObjPtr) -> CFIndex;
    fn CFHash(cf: RawObjPtr) -> CFHashCode;
    fn CFEqual(cf1: RawObjPtr, cf2: RawObjPtr) -> Boolean;
    fn CFGetTypeID(cf: RawObjPtr) -> CFTypeID;
}

pub trait CFTypeInterface
where
    Self: AsRaw,
    Self: Sized,
{
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
