use super::{NSObject, NSObjectInterface, NSObjectProtocol};
use crate::base::{
    AsRaw, IsKindOf, NSUInteger, ObjCClass, Ownership, Ptr, RawClassPtr, RawObjPtr, Retained, Type,
    TypeKind, BOOL,
};

//-------------------------------------------------------------------
// NSString

extern "C" {
    fn choco_Foundation_NSString_class() -> RawClassPtr;

    fn choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
        class: RawClassPtr,
        bytes: *const std::ffi::c_void,
        len: usize,
        encoding: NSStringEncoding,
    ) -> Option<RawObjPtr>;

    fn choco_Foundation_NSStringInterface_instance_UTF8String(self_: RawObjPtr) -> *const i8;
    fn choco_Foundation_NSStringInterface_instance_characterAtIndex(
        self_: RawObjPtr,
        index: NSUInteger,
    ) -> u16;
    fn choco_Foundation_NSStringInterface_instance_length(self_: RawObjPtr) -> NSUInteger;
    fn choco_Foundation_NSStringInterface_instance_isEqualToString(
        self_: RawObjPtr,
        other: RawObjPtr,
    ) -> BOOL;
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct NSStringEncoding(usize);

impl NSStringEncoding {
    pub const ASCII: Self = Self(1);
    pub const UTF8: Self = Self(4);
    pub const UTF16: Self = Self(10);
    pub const UTF16_BE: Self = Self(0x90000100);
    pub const UTF16_LE: Self = Self(0x94000100);
    pub const UTF32: Self = Self(0x8c000100);
    pub const UTF32_BE: Self = Self(0x98000100);
    pub const UTF32_LE: Self = Self(0x9c000100);
    pub const NEXTSTEP: Self = Self(2);
    pub const JAPANESE_EUC: Self = Self(3);
    pub const ISO_LATIN1: Self = Self(5);
    pub const SYMBOL: Self = Self(6);
    pub const NON_LOSSY_ASCII: Self = Self(7);
    pub const SHIFT_JIS: Self = Self(8);
    pub const ISO_LATIN2: Self = Self(9);
    pub const WINDOWS_CP1251: Self = Self(11);
    pub const WINDOWS_CP1252: Self = Self(12);
    pub const WINDOWS_CP1253: Self = Self(13);
    pub const WINDOWS_CP1254: Self = Self(14);
    pub const WINDOWS_CP1250: Self = Self(15);
    pub const ISO2022JP: Self = Self(21);
    pub const MACOS_ROMAN: Self = Self(30);
}

pub trait NSStringInterface: NSObjectInterface
// where
//     Self: NSCopyingProtocol + NSMutableCopyingProtocol,
{
    fn new_with_str(text: &str) -> Ptr<Self, Retained> {
        let bytes = text.as_ptr() as *const std::ffi::c_void;
        let len = text.len();
        let encoding = NSStringEncoding::UTF8;
        unsafe {
            let raw = choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
                Self::class(),
                bytes,
                len,
                encoding,
            )
            .unwrap();
            Ptr::from_raw_unchecked(raw)
        }
    }
}

pub trait NSStringInterfaceInstanceMethods: AsRaw {
    fn to_string(&self) -> Result<String, std::str::Utf8Error> {
        let raw_self = self.as_raw();
        let cstr = unsafe {
            let bytes = choco_Foundation_NSStringInterface_instance_UTF8String(raw_self);
            std::ffi::CStr::from_ptr(bytes)
        };
        Ok(cstr.to_str()?.to_string())
    }

    fn to_string_lossy(&self) -> String {
        let raw_self = self.as_raw();
        let cstr = unsafe {
            let bytes = choco_Foundation_NSStringInterface_instance_UTF8String(raw_self);
            std::ffi::CStr::from_ptr(bytes)
        };
        cstr.to_string_lossy().to_string()
    }

    fn char_at(&self, index: usize) -> u16 {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSStringInterface_instance_characterAtIndex(raw_self, index) }
    }

    fn len(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSStringInterface_instance_length(raw_self) }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_equal_to_string<OtherT, OtherOwnership>(&self, obj: &Ptr<OtherT, OtherOwnership>) -> bool
    where
        OtherT: NSStringInterface,
        OtherOwnership: Ownership,
    {
        let self_raw = self.as_raw();
        let obj_raw = obj.as_raw();
        let ret = unsafe {
            choco_Foundation_NSStringInterface_instance_isEqualToString(self_raw, obj_raw)
        };
        ret.into()
    }
}

