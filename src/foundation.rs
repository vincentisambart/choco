use crate::base::*;
use choco_macro::NSObjectProtocol;

//-------------------------------------------------------------------
// NSString

extern "C" {
    fn choco_Foundation_NSString_class() -> NullableObjCClassPtr;

    fn choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
        class: ObjCClassPtr,
        bytes: *const std::ffi::c_void,
        len: usize,
        encoding: NSStringEncoding,
    ) -> RawNullableObjCPtr;

    fn choco_Foundation_NSStringInterface_instance_UTF8String(self_: RawObjCPtr) -> *const i8;
    fn choco_Foundation_NSStringInterface_instance_characterAtIndex(
        self_: RawObjCPtr,
        index: NSUInteger,
    ) -> u16;
    fn choco_Foundation_NSStringInterface_instance_length(self_: RawObjCPtr) -> NSUInteger;
    fn choco_Foundation_NSStringInterface_instance_isEqualToString(
        self_: RawObjCPtr,
        other: RawNullableObjCPtr,
    ) -> BOOL;
}

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
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "Foundation")]
pub struct NSString {
    ptr: OwnedObjCPtr,
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

/// Unowned version of NSString used for static strings.
/// The main difference is that it's Copy and doesn't do anything on drop.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StaticNSString {
    raw: RawObjCPtr,
}

impl StaticNSString {
    pub(crate) unsafe fn from_static(raw: RawObjCPtr) -> Self {
        Self { raw }
    }
}

impl NSObjectProtocol for StaticNSString {
    type Owned = NSString;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self::Owned { ptr }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.raw
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_Foundation_NSString_class() }
            .into_opt()
            .expect("expecting +[NSString class] to return a non null pointer")
    }
}

impl NSObjectInterface for StaticNSString {}
impl NSStringInterface for StaticNSString {}

impl IsKindOf<NSObject> for StaticNSString {}
impl IsKindOf<NSString> for StaticNSString {}

impl std::cmp::PartialEq<StaticNSString> for NSString {
    fn eq(&self, other: &StaticNSString) -> bool {
        self.is_equal_to_string(other)
    }
}

impl std::cmp::PartialEq<NSString> for StaticNSString {
    fn eq(&self, other: &NSString) -> bool {
        self.is_equal_to_string(other)
    }
}

//-------------------------------------------------------------------
// NSURL

