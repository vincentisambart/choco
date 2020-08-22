use crate::base::{objc, ptr};
use objc::{NSObjectInterface, NSObjectInterfaceClassMethods, NSObjectProtocol, NSUInteger, BOOL};
use ptr::ObjCClass;
use objc::IsKindOf;

//-------------------------------------------------------------------
// NSString interface

extern "C" {
    fn choco_Foundation_NSString_class() -> ptr::ClassPtr;

    fn choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
        class: ptr::ClassPtr,
        bytes: *const std::ffi::c_void,
        len: usize,
        encoding: NSStringEncoding,
    ) -> Option<ptr::RawPtr>;

    fn choco_Foundation_NSStringInterface_instance_UTF8String(self_: ptr::RawPtr) -> *const i8;
    fn choco_Foundation_NSStringInterface_instance_characterAtIndex(
        self_: ptr::RawPtr,
        index: NSUInteger,
    ) -> u16;
    fn choco_Foundation_NSStringInterface_instance_length(self_: ptr::RawPtr) -> NSUInteger;
    fn choco_Foundation_NSStringInterface_instance_isEqualToString(
        self_: ptr::RawPtr,
        other: ptr::RawPtr,
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

    fn is_equal_to_string(&self, obj: &impl NSStringInterface) -> bool {
        let self_raw = self.as_raw();
        let obj_raw = obj.as_raw();
        let ret = unsafe {
            choco_Foundation_NSStringInterface_instance_isEqualToString(self_raw, obj_raw)
        };
        ret.into()
    }
}

pub trait NSStringInterfaceClassMethods: NSObjectInterfaceClassMethods {
    fn new_with_str(text: &str) -> ptr::OwnedPtr<Self> {
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
            ptr::OwnedPtr::from_owned_raw_unchecked(raw)
        }
    }
}

//-------------------------------------------------------------------
// NSString

pub struct NSString {}

impl NSString {}

impl ptr::Type for NSString {
    const KIND: ptr::TypeKind = ptr::TypeKind::ObjC;
}

impl ObjCClass for NSString {
    fn class() -> ptr::ClassPtr {
        unsafe { choco_Foundation_NSString_class() }
    }
}
impl NSObjectInterfaceClassMethods for NSString {}
impl NSStringInterfaceClassMethods for NSString {}

impl NSObjectProtocol for ptr::OwnedPtr<NSString> {
    type Class = NSString;
}
impl NSObjectInterface for ptr::OwnedPtr<NSString> {}
impl NSStringInterface for ptr::OwnedPtr<NSString> {}

unsafe impl IsKindOf<objc::NSObject> for NSString {}


// impl NSCopyingProtocol for NSString {
//     type Immutable = ImmutableNSString;
// }

// impl NSMutableCopyingProtocol for NSString {
//     type Mutable = NSMutableString;
// }

#[cfg(test)]
mod string_tests {
    use super::*;

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
        assert!(obj.is_kind_of(objc::NSObject::class()));
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
//     fn as_raw_ptr(&self) -> ptr::RawPtr {
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

//     fn class() -> ptr::ClassPtr {
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

// //-------------------------------------------------------------------
// // StaticNSString

// /// Unowned version of NSString used for static strings.
// /// The main difference is that it's Copy and doesn't do any reference counting.
// pub struct StaticNSString {
//     ptr: ptr::StaticPtr,
// }

// impl StaticNSString {
//     /// # Safety
//     /// The raw pointer passed in must be a pointer to a static NSString.
//     pub(crate) unsafe fn from_static_unchecked(ptr: ptr::StaticPtr) -> Self {
//         Self { ptr }
//     }
// }

// impl ptr::AsRaw for StaticNSString {
//     fn as_raw_ptr(&self) -> ptr::RawPtr {
//         self.ptr.as_raw_ptr()
//     }
// }

// // TODO: Should not be able to do StaticNSString::new()
// impl NSObjectProtocol for StaticNSString {
//     type Owned = NSString;

//     fn class() -> ptr::ClassPtr {
//         unsafe { choco_Foundation_NSString_class() }
//     }
// }

// impl NSObjectInterface for StaticNSString {}
// impl NSStringInterface for StaticNSString {}
// impl NSCopyingProtocol for StaticNSString {
//     type Immutable = ImmutableNSString;
// }
// unsafe impl IsKindOf<NSObject> for StaticNSString {}
// unsafe impl IsKindOf<NSString> for StaticNSString {}

// impl<Rhs: NSStringInterface> std::cmp::PartialEq<Rhs> for StaticNSString {
//     fn eq(&self, other: &Rhs) -> bool {
//         self.is_equal_to_string(other)
//     }
// }

// impl NSMutableCopyingProtocol for StaticNSString {
//     type Mutable = NSMutableString;
// }

// // TODO: BorrowedNSString

// //-------------------------------------------------------------------
// // NSMutableString interface

// extern "C" {
//     fn choco_Foundation_NSMutableString_class() -> ptr::ClassPtr;
// }

// pub trait NSMutableStringInterface: NSObjectInterface {}

// //-------------------------------------------------------------------
// // NSMutableString

// pub struct NSMutableString {
//     ptr: ptr::OwnedPtr,
// }

// impl ptr::AsRaw for NSMutableString {
//     fn as_raw_ptr(&self) -> ptr::RawPtr {
//         self.ptr.as_raw_ptr()
//     }
// }

// impl ptr::FromOwned for NSMutableString {
//     unsafe fn from_owned_ptr_unchecked(ptr: ptr::OwnedPtr) -> Self {
//         Self { ptr }
//     }
// }

// impl NSObjectProtocol for NSMutableString {
//     type Owned = Self;

//     fn class() -> ptr::ClassPtr {
//         unsafe { choco_Foundation_NSMutableString_class() }
//     }
// }

// impl NSObjectInterface for NSMutableString {}
// impl NSStringInterface for NSMutableString {}
// impl NSMutableStringInterface for NSMutableString {}
// impl ValidObjCGeneric for NSMutableString {}
// unsafe impl IsKindOf<NSObject> for NSMutableString {}
// unsafe impl IsKindOf<NSString> for NSMutableString {}

// impl<Rhs: NSStringInterface> std::cmp::PartialEq<Rhs> for NSMutableString {
//     fn eq(&self, other: &Rhs) -> bool {
//         self.is_equal_to_string(other)
//     }
// }

// impl NSCopyingProtocol for NSMutableString {
//     type Immutable = ImmutableNSString;
// }

// impl NSMutableCopyingProtocol for NSMutableString {
//     type Mutable = NSMutableString;
// }
