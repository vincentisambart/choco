use super::*;
use choco_macro::NSObjectProtocol;

//-------------------------------------------------------------------
// NSString interface

extern "C" {
    fn choco_Foundation_NSString_class() -> NullableObjCClassPtr;

    fn choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
        class: ObjCClassPtr,
        bytes: *const std::ffi::c_void,
        len: usize,
        encoding: NSStringEncoding,
    ) -> NullableRawObjCPtr;

    fn choco_Foundation_NSStringInterface_instance_UTF8String(self_: RawObjCPtr) -> *const i8;
    fn choco_Foundation_NSStringInterface_instance_characterAtIndex(
        self_: RawObjCPtr,
        index: NSUInteger,
    ) -> u16;
    fn choco_Foundation_NSStringInterface_instance_length(self_: RawObjCPtr) -> NSUInteger;
    fn choco_Foundation_NSStringInterface_instance_isEqualToString(
        self_: RawObjCPtr,
        other: NullableRawObjCPtr,
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

pub trait NSStringInterface: NSObjectInterface {
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
            choco_Foundation_NSStringInterface_instance_isEqualToString(self_raw, obj_raw.into())
        };
        ret.into()
    }

    fn new_with_str(text: &str) -> Self::Owned {
        let bytes = text.as_ptr() as *const std::ffi::c_void;
        let len = text.len();
        let encoding = NSStringEncoding::UTF8;
        let raw_ptr = unsafe {
            choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
                Self::class(),
                bytes,
                len,
                encoding,
            )
        };
        let raw = raw_ptr.into_opt().expect(
            "expecting -[[<class> alloc] initWithBytes:length:encoding:] to return a non null pointer",
        );
        unsafe { Self::Owned::from_owned_raw_unchecked(raw) }
    }
}

//-------------------------------------------------------------------
// NSString

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation)]
pub struct NSString {
    ptr: OwnedObjCPtr,
}

impl NSString {
    // NSString instances we created directly are known to be immutable.
    fn new_with_str(text: &str) -> ImmutableNSString {
        let new = <Self as NSStringInterface>::new_with_str(text);
        ImmutableNSString { ptr: new.ptr }
    }
}

impl NSObjectInterface for NSString {}
impl NSStringInterface for NSString {}

impl From<NSString> for NSObject {
    fn from(obj: NSString) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSObject> for NSString {}

impl std::cmp::PartialEq for NSString {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<NSString> for ImmutableNSString {
    fn eq(&self, other: &NSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<NSString> for StaticNSString {
    fn eq(&self, other: &NSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<NSString> for NSMutableString {
    fn eq(&self, other: &NSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl NSCopyingProtocol for NSString {
    type Immutable = ImmutableNSString;
}

impl NSMutableCopyingProtocol for NSString {
    type Mutable = NSMutableString;
}

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
        assert!(obj.is_kind_of(NSObject::class()));
        assert!(obj.is_kind_of(NSString::class()));
        assert_eq!(obj.len(), 2); // NSString's "length" the number of UTF-18 code units.
        assert_eq!(&obj.to_string().unwrap(), text);
    }
}

//-------------------------------------------------------------------
// ImmutableNSString

/// Version of NSString we are statically sure to be immutable.
#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation, objc_class = NSString)]
pub struct ImmutableNSString {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for ImmutableNSString {}
impl NSStringInterface for ImmutableNSString {}

impl From<ImmutableNSString> for NSObject {
    fn from(obj: ImmutableNSString) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl From<ImmutableNSString> for NSString {
    fn from(obj: ImmutableNSString) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for ImmutableNSString {}
impl IsKindOf<NSString> for ImmutableNSString {}

impl std::cmp::PartialEq for ImmutableNSString {
    fn eq(&self, other: &ImmutableNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<ImmutableNSString> for NSString {
    fn eq(&self, other: &ImmutableNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<ImmutableNSString> for StaticNSString {
    fn eq(&self, other: &ImmutableNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<ImmutableNSString> for NSMutableString {
    fn eq(&self, other: &ImmutableNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl NSMutableCopyingProtocol for ImmutableNSString {
    type Mutable = NSMutableString;
}

// An ImmutableNSString is known to be immutable so can be shared between threads.
unsafe impl Send for ImmutableNSString {}
unsafe impl Sync for ImmutableNSString {}

//-------------------------------------------------------------------
// StaticNSString

/// Unowned version of NSString used for static strings.
/// The main difference is that it's Copy and doesn't do any reference counting.
#[repr(transparent)]
#[derive(Copy, Clone, NSObjectProtocol)]
#[choco(framework = Foundation, owned = NSString)]
pub struct StaticNSString {
    raw: RawObjCPtr,
}

impl StaticNSString {
    /// # Safety
    /// The raw pointer passed in must be a pointer to a static NSString.
    pub(crate) unsafe fn from_static(raw: RawObjCPtr) -> Self {
        Self { raw }
    }
}

impl NSObjectInterface for StaticNSString {}
impl NSStringInterface for StaticNSString {}

impl IsKindOf<NSObject> for StaticNSString {}
impl IsKindOf<NSString> for StaticNSString {}

impl std::cmp::PartialEq for StaticNSString {
    fn eq(&self, other: &StaticNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<StaticNSString> for NSString {
    fn eq(&self, other: &StaticNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<StaticNSString> for ImmutableNSString {
    fn eq(&self, other: &StaticNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<StaticNSString> for NSMutableString {
    fn eq(&self, other: &StaticNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl NSMutableCopyingProtocol for StaticNSString {
    type Mutable = NSMutableString;
}

// A StaticNSString is known to be immutable so can be shared between threads.
unsafe impl Send for StaticNSString {}
unsafe impl Sync for StaticNSString {}

//-------------------------------------------------------------------
// NSMutableString interface

extern "C" {
    fn choco_Foundation_NSMutableString_class() -> NullableObjCClassPtr;
}

pub trait NSMutableStringInterface: NSObjectInterface {}

//-------------------------------------------------------------------
// NSMutableString

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation)]
pub struct NSMutableString {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for NSMutableString {}
impl NSStringInterface for NSMutableString {}
impl NSMutableStringInterface for NSMutableString {}

impl From<NSMutableString> for NSObject {
    fn from(obj: NSMutableString) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSObject> for NSMutableString {}
impl IsKindOf<NSString> for NSMutableString {}

impl std::cmp::PartialEq for NSMutableString {
    fn eq(&self, other: &NSMutableString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<NSMutableString> for NSString {
    fn eq(&self, other: &NSMutableString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<NSMutableString> for StaticNSString {
    fn eq(&self, other: &NSMutableString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<NSMutableString> for ImmutableNSString {
    fn eq(&self, other: &NSMutableString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl NSCopyingProtocol for NSMutableString {
    type Immutable = ImmutableNSString;
}

impl NSMutableCopyingProtocol for NSMutableString {
    type Mutable = ImmutableNSString;
}