impl<T, O> NSStringInterfaceInstanceMethods for Ptr<T, O>
where
    T: NSStringInterface,
    O: Ownership,
{
}

pub struct NSString {}

impl Type for NSString {
    const KIND: TypeKind = TypeKind::ObjC;
}

unsafe impl IsKindOf<NSObject> for NSString {}

impl ObjCClass for NSString {
    fn class() -> RawClassPtr {
        unsafe { choco_Foundation_NSString_class() }
    }
}

impl NSObjectProtocol for NSString {}
impl NSObjectInterface for NSString {}
impl NSStringInterface for NSString {}

// impl NSCopyingProtocol for NSString {
//     type Immutable = ImmutableNSString;
// }

// impl NSMutableCopyingProtocol for NSString {
//     type Mutable = NSMutableString;
// }

#[cfg(test)]
mod string_tests {
    use super::*;
    use crate::foundation::NSObject;
    use crate::foundation::NSObjectProtocolInstanceMethods as _;

    #[test]
    fn empty_strings() {
        let string1 = NSString::new();
        let string2 = NSString::new();
        assert!(string1.is_kind_of(NSString::class()));
        assert!(string2.is_kind_of(NSString::class()));
        assert!(string1.is_equal(&string1));
        assert!(string1.is_equal(&string2));
        assert_eq!(string1.len(), 0);
        assert_eq!(string2.len(), 0);
        assert_eq!(&string1.to_string().unwrap(), "");
    }

    #[test]
    fn new_with_str() {
        let text = "😁";
        let obj = NSString::new_with_str(text);
        assert!(obj.is_kind_of(NSObject::class()));
        assert!(obj.is_kind_of(NSString::class()));
        assert_eq!(obj.len(), 2); // NSString's "length" the number of UTF-18 code units.
        assert_eq!(&obj.to_string().unwrap(), text);
    }
}

// //-------------------------------------------------------------------
// // ImmutableNSString

// /// Version of NSString we are statically sure to be immutable.
// pub struct ImmutableNSString {
//     ptr: ptr::OwnedPtr,
// }

// impl ptr::AsRaw for ImmutableNSString {
//     fn as_raw_ptr(&self) -> RawObjPtr {
//         self.ptr.as_raw_ptr()
//     }
// }

// impl ptr::FromOwned for ImmutableNSString {
//     unsafe fn from_owned_ptr_unchecked(ptr: ptr::OwnedPtr) -> Self {
//         Self { ptr }
//     }
// }

// impl NSObjectProtocol for ImmutableNSString {
//     type Owned = NSString;

//     fn class() -> RawClassPtr {
//         unsafe { choco_Foundation_NSString_class() }
//     }
// }

// impl NSObjectInterface for ImmutableNSString {}
// impl NSStringInterface for ImmutableNSString {}
// impl NSCopyingProtocol for ImmutableNSString {
//     type Immutable = Self;
// }
// unsafe impl IsKindOf<NSObject> for ImmutableNSString {}
// unsafe impl IsKindOf<NSString> for ImmutableNSString {}

// impl<Rhs: NSStringInterface> std::cmp::PartialEq<Rhs> for ImmutableNSString {
//     fn eq(&self, other: &Rhs) -> bool {
//         self.is_equal_to_string(other)
//     }
// }

// impl NSMutableCopyingProtocol for ImmutableNSString {
//     type Mutable = NSMutableString;
// }

// // An ImmutableNSString is known to be immutable so can be shared between threads.
// unsafe impl Send for ImmutableNSString {}
// unsafe impl Sync for ImmutableNSString {}

//-------------------------------------------------------------------
// NSMutableString

extern "C" {
    fn choco_Foundation_NSMutableString_class() -> RawClassPtr;
}

pub trait NSMutableStringInterface: NSStringInterface {}

pub struct NSMutableString {}

impl Type for NSMutableString {
    const KIND: TypeKind = TypeKind::ObjC;
}

unsafe impl IsKindOf<NSObject> for NSMutableString {}
unsafe impl IsKindOf<NSString> for NSMutableString {}

impl ObjCClass for NSMutableString {
    fn class() -> RawClassPtr {
        unsafe { choco_Foundation_NSString_class() }
    }
}

impl NSObjectProtocol for NSMutableString {}
impl NSObjectInterface for NSMutableString {}
impl NSStringInterface for NSMutableString {}
impl NSMutableStringInterface for NSMutableString {}

// impl NSCopyingProtocol for NSMutableString {
//     type Immutable = ImmutableNSString;
// }

// impl NSMutableCopyingProtocol for NSMutableString {
//     type Mutable = NSMutableString;
// }
