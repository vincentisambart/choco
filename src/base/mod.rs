use std::marker::PhantomData;
use std::ptr::NonNull;

// pub(crate) mod block;
// pub(crate) mod core_foundation;

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_release(value: RawObjPtr);
    fn objc_retain(value: RawObjPtr) -> Option<RawObjPtr>;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(cf: RawObjPtr);
    fn CFRetain(cf: RawObjPtr) -> Option<RawObjPtr>;
}

#[repr(C)]
pub struct OpaqueObject {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct OpaqueClass {
    _private: [u8; 0],
}

pub enum TypeKind {
    ObjC,
    CF,
}

pub trait Type: Sized {
    const KIND: TypeKind;
}

pub trait ObjCClass: Type {
    fn class() -> RawClassPtr;
}

/// Marker trait used for handling of type parameters in NSArray and NSDictionary.
pub unsafe trait IsKindOf<T: Type>: Type {}
unsafe impl<T: Type> IsKindOf<T> for T {}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RawObjPtr {
    ptr: NonNull<OpaqueObject>,
}

impl RawObjPtr {
    pub unsafe fn retain<T: Type>(&self) -> Ptr<T, Retained> {
        let retained = match T::KIND {
            TypeKind::ObjC => objc_retain(*self),
            TypeKind::CF => CFRetain(*self),
        }
        .unwrap();
        Ptr {
            raw: retained,
            _marker: PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RawClassPtr {
    ptr: NonNull<OpaqueClass>,
}

// TODO: Try to find a better name?
pub trait PtrBehavior: Sized {
    // fn release<T: Type>(ptr: &mut Ptr<T, Self>);
    fn release<T: Type>(raw: RawObjPtr);
}

pub struct Ptr<T, Behavior = Retained>
where
    T: Type,
    Behavior: PtrBehavior,
{
    raw: RawObjPtr,
    _marker: PhantomData<*const (T, Behavior)>,
}

impl<T, Behavior> Ptr<T, Behavior>
where
    T: Type,
    Behavior: PtrBehavior,
{
    pub(crate) unsafe fn from_raw_unchecked(raw: RawObjPtr) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn retain(&self) -> Ptr<T, Retained> {
        unsafe { self.raw.retain() }
    }

    pub fn as_raw(&self) -> RawObjPtr {
        self.raw
    }
}

pub trait AsRaw {
    fn as_raw(&self) -> RawObjPtr;
}

impl<T, Behavior> AsRaw for Ptr<T, Behavior>
where
    T: Type,
    Behavior: PtrBehavior,
{
    fn as_raw(&self) -> RawObjPtr {
        self.as_raw()
    }
}

impl<T, Behavior> Drop for Ptr<T, Behavior>
where
    T: Type,
    Behavior: PtrBehavior,
{
    fn drop(&mut self) {
        Behavior::release::<T>(self.raw)
    }
}

pub struct Retained {}

impl PtrBehavior for Retained {
    fn release<T: Type>(raw: RawObjPtr) {
        unsafe {
            match T::KIND {
                TypeKind::ObjC => objc_release(raw),
                TypeKind::CF => CFRelease(raw),
            }
        }
    }
}

pub struct Static {}

impl PtrBehavior for Static {
    fn release<T: Type>(_raw: RawObjPtr) {}
}

unsafe impl<T: Type> Send for Ptr<T, Static> {}
unsafe impl<T: Type> Sync for Ptr<T, Static> {}

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

/// Computes the FourCC for the string passed.
///
/// Expecting the &str passed to be all ASCII of length 4.
/// No explicit check is done (though a shorter length will end up with a panic).
const fn fourcc_unchecked(text: &str) -> u32 {
    let bytes = text.as_bytes();

    ((bytes[0] & 0x7F) as u32) << 24
        | ((bytes[1] & 0x7F) as u32) << 16
        | ((bytes[2] & 0x7F) as u32) << 8
        | ((bytes[3] & 0x7F) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fourcc() {
        assert_eq!(fourcc_unchecked("soun"), 0x736F756Eu32);
        assert_eq!(fourcc_unchecked("text"), 0x74657874u32);
    }
}