extern "C" {
    fn choco_Foundation_NSURL_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSURLInterface_class_newWithString(
        class: ObjCClassPtr,
        urlString: RawObjCPtr,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSURLInterface_class_fileURLWithPath(
        class: ObjCClassPtr,
        path: RawObjCPtr,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSURLInterface_class_fileURLWithPath_isDirectory(
        class: ObjCClassPtr,
        path: RawObjCPtr,
        is_directory: BOOL,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSURLInterface_instance_absoluteString(
        self_: RawObjCPtr,
    ) -> RawNullableObjCPtr;
}

pub trait NSURLInterface: NSObjectInterface {
    fn new_with_string(string: &impl NSStringInterface) -> Option<Self::Owned> {
        let raw_ptr = unsafe {
            choco_Foundation_NSURLInterface_class_newWithString(Self::class(), string.as_raw())
        };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { Self::from_owned_raw_unchecked(raw) })
    }

    // If you know if path is a directory or not, use file_url_with_path_is_directory() as it does not require to access the file system.
    // file_url_with_path() checks on the disk if the path is a directory (if it does not exist, it's not considered a directory).
    fn file_url_with_path(path: &impl NSStringInterface) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_Foundation_NSURLInterface_class_fileURLWithPath(Self::class(), path.as_raw())
        };
        // In fact if the path is empty you will get a nil, but the documentation says you should not pass an empty path.
        let raw = raw_ptr.into_opt().expect(
            "expecting -[[<class> alloc] initFileURLWithPath:] to return a non null pointer",
        );
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }

    fn file_url_with_path_is_directory(
        path: &impl NSStringInterface,
        is_directory: bool,
    ) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_Foundation_NSURLInterface_class_fileURLWithPath_isDirectory(
                Self::class(),
                path.as_raw(),
                is_directory.into(),
            )
        };
        // In fact if the path is empty you will get a nil, but the documentation says you should not pass an empty path.
        let raw = raw_ptr.into_opt()
            .expect("expecting -[[<class> alloc] initFileURLWithPath:isDirectory:] to return a non null pointer");
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }

    fn absolute_string(&self) -> NSString {
        let raw_self = self.as_raw();
        let raw_ptr = unsafe { choco_Foundation_NSURLInterface_instance_absoluteString(raw_self) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[NSURL absoluteString] to return a non null pointer");
        unsafe { NSString::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "Foundation")]
pub struct NSURL {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for NSURL {}
impl NSURLInterface for NSURL {}

impl From<NSURL> for NSObject {
    fn from(obj: NSURL) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSObject> for NSURL {}

#[cfg(test)]
mod url_tests {
    use super::*;

    #[test]
    fn simple_url() {
        let invalid_url_string = NSString::new_with_str("🍁");
        let valid_url_string = NSString::new_with_str("https://www.rust-lang.org/");
        assert!(NSURL::new_with_string(&invalid_url_string).is_none());
        let valid_url = NSURL::new_with_string(&valid_url_string).unwrap();
        assert!(valid_url.absolute_string().is_equal(&valid_url_string));
        assert_eq!(
            &valid_url.absolute_string().to_string().unwrap(),
            "https://www.rust-lang.org/"
        );
        assert!(valid_url.is_kind_of(NSObject::class()));
        assert!(valid_url.is_kind_of(NSURL::class()));
        assert!(!valid_url.is_kind_of(NSString::class()));
    }
}

//-------------------------------------------------------------------
// NSArray

extern "C" {
    fn choco_Foundation_NSArray_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSArrayInterface_instance_count(self_: RawObjCPtr) -> usize;
    fn choco_Foundation_NSArrayInterface_instance_firstObject(
        self_: RawObjCPtr,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSArrayInterface_instance_lastObject(
        self_: RawObjCPtr,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSArrayInterface_instance_objectAtIndex(
        self_: RawObjCPtr,
        index: usize,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(
        self_: RawObjCPtr,
        obj: RawObjCPtr,
    ) -> RawNullableObjCPtr;
}

pub trait NSArrayInterface<T: NSObjectProtocol>: NSObjectInterface {
    fn first(&self) -> Option<T::Owned> {
        let raw_self = self.as_raw();
        let raw_ptr = unsafe { choco_Foundation_NSArrayInterface_instance_firstObject(raw_self) };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { T::from_owned_raw_unchecked(raw) })
    }
    fn last(&self) -> Option<T::Owned> {
        let raw_self = self.as_raw();
        let raw_ptr = unsafe { choco_Foundation_NSArrayInterface_instance_lastObject(raw_self) };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { T::from_owned_raw_unchecked(raw) })
    }

    fn object_at(&self, index: usize) -> T::Owned {
        let raw_self = self.as_raw();
        let raw_ptr =
            unsafe { choco_Foundation_NSArrayInterface_instance_objectAtIndex(raw_self, index) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[NSArray objectAtIndex:] to return a non null pointer");

        unsafe { T::from_owned_raw_unchecked(raw) }
    }

    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSArrayInterface_instance_count(raw_self) }
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn adding_object<Object>(&self, object: &Object) -> NSArray<T>
    where
        Object: IsKindOf<T>,
    {
        let raw_self = self.as_raw();
        let raw_obj = object.as_raw();
        let raw_ptr = unsafe {
            choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(raw_self, raw_obj)
        };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting +[NSArray arrayByAddingObject:] to return a non null pointer");
        unsafe { NSArray::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSArray<T: NSObjectProtocol> {
    ptr: OwnedObjCPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: NSObjectProtocol> NSObjectProtocol for NSArray<T> {
    type Owned = Self;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw()
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_Foundation_NSArray_class() }
            .into_opt()
            .expect("expecting +[NSArray class] to return a non null pointer")
    }
}

impl<T: NSObjectProtocol> NSObjectInterface for NSArray<T> {}
impl<T: NSObjectProtocol> NSArrayInterface<T> for NSArray<T> {}

impl<T: NSObjectProtocol> From<NSArray<T>> for NSObject {
    fn from(obj: NSArray<T>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn empty_arrays() {
        let array1: NSArray<NSObject> = NSArray::new();
        let array2: NSArray<NSObject> = NSArray::new();
        assert!(array1.is_equal(&array1));
        assert!(array1.is_equal(&array2));
        assert_eq!(array1.count(), 0);
        assert_eq!(array2.count(), 0);
        assert!(array1.first().is_none());
        assert!(array1.last().is_none());
    }

    #[test]
    fn adding() {
        let array1: NSArray<NSObject> = NSArray::new();
        let array2: NSArray<NSObject> = NSArray::new();
        let obj1 = NSObject::new();
        let str1 = NSString::new();
        let array1 = array1.adding_object(&obj1);
        let array2 = array2.adding_object(&obj1);
        assert_eq!(array1.count(), 1);
        assert!(array1.is_equal(&array2));
        assert!(array1.object_at(0).is_equal(&obj1));
        assert!(array1.first().unwrap().is_equal(&obj1));

        // We should be able to add any type inheriting from NSObject into array1
        let array1 = array1.adding_object(&str1);
        assert_eq!(array1.count(), 2);
    }
}

//-------------------------------------------------------------------
// NSDictionary

extern "C" {
    fn choco_Foundation_NSDictionary_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSDictionaryInterface_instance_count(self_: RawObjCPtr) -> usize;
    fn choco_Foundation_NSDictionaryInterface_instance_objectForKey(
        self_: RawObjCPtr,
        key: RawObjCPtr,
    ) -> RawNullableObjCPtr;
}

pub trait NSDictionaryInterface<K: NSObjectProtocol, V: NSObjectProtocol>:
    NSObjectInterface
{
    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDictionaryInterface_instance_count(raw_self) }
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn get<Key>(&self, key: &Key) -> Option<V::Owned>
    where
        Key: IsKindOf<K>,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        let raw_ptr = unsafe {
            choco_Foundation_NSDictionaryInterface_instance_objectForKey(raw_self, raw_key)
        };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { V::from_owned_raw_unchecked(raw) })
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "Foundation")]
pub struct NSDictionary<K: NSObjectProtocol, V: NSObjectProtocol> {
    ptr: OwnedObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K: NSObjectProtocol, V: NSObjectProtocol> NSObjectInterface for NSDictionary<K, V> {}
impl<K: NSObjectProtocol, V: NSObjectProtocol> NSDictionaryInterface<K, V> for NSDictionary<K, V> {}

impl<K: NSObjectProtocol, V: NSObjectProtocol> From<NSDictionary<K, V>> for NSObject {
    fn from(obj: NSDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

//-------------------------------------------------------------------
// NSMutableDictionary

extern "C" {
    fn choco_Foundation_NSMutableDictionary_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSMutableDictionary_instance_setObject_forKey(
        self_: RawObjCPtr,
        object: RawObjCPtr,
        key: RawObjCPtr,
    );
    fn choco_Foundation_NSMutableDictionary_instance_removeObjectForKey(
        self_: RawObjCPtr,
        key: RawObjCPtr,
    );
    fn choco_Foundation_NSMutableDictionary_instance_removeAllObjects(self_: RawObjCPtr);
}

pub trait NSMutableDictionaryInterface<K: NSObjectProtocol, V: NSObjectProtocol>:
    NSDictionaryInterface<K, V>
{
    fn set<Key, Value>(&self, key: &Key, value: &Value)
    where
        Key: IsKindOf<K>,
        Value: IsKindOf<V>,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        let raw_value = value.as_raw();
        unsafe {
            choco_Foundation_NSMutableDictionary_instance_setObject_forKey(
                raw_self, raw_value, raw_key,
            )
        }
    }

    fn remove<Key>(&self, key: &Key)
    where
        Key: IsKindOf<K>,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        unsafe {
            choco_Foundation_NSMutableDictionary_instance_removeObjectForKey(raw_self, raw_key)
        }
    }

    fn remove_all(&self) {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSMutableDictionary_instance_removeAllObjects(raw_self) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "Foundation")]
pub struct NSMutableDictionary<K: NSObjectProtocol, V: NSObjectProtocol> {
    ptr: OwnedObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K: NSObjectProtocol, V: NSObjectProtocol> NSObjectInterface for NSMutableDictionary<K, V> {}
impl<K: NSObjectProtocol, V: NSObjectProtocol> NSDictionaryInterface<K, V>
    for NSMutableDictionary<K, V>
{
}
impl<K: NSObjectProtocol, V: NSObjectProtocol> NSMutableDictionaryInterface<K, V>
    for NSMutableDictionary<K, V>
{
}

impl<K: NSObjectProtocol, V: NSObjectProtocol> From<NSMutableDictionary<K, V>> for NSObject {
    fn from(obj: NSMutableDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<K: NSObjectProtocol, V: NSObjectProtocol> From<NSMutableDictionary<K, V>>
    for NSDictionary<K, V>
{
    fn from(obj: NSMutableDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

#[cfg(test)]
mod dictionary_tests {
    use super::*;

    #[test]
    fn simple_dictionary() {
        let dic: NSMutableDictionary<NSString, NSDate> = NSMutableDictionary::new();
        assert!(dic.is_empty());
        assert_eq!(dic.count(), 0);
        let date = NSDate::new();
        let key = NSString::new_with_str("abcd");
        dic.set(&key, &date);
        assert_eq!(dic.count(), 1);
        let got = dic.get(&key).unwrap();
        assert!(got.is_equal(&date));
    }
}

//-------------------------------------------------------------------
// NSDate

#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NSTimeInterval {
    secs: f64,
}

impl NSTimeInterval {
    pub fn from_secs(secs: f64) -> Self {
        Self { secs }
    }

    pub fn secs(self) -> f64 {
        self.secs
    }
}

extern "C" {
    fn choco_Foundation_NSDate_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSDate_instance_timeIntervalSinceNow(self_: RawObjCPtr) -> NSTimeInterval;
    fn choco_Foundation_NSDate_instance_timeIntervalSinceReferenceDate(
        self_: RawObjCPtr,
    ) -> NSTimeInterval;
    fn choco_Foundation_NSDate_instance_timeIntervalSince1970(self_: RawObjCPtr) -> NSTimeInterval;
    fn choco_Foundation_NSDate_instance_timeIntervalSinceDate(
        self_: RawObjCPtr,
        anotherDate: RawObjCPtr,
    ) -> NSTimeInterval;
}

pub trait NSDateInterface: NSObjectInterface {
    fn since_now(&self) -> NSTimeInterval {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDate_instance_timeIntervalSinceNow(raw_self) }
    }

    fn since_reference_date(&self) -> NSTimeInterval {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDate_instance_timeIntervalSinceReferenceDate(raw_self) }
    }

    fn since_1970(&self) -> NSTimeInterval {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDate_instance_timeIntervalSince1970(raw_self) }
    }

    fn since(&self, another_date: &NSDate) -> NSTimeInterval {
        let raw_self = self.as_raw();
        let raw_another_date = another_date.as_raw();
        unsafe {
            choco_Foundation_NSDate_instance_timeIntervalSinceDate(raw_self, raw_another_date)
        }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "Foundation")]
pub struct NSDate {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for NSDate {}
impl NSDateInterface for NSDate {}

impl From<NSDate> for NSObject {
    fn from(obj: NSDate) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSDate> for NSString {}

impl std::ops::Sub for &NSDate {
    type Output = NSTimeInterval;

    fn sub(self, rhs: Self) -> Self::Output {
        self.since(rhs)
    }
}

//-------------------------------------------------------------------
// NSNumber

extern "C" {
    fn choco_Foundation_NSNumber_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSNumberInterface_class_newWithBool(
        class: ObjCClassPtr,
        value: BOOL,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSNumberInterface_class_newWithInteger(
        class: ObjCClassPtr,
        value: NSInteger,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSNumberInterface_class_newWithUnsignedInteger(
        class: ObjCClassPtr,
        value: NSUInteger,
    ) -> RawNullableObjCPtr;
    fn choco_Foundation_NSNumberInterface_instance_boolValue(self_: RawObjCPtr) -> BOOL;
    fn choco_Foundation_NSNumberInterface_instance_integerValue(self_: RawObjCPtr) -> NSInteger;
    fn choco_Foundation_NSNumberInterface_instance_unsignedIntegerValue(
        self_: RawObjCPtr,
    ) -> NSUInteger;
}

pub trait NSNumberInterface: NSObjectInterface {
    fn from_bool(value: bool) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_Foundation_NSNumberInterface_class_newWithBool(Self::class(), value.into())
        };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[[<class> alloc] initWithBool:] to return a non null pointer");
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }

    fn from_isize(value: isize) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_Foundation_NSNumberInterface_class_newWithInteger(Self::class(), value)
        };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[[<class> alloc] initWithInteger:] to return a non null pointer");
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }

    fn from_usize(value: usize) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_Foundation_NSNumberInterface_class_newWithUnsignedInteger(Self::class(), value)
        };
        let raw = raw_ptr.into_opt().expect(
            "expecting -[[<class> alloc] initWithUnsignedInteger:] to return a non null pointer",
        );
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }

    fn as_bool(&self) -> bool {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSNumberInterface_instance_boolValue(raw_self) }.into()
    }

    fn as_isize(&self) -> isize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSNumberInterface_instance_integerValue(raw_self) }
    }

    fn as_usize(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSNumberInterface_instance_unsignedIntegerValue(raw_self) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "Foundation")]
pub struct NSNumber {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for NSNumber {}
impl NSNumberInterface for NSNumber {}

impl From<NSNumber> for NSObject {
    fn from(obj: NSNumber) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSObject> for NSNumber {}

#[cfg(test)]
mod number_tests {
    use super::*;

    #[test]
    fn bool_value() {
        let t1 = NSNumber::from_bool(true);
        let t2 = NSNumber::from_bool(true);
        let f = NSNumber::from_bool(false);
        assert!(t1.is_kind_of(NSNumber::class()));
        assert!(t2.is_kind_of(NSNumber::class()));
        assert!(t1.is_equal(&t1));
        assert!(t1.is_equal(&t2));
        assert!(!t1.is_equal(&f));
        assert_eq!(t1.as_bool(), true);
        assert_eq!(t2.as_bool(), true);
        assert_eq!(f.as_bool(), false);
    }

    #[test]
    fn isize_value() {
        let t = NSNumber::from_bool(true);
        let f = NSNumber::from_bool(false);
        let i = NSNumber::from_isize(12345657890);
        assert!(t.is_kind_of(NSNumber::class()));
        assert!(f.is_kind_of(NSNumber::class()));
        assert!(i.is_kind_of(NSNumber::class()));
        assert_eq!(t.as_isize(), 1);
        assert_eq!(f.as_isize(), 0);
        assert_eq!(i.as_isize(), 12345657890);
    }
}

//-------------------------------------------------------------------
// NSError

extern "C" {
    fn choco_Foundation_NSError_class() -> NullableObjCClassPtr;
}

pub trait NSErrorInterface: NSObjectInterface {}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "Foundation")]
pub struct NSError {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for NSError {}
impl NSErrorInterface for NSError {}

impl From<NSError> for NSObject {
    fn from(obj: NSError) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSObject> for NSError {}

/// # Safety
/// - if non null, raw_ptr must be owned and of type T.
/// - if non null, raw_unowned_error must not be owned (probably autoreleased) and point to a NSError.
pub(crate) unsafe fn make_result_unchecked<T: NSObjectInterface>(
    raw_ptr: RawNullableObjCPtr,
    raw_unowned_error: RawNullableObjCPtr,
) -> Result<T::Owned, NSError> {
    // Create the object before checking the error,
    // because if both the new object and error are not null,
    // we want to the object to be properly released.
    let obj = raw_ptr
        .into_opt()
        .map(|raw_ptr| T::from_owned_raw_unchecked(raw_ptr));
    match raw_unowned_error.into_opt() {
        None => Ok(obj.expect("expecting non null return value pointer when no error")),
        Some(raw_error) => Err(NSError::retain_unowned_raw_unchecked(raw_error)),
    }
}
