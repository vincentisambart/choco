use crate::core::*;

//-------------------------------------------------------------------
// NSString

extern "C" {
    fn choco_Foundation_NSString_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSStringInterface_instance_UTF8String(self_: RawObjCPtr) -> *const i8;
    fn choco_Foundation_NSStringInterface_instance_characterAtIndex(
        self_: RawObjCPtr,
        index: NSUInteger,
    ) -> u16;
    fn choco_Foundation_NSStringInterface_instance_length(self_: RawObjCPtr) -> NSUInteger;
    fn choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
        class: ObjCClassPtr,
        bytes: *const std::ffi::c_void,
        len: usize,
        encoding: NSStringEncoding,
    ) -> RawNullableObjCPtr;
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

    fn new_with_str(text: &str) -> Self::Owned {
        let bytes = text.as_ptr() as *const std::ffi::c_void;
        let len = text.len();
        let encoding = NSStringEncoding::UTF8;
        let raw = unsafe {
            choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(
                Self::class(),
                bytes,
                len,
                encoding,
            )
        }
        .into_opt()
        .expect(
            "expecting -[NSString initWithBytes:length:encoding:] to return a non null pointer",
        );
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSString {
    ptr: OwnedObjCPtr,
}

impl ObjCPtr for NSString {
    type Owned = Self;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self { ptr }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw()
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_Foundation_NSString_class() }
            .into_opt()
            .expect("expecting +[NSString class] to return a non null pointer")
    }
}

impl NSObjectProtocol for NSString {}
impl NSObjectInterface for NSString {}
impl NSStringInterface for NSString {}

impl From<NSString> for NSObject {
    fn from(obj: NSString) -> Self {
        unsafe { NSObject::from_owned_unchecked(obj.ptr) }
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
        unsafe {
            choco_Foundation_NSURLInterface_class_newWithString(Self::class(), string.as_raw())
        }
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
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[[NSURL alloc] initFileURLWithPath:] to return a non null pointer");
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
            .expect("expecting -[[NSURL alloc] initFileURLWithPath:isDirectory:] to return a non null pointer");
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
#[derive(Clone)]
pub struct NSURL {
    ptr: OwnedObjCPtr,
}

impl ObjCPtr for NSURL {
    type Owned = Self;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self { ptr }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw()
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_Foundation_NSURL_class() }
            .into_opt()
            .expect("expecting +[NSURL class] to return a non null pointer")
    }
}

impl NSObjectProtocol for NSURL {}
impl NSObjectInterface for NSURL {}
impl NSURLInterface for NSURL {}

impl From<NSURL> for NSObject {
    fn from(obj: NSURL) -> Self {
        unsafe { NSObject::from_owned_unchecked(obj.ptr) }
    }
}

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

pub trait NSArrayInterface<T: ObjCPtr>: NSObjectInterface {
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
        Object: ObjCPtr + Into<T>,
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
pub struct NSArray<T: ObjCPtr> {
    ptr: OwnedObjCPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ObjCPtr> ObjCPtr for NSArray<T> {
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

impl<T: ObjCPtr> NSObjectProtocol for NSArray<T> {}
impl<T: ObjCPtr> NSObjectInterface for NSArray<T> {}
impl<T: ObjCPtr> NSArrayInterface<T> for NSArray<T> {}

impl<T: ObjCPtr> From<NSArray<T>> for NSObject {
    fn from(obj: NSArray<T>) -> Self {
        unsafe { NSObject::from_owned_unchecked(obj.ptr) }
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

pub trait NSDictionaryInterface<K: ObjCPtr, V: ObjCPtr>: NSObjectInterface {
    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDictionaryInterface_instance_count(raw_self) }
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn object_for<Key>(&self, key: &Key) -> Option<V::Owned>
    where
        Key: ObjCPtr + Into<K>,
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
#[derive(Clone)]
pub struct NSDictionary<K: ObjCPtr, V: ObjCPtr> {
    ptr: OwnedObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K: ObjCPtr, V: ObjCPtr> ObjCPtr for NSDictionary<K, V> {
    type Owned = Self;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self {
            ptr,
            _marker_k: std::marker::PhantomData,
            _marker_v: std::marker::PhantomData,
        }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw()
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_Foundation_NSDictionary_class() }
            .into_opt()
            .expect("expecting +[NSDictionary class] to return a non null pointer")
    }
}

impl<K: ObjCPtr, V: ObjCPtr> NSObjectProtocol for NSDictionary<K, V> {}
impl<K: ObjCPtr, V: ObjCPtr> NSObjectInterface for NSDictionary<K, V> {}
impl<K: ObjCPtr, V: ObjCPtr> NSDictionaryInterface<K, V> for NSDictionary<K, V> {}

impl<K: ObjCPtr, V: ObjCPtr> From<NSDictionary<K, V>> for NSObject {
    fn from(obj: NSDictionary<K, V>) -> Self {
        unsafe { NSObject::from_owned_unchecked(obj.ptr) }
    }
}