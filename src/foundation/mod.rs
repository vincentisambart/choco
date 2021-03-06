use crate::base::{
    AsRaw, NSUInteger, ObjCClass, Ptr, PtrBehavior, RawClassPtr, RawObjPtr, Retained, Type,
    TypeKind, BOOL,
};

mod nsarray;
// mod nsdictionary;
mod nsstring;
pub use nsarray::*;
// pub use nsdictionary::*;
pub use nsstring::*;
// pub(crate) mod prelude;

//-------------------------------------------------------------------
// NSObject
//
// Note that technically NSObject is not really part of Foundation,
// more part of the Objective-C runtime, but having it here makes
// things easier to understand, and it relies on NSString anyway.

extern "C" {
    fn choco_Foundation_NSObjectProtocol_instance_hash(self_: RawObjPtr) -> NSUInteger;
    fn choco_Foundation_NSObjectProtocol_instance_isEqual(
        self_: RawObjPtr,
        other: Option<RawObjPtr>,
    ) -> BOOL;
    fn choco_Foundation_NSObjectProtocol_instance_isKindOfClass(
        self_: RawObjPtr,
        class: RawClassPtr,
    ) -> BOOL;
    fn choco_Foundation_NSObjectProtocol_instance_description(
        self_: RawObjPtr,
    ) -> Option<RawObjPtr>;
    fn choco_Foundation_NSObjectProtocol_instance_debugDescription(
        self_: RawObjPtr,
    ) -> Option<RawObjPtr>;

    fn choco_Foundation_NSObject_class() -> RawClassPtr;
    fn choco_Foundation_NSObjectInterface_class_new(class: RawClassPtr) -> Option<RawObjPtr>;
}

pub trait NSObjectProtocol: ObjCClass {}

trait NSObjectProtocolInstanceMethods: AsRaw {
    fn hash(&self) -> usize {
        unsafe { choco_Foundation_NSObjectProtocol_instance_hash(self.as_raw()) }
    }

    // In Objective-C, the parameter to -[NSObject isEqual:] is nullable,
    // we consider it non-nullable to makes things simpler.
    fn is_equal<OtherT: NSObjectProtocol, OtherBehavior: PtrBehavior>(
        &self,
        obj: &Ptr<OtherT, OtherBehavior>,
    ) -> bool {
        let self_raw = self.as_raw();
        let obj_raw = obj.as_raw();
        let ret =
            unsafe { choco_Foundation_NSObjectProtocol_instance_isEqual(self_raw, obj_raw.into()) };
        ret.into()
    }

    fn is_kind_of(&self, class: RawClassPtr) -> bool {
        let self_raw = self.as_raw();
        let ret =
            unsafe { choco_Foundation_NSObjectProtocol_instance_isKindOfClass(self_raw, class) };
        ret.into()
    }

    // fn description(&self) -> Retained<NSString> {
    //     let self_raw = self.as_raw();
    //     unsafe {
    //         let raw = choco_Foundation_NSObjectProtocol_instance_description(self_raw).unwrap();
    //         Retained::from_owned_raw_unchecked(raw)
    //     }
    // }

    // fn debug_description(&self) -> Retained<NSString> {
    //     let self_raw = self.as_raw();
    //     unsafe {
    //         let raw = choco_Foundation_NSObjectProtocol_instance_debugDescription(self_raw).unwrap();
    //         Retained::from_owned_raw_unchecked(raw)
    //     }
    // }
}

impl<T, Behavior> NSObjectProtocolInstanceMethods for Ptr<T, Behavior>
where
    T: NSObjectProtocol,
    Behavior: PtrBehavior,
{
}

pub trait NSObjectInterface: NSObjectProtocol + ObjCClass {
    fn new() -> Ptr<Self, Retained> {
        unsafe {
            let raw = choco_Foundation_NSObjectInterface_class_new(Self::class()).unwrap();
            Ptr::from_raw_unchecked(raw)
        }
    }
}

pub struct NSObject {}

impl Type for NSObject {
    const KIND: TypeKind = TypeKind::ObjC;
}

impl ObjCClass for NSObject {
    fn class() -> RawClassPtr {
        unsafe { choco_Foundation_NSObject_class() }
    }
}
impl NSObjectProtocol for NSObject {}
impl NSObjectInterface for NSObject {}

impl<LhsT, LhsBehavior, RhsT, RhsBehavior> std::cmp::PartialEq<Ptr<RhsT, RhsBehavior>>
    for Ptr<LhsT, LhsBehavior>
where
    LhsT: NSObjectInterface,
    RhsT: NSObjectInterface,
    LhsBehavior: PtrBehavior,
    RhsBehavior: PtrBehavior,
{
    fn eq(&self, other: &Ptr<RhsT, RhsBehavior>) -> bool {
        self.is_equal(other)
    }
}

#[cfg(test)]
#[allow(clippy::eq_op)]
mod tests {
    use super::*;

    #[test]
    fn object_comparison() {
        let obj1 = NSObject::new();
        let obj2 = NSObject::new();
        assert!(obj1.is_equal(&obj1));
        assert!(obj2.is_equal(&obj2));
        assert!(!obj1.is_equal(&obj2));
        assert!(obj1 == obj1);
        assert!(obj2 == obj2);
        assert!(obj1 != obj2);
    }

    #[test]
    fn retain() {
        let obj1 = NSObject::new();
        let obj2 = obj1.retain();
        assert!(obj1.is_equal(&obj2));
        assert!(obj1 == obj2);
    }
}

// //-------------------------------------------------------------------
// // NSCopying/NSMutableCopying

// extern "C" {
//     // Technically, copy and mutableCopy are methods of NSObject, but they will just throw an exception for types that are not NSCopying/NSMutableCopying.
//     fn choco_Foundation_NSCopyingProtocol_copy(self_: RawObjPtr) -> Option<RawObjPtr>;
//     fn choco_Foundation_NSMutableCopyingProtocol_mutableCopy(
//         self_: RawObjPtr,
//     ) -> Option<RawObjPtr>;
// }

// // pub trait NSCopyingProtocol: objc::NSObjectInterface {
// //     type Immutable: FromOwned;

// //     fn copy(&self) -> Self::Immutable {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe {
// //             let owned_ptr = choco_Foundation_NSCopyingProtocol_copy(raw_self)
// //                 .unwrap()
// //                 .consider_owned();
// //             Self::Immutable::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }
// // }

// // pub trait NSMutableCopyingProtocol: NSObjectInterface {
// //     type Mutable: FromOwned;

// //     fn mutable_copy(&self) -> Self::Mutable {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe {
// //             let owned_ptr = choco_Foundation_NSMutableCopyingProtocol_mutableCopy(raw_self)
// //                 .unwrap()
// //                 .consider_owned();
// //             Self::Mutable::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }
// // }

//-------------------------------------------------------------------
// NSFastEnumeration

// Useful documentation:
// - NSFastEnumeration's Apple documentation https://developer.apple.com/documentation/foundation/nsfastenumeration?language=objc
// - How clang rewrites Objective-C's for loops https://github.com/llvm/llvm-project/blob/ee068aafbc5c6722158d5113290a211503e1cfe4/clang/lib/Frontend/Rewrite/RewriteModernObjC.cpp#L1651-L1682
// - How Swift implements it https://github.com/apple/swift/blob/3a50f93c60f6718ccdb9f685c57e8ac8188e1788/stdlib/public/Darwin/Foundation/NSFastEnumeration.swift

extern "C" {
    fn choco_Foundation_NSFastEnumerationProtocol_instance_countByEnumeratingWithState(
        self_: RawObjPtr,
        state: *mut NSFastEnumerationState,
        buffer: *mut Option<RawObjPtr>,
        len: usize,
    ) -> usize;
}

#[repr(C)]
struct NSFastEnumerationState {
    state: usize,
    items: *mut Option<RawObjPtr>,
    mutations: *mut usize,
    extra: [usize; 5],
}

impl NSFastEnumerationState {
    fn new() -> Self {
        Self {
            state: 0,
            items: std::ptr::null_mut(),
            mutations: std::ptr::null_mut(),
            extra: [0; 5],
        }
    }
}

const FAST_ENUMERATOR_BUFFER_LEN: usize = 16;
pub struct NSFastEnumerationIter<'enumerable, Item>
where
    Item: Type,
{
    enumerable: RawObjPtr,
    /// state.items will not always point to this buffer, it can be using storage local to the enumerable.
    buffer: [Option<RawObjPtr>; FAST_ENUMERATOR_BUFFER_LEN],
    state: NSFastEnumerationState,
    start_mutations: usize,
    /// next index to read in state.items
    index: usize,
    /// count of items currently available in state.items
    preloaded_count: usize,
    _marker: std::marker::PhantomData<&'enumerable Item>,
}

// impl<'enumerable, Item> NSFastEnumerationIter<'enumerable, Item>
// where
//     Item: Type,
// {
//     fn new<Enumerable>(enumerable: &'enumerable Enumerable) -> Self
//     where
//         Enumerable: NSFastEnumerationProtocol<Item>,
//     {
//         Self {
//             enumerable: enumerable.as_raw(),
//             buffer: [Default::default(); FAST_ENUMERATOR_BUFFER_LEN],
//             state: NSFastEnumerationState::new(),
//             start_mutations: 0,
//             index: 0,
//             preloaded_count: 0,
//             _marker: std::marker::PhantomData,
//         }
//     }
// }

// impl<'enumerable, Item> Iterator for NSFastEnumerationIter<'enumerable, Item>
// where
//     Item: Type,
// {
//     type Item = Retained<Item>;

//     fn next(&mut self) -> Option<Retained<Item>> {
//         if self.index + 1 > self.preloaded_count {
//             self.index = 0;

//             let buffer_ptr = self.buffer.as_mut_ptr();
//             self.preloaded_count = unsafe {
//                 choco_Foundation_NSFastEnumerationProtocol_instance_countByEnumeratingWithState(
//                     self.enumerable,
//                     &mut self.state,
//                     buffer_ptr,
//                     FAST_ENUMERATOR_BUFFER_LEN,
//                 )
//             };
//             debug_assert!(
//                 buffer_ptr != self.state.items || self.preloaded_count < FAST_ENUMERATOR_BUFFER_LEN,
//                 "when using our provided buffer, a count longer than the buffer is unexpected"
//             );

//             if self.preloaded_count == 0 {
//                 return None;
//             }

//             self.start_mutations = unsafe { self.state.mutations.read() };
//         } else if unsafe { self.state.mutations.read() } != self.start_mutations {
//             panic!("mutation detected during iteration");
//         }

//         // The pointer in the buffer is borrowed so we have to retain it.
//         let owned_ptr = unsafe { self.state.items.add(self.index).read().unwrap().retain() };

//         self.index += 1;

//         // Make sure we return an owned value.
//         Some(owned_ptr)
//     }
// }

// pub trait NSFastEnumerationProtocol<Item>: objc::NSObjectInterface
// where
//     Item: Type,
// {
//     fn iter(&'_ self) -> NSFastEnumerationIter<'_, Item> {
//         NSFastEnumerationIter::new(self)
//     }
// }

// #[cfg(test)]
// mod nsfastenumeration_tests {
//     use super::*;

//     #[test]
//     fn size() {
//         assert_eq!(std::mem::size_of::<NSFastEnumerationState>(), 64);
//     }

//     #[test]
//     fn iter() {
//         use objc::NSObjectInterfaceClassMethods;

//         // Should test most length-related corner cases
//         for array_len in (0..FAST_ENUMERATOR_BUFFER_LEN * 3) {
//             objc::autorelease_pool(|| {
//                 let array: Retained<NSMutableArray<NSString>> = NSMutableArray::new();
//                 for i in 0..array_len {
//                     let text = format!("item{}", i);
//                     array.add_object(&NSString::new_with_str(&text));
//                 }
//                 let vec = array
//                     .iter()
//                     .map(|item| item.to_string().unwrap())
//                     .collect::<Vec<_>>();
//                 assert_eq!(vec.len(), array_len);
//                 for (i, item) in vec.iter().enumerate() {
//                     let expected_text = format!("item{}", i);
//                     assert_eq!(item, &expected_text);
//                 }
//             });
//         }
//     }
// }

// // //-------------------------------------------------------------------
// // // NSURL

// // extern "C" {
// //     fn choco_Foundation_NSURL_class() -> objc::ClassPtr;
// //     fn choco_Foundation_NSURLInterface_class_newWithString(
// //         class: objc::ClassPtr,
// //         urlString: RawObjPtr,
// //     ) -> Option<RawObjPtr>;
// //     fn choco_Foundation_NSURLInterface_class_fileURLWithPath(
// //         class: objc::ClassPtr,
// //         path: RawObjPtr,
// //     ) -> Option<RawObjPtr>;
// //     fn choco_Foundation_NSURLInterface_class_fileURLWithPath_isDirectory(
// //         class: objc::ClassPtr,
// //         path: RawObjPtr,
// //         is_directory: BOOL,
// //     ) -> Option<RawObjPtr>;
// //     fn choco_Foundation_NSURLInterface_instance_absoluteString(
// //         self_: RawObjPtr,
// //     ) -> Option<RawObjPtr>;
// // }

// // pub trait NSURLInterface: NSObjectInterface
// // where
// //     Self: NSCopyingProtocol,
// // {
// //     fn new_with_string(string: &impl NSStringInterface) -> Option<Self::Owned> {
// //         unsafe {
// //             choco_Foundation_NSURLInterface_class_newWithString(Self::class(), string.as_raw_ptr())
// //                 .map(|raw| Self::Owned::from_owned_ptr_unchecked(raw.consider_owned()))
// //         }
// //     }

// //     // If you know if path is a directory or not, use file_url_with_path_is_directory() as it does not require to access the file system.
// //     // file_url_with_path() checks on the disk if the path is a directory (if it does not exist, it's not considered a directory).
// //     fn file_url_with_path(path: &impl NSStringInterface) -> Self::Owned {
// //         unsafe {
// //             let owned_ptr = choco_Foundation_NSURLInterface_class_fileURLWithPath(
// //                 Self::class(),
// //                 path.as_raw_ptr(),
// //             )
// //             .unwrap()
// //             .consider_owned();
// //             Self::Owned::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }

// //     fn file_url_with_path_is_directory(
// //         path: &impl NSStringInterface,
// //         is_directory: bool,
// //     ) -> Self::Owned {
// //         unsafe {
// //             let owned_ptr = choco_Foundation_NSURLInterface_class_fileURLWithPath_isDirectory(
// //                 Self::class(),
// //                 path.as_raw_ptr(),
// //                 is_directory.into(),
// //             )
// //             // In fact if the path is empty you will get a nil, but the documentation says you should not pass an empty path.
// //             .unwrap()
// //             .consider_owned();
// //             Self::Owned::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }

// //     fn absolute_string(&self) -> NSString {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe {
// //             let owned_ptr = choco_Foundation_NSURLInterface_instance_absoluteString(raw_self)
// //                 .unwrap()
// //                 .consider_owned();
// //             NSString::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }
// // }

// // pub struct NSURL {
// //     ptr: objc::OwnedPtr,
// // }

// // impl AsRaw for NSURL {
// //     fn as_raw_ptr(&self) -> RawObjPtr {
// //         self.ptr.as_raw_ptr()
// //     }
// // }

// // impl FromOwned for NSURL {
// //     unsafe fn from_owned_ptr_unchecked(ptr: objc::OwnedPtr) -> Self {
// //         Self { ptr }
// //     }
// // }

// // impl NSObjectProtocol for NSURL {
// //     type Owned = Self;

// //     fn class() -> objc::ClassPtr {
// //         unsafe { choco_Foundation_NSURL_class() }
// //     }
// // }

// // impl NSObjectInterface for NSURL {}
// // impl NSURLInterface for NSURL {}
// // impl NSCopyingProtocol for NSURL {
// //     type Immutable = Self;
// // }
// // impl ValidObjCGeneric for NSURL {}
// // unsafe impl IsKindOf<NSObject> for NSURL {}

// // // A NSSURL is immutable so can be shared between threads.
// // unsafe impl Send for NSURL {}
// // unsafe impl Sync for NSURL {}

// // #[cfg(test)]
// // mod url_tests {
// //     use super::*;

// //     #[test]
// //     fn simple_url() {
// //         let invalid_url_string = NSString::new_with_str("🍁");
// //         let valid_url_string = NSString::new_with_str("https://www.rust-lang.org/");
// //         assert!(NSURL::new_with_string(&invalid_url_string).is_none());
// //         let valid_url = NSURL::new_with_string(&valid_url_string).unwrap();
// //         assert!(valid_url.absolute_string().is_equal(&valid_url_string));
// //         assert_eq!(
// //             &valid_url.absolute_string().to_string().unwrap(),
// //             "https://www.rust-lang.org/"
// //         );
// //         assert!(valid_url.is_kind_of(NSObject::class()));
// //         assert!(valid_url.is_kind_of(NSURL::class()));
// //         assert!(!valid_url.is_kind_of(NSString::class()));
// //     }
// // }

// // //-------------------------------------------------------------------
// // // NSDate

// // #[derive(Copy, Clone, PartialEq, PartialOrd)]
// // #[repr(transparent)]
// // pub struct NSTimeInterval {
// //     secs: f64,
// // }

// // impl NSTimeInterval {
// //     pub fn from_secs(secs: f64) -> Self {
// //         Self { secs }
// //     }

// //     pub fn secs(self) -> f64 {
// //         self.secs
// //     }
// // }

// // extern "C" {
// //     fn choco_Foundation_NSDate_class() -> objc::ClassPtr;
// //     fn choco_Foundation_NSDateInterface_instance_timeIntervalSinceNow(
// //         self_: RawObjPtr,
// //     ) -> NSTimeInterval;
// //     fn choco_Foundation_NSDateInterface_instance_timeIntervalSinceReferenceDate(
// //         self_: RawObjPtr,
// //     ) -> NSTimeInterval;
// //     fn choco_Foundation_NSDateInterface_instance_timeIntervalSince1970(
// //         self_: RawObjPtr,
// //     ) -> NSTimeInterval;
// //     fn choco_Foundation_NSDateInterface_instance_timeIntervalSinceDate(
// //         self_: RawObjPtr,
// //         anotherDate: RawObjPtr,
// //     ) -> NSTimeInterval;
// // }

// // pub trait NSDateInterface: NSObjectInterface
// // where
// //     Self: NSCopyingProtocol,
// // {
// //     fn since_now(&self) -> NSTimeInterval {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe { choco_Foundation_NSDateInterface_instance_timeIntervalSinceNow(raw_self) }
// //     }

// //     fn since_reference_date(&self) -> NSTimeInterval {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe {
// //             choco_Foundation_NSDateInterface_instance_timeIntervalSinceReferenceDate(raw_self)
// //         }
// //     }

// //     fn since_1970(&self) -> NSTimeInterval {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe { choco_Foundation_NSDateInterface_instance_timeIntervalSince1970(raw_self) }
// //     }

// //     fn since(&self, another_date: &NSDate) -> NSTimeInterval {
// //         let raw_self = self.as_raw_ptr();
// //         let raw_another_date = another_date.as_raw_ptr();
// //         unsafe {
// //             choco_Foundation_NSDateInterface_instance_timeIntervalSinceDate(
// //                 raw_self,
// //                 raw_another_date,
// //             )
// //         }
// //     }
// // }

// // pub struct NSDate {
// //     ptr: objc::OwnedPtr,
// // }

// // impl AsRaw for NSDate {
// //     fn as_raw_ptr(&self) -> RawObjPtr {
// //         self.ptr.as_raw_ptr()
// //     }
// // }

// // impl FromOwned for NSDate {
// //     unsafe fn from_owned_ptr_unchecked(ptr: objc::OwnedPtr) -> Self {
// //         Self { ptr }
// //     }
// // }

// // impl NSObjectProtocol for NSDate {
// //     type Owned = Self;

// //     fn class() -> objc::ClassPtr {
// //         unsafe { choco_Foundation_NSDate_class() }
// //     }
// // }

// // impl NSObjectInterface for NSDate {}
// // impl NSDateInterface for NSDate {}
// // impl NSCopyingProtocol for NSDate {
// //     type Immutable = Self;
// // }
// // impl ValidObjCGeneric for NSDate {}
// // unsafe impl IsKindOf<NSObject> for NSDate {}

// // impl std::ops::Sub for &NSDate {
// //     type Output = NSTimeInterval;

// //     fn sub(self, rhs: Self) -> Self::Output {
// //         self.since(rhs)
// //     }
// // }

// // // A NSDate is immutable so can be shared between threads.
// // unsafe impl Send for NSDate {}
// // unsafe impl Sync for NSDate {}

// // //-------------------------------------------------------------------
// // // NSValue

// // extern "C" {
// //     fn choco_Foundation_NSValue_class() -> objc::ClassPtr;
// // }

// // pub trait NSValueInterface: NSObjectInterface
// // where
// //     Self: NSCopyingProtocol,
// // {
// // }

// // //-------------------------------------------------------------------
// // // NSNumber

// // extern "C" {
// //     fn choco_Foundation_NSNumber_class() -> objc::ClassPtr;
// //     fn choco_Foundation_NSNumberInterface_class_newWithBool(
// //         class: objc::ClassPtr,
// //         value: BOOL,
// //     ) -> Option<RawObjPtr>;
// //     fn choco_Foundation_NSNumberInterface_class_newWithInteger(
// //         class: objc::ClassPtr,
// //         value: NSInteger,
// //     ) -> Option<RawObjPtr>;
// //     fn choco_Foundation_NSNumberInterface_class_newWithUnsignedInteger(
// //         class: objc::ClassPtr,
// //         value: NSUInteger,
// //     ) -> Option<RawObjPtr>;
// //     fn choco_Foundation_NSNumberInterface_instance_boolValue(self_: RawObjPtr) -> BOOL;
// //     fn choco_Foundation_NSNumberInterface_instance_integerValue(
// //         self_: RawObjPtr,
// //     ) -> NSInteger;
// //     fn choco_Foundation_NSNumberInterface_instance_unsignedIntegerValue(
// //         self_: RawObjPtr,
// //     ) -> NSUInteger;
// // }

// // pub trait NSNumberInterface: NSValueInterface {
// //     fn from_bool(value: bool) -> Self::Owned {
// //         unsafe {
// //             let owned_ptr =
// //                 choco_Foundation_NSNumberInterface_class_newWithBool(Self::class(), value.into())
// //                     .unwrap()
// //                     .consider_owned();
// //             Self::Owned::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }

// //     fn from_isize(value: isize) -> Self::Owned {
// //         unsafe {
// //             let owned_ptr =
// //                 choco_Foundation_NSNumberInterface_class_newWithInteger(Self::class(), value)
// //                     .unwrap()
// //                     .consider_owned();
// //             Self::Owned::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }

// //     fn from_usize(value: usize) -> Self::Owned {
// //         unsafe {
// //             let owned_ptr = choco_Foundation_NSNumberInterface_class_newWithUnsignedInteger(
// //                 Self::class(),
// //                 value,
// //             )
// //             .unwrap()
// //             .consider_owned();
// //             Self::Owned::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }

// //     fn as_bool(&self) -> bool {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe { choco_Foundation_NSNumberInterface_instance_boolValue(raw_self) }.into()
// //     }

// //     fn as_isize(&self) -> isize {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe { choco_Foundation_NSNumberInterface_instance_integerValue(raw_self) }
// //     }

// //     fn as_usize(&self) -> usize {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe { choco_Foundation_NSNumberInterface_instance_unsignedIntegerValue(raw_self) }
// //     }
// // }

// // pub struct NSNumber {
// //     ptr: objc::OwnedPtr,
// // }

// // impl AsRaw for NSNumber {
// //     fn as_raw_ptr(&self) -> RawObjPtr {
// //         self.ptr.as_raw_ptr()
// //     }
// // }

// // impl FromOwned for NSNumber {
// //     unsafe fn from_owned_ptr_unchecked(ptr: objc::OwnedPtr) -> Self {
// //         Self { ptr }
// //     }
// // }

// // impl NSObjectProtocol for NSNumber {
// //     type Owned = Self;

// //     fn class() -> objc::ClassPtr {
// //         unsafe { choco_Foundation_NSNumber_class() }
// //     }
// // }

// // impl NSObjectInterface for NSNumber {}
// // impl NSValueInterface for NSNumber {}
// // impl NSNumberInterface for NSNumber {}
// // impl NSCopyingProtocol for NSNumber {
// //     type Immutable = Self;
// // }
// // impl ValidObjCGeneric for NSNumber {}
// // unsafe impl IsKindOf<NSObject> for NSNumber {}

// // // A NSNumber is immutable so can be shared between threads.
// // unsafe impl Send for NSNumber {}
// // unsafe impl Sync for NSNumber {}

// // #[cfg(test)]
// // mod number_tests {
// //     use super::*;

// //     #[test]
// //     fn bool_value() {
// //         let t1 = NSNumber::from_bool(true);
// //         let t2 = NSNumber::from_bool(true);
// //         let f = NSNumber::from_bool(false);
// //         assert!(t1.is_kind_of(NSNumber::class()));
// //         assert!(t2.is_kind_of(NSNumber::class()));
// //         assert!(t1.is_equal(&t1));
// //         assert!(t1.is_equal(&t2));
// //         assert!(!t1.is_equal(&f));
// //         assert_eq!(t1.as_bool(), true);
// //         assert_eq!(t2.as_bool(), true);
// //         assert_eq!(f.as_bool(), false);
// //     }

// //     #[test]
// //     fn isize_value() {
// //         let t = NSNumber::from_bool(true);
// //         let f = NSNumber::from_bool(false);
// //         let i = NSNumber::from_isize(12345657890);
// //         assert!(t.is_kind_of(NSNumber::class()));
// //         assert!(f.is_kind_of(NSNumber::class()));
// //         assert!(i.is_kind_of(NSNumber::class()));
// //         assert_eq!(t.as_isize(), 1);
// //         assert_eq!(f.as_isize(), 0);
// //         assert_eq!(i.as_isize(), 12345657890);
// //     }
// // }

// // //-------------------------------------------------------------------
// // // NSError

// // extern "C" {
// //     fn choco_Foundation_NSError_class() -> objc::ClassPtr;
// //     fn choco_Foundation_NSErrorInterface_instance_code(self_: RawObjPtr) -> NSInteger;
// //     fn choco_Foundation_NSErrorInterface_instance_domain(
// //         self_: RawObjPtr,
// //     ) -> Option<RawObjPtr>;
// //     fn choco_Foundation_NSErrorInterface_instance_localizedDescription(
// //         self_: RawObjPtr,
// //     ) -> Option<RawObjPtr>;
// // }

// // pub trait NSErrorInterface: NSObjectInterface
// // where
// //     Self: NSCopyingProtocol,
// // {
// //     fn code(&self) -> NSInteger {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe { choco_Foundation_NSErrorInterface_instance_code(raw_self) }
// //     }

// //     fn domain(&self) -> NSString {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe {
// //             let owned_ptr = choco_Foundation_NSErrorInterface_instance_domain(raw_self)
// //                 .unwrap()
// //                 .consider_owned();
// //             NSString::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }

// //     fn localized_description(&self) -> NSString {
// //         let raw_self = self.as_raw_ptr();
// //         unsafe {
// //             let owned_ptr =
// //                 choco_Foundation_NSErrorInterface_instance_localizedDescription(raw_self)
// //                     .unwrap()
// //                     .consider_owned();
// //             NSString::from_owned_ptr_unchecked(owned_ptr)
// //         }
// //     }
// // }

// // pub struct NSError {
// //     ptr: objc::OwnedPtr,
// // }

// // impl AsRaw for NSError {
// //     fn as_raw_ptr(&self) -> RawObjPtr {
// //         self.ptr.as_raw_ptr()
// //     }
// // }

// // impl FromOwned for NSError {
// //     unsafe fn from_owned_ptr_unchecked(ptr: objc::OwnedPtr) -> Self {
// //         Self { ptr }
// //     }
// // }

// // impl NSObjectProtocol for NSError {
// //     type Owned = Self;

// //     fn class() -> objc::ClassPtr {
// //         unsafe { choco_Foundation_NSError_class() }
// //     }
// // }

// // impl NSObjectInterface for NSError {}
// // impl NSErrorInterface for NSError {}
// // impl NSCopyingProtocol for NSError {
// //     type Immutable = Self;
// // }
// // impl ValidObjCGeneric for NSError {}
// // unsafe impl IsKindOf<NSObject> for NSError {}

// // // A NSError is immutable so can be shared between threads.
// // unsafe impl Send for NSError {}
// // unsafe impl Sync for NSError {}

// // impl std::fmt::Debug for NSError {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         let description = self.debug_description();
// //         let rusty = description.to_string_lossy();
// //         f.write_str(&rusty)
// //     }
// // }

// // /// # Safety
// // /// - if non null, raw_ptr must be owned and of type T.
// // /// - if non null, raw_unowned_error must not be owned (probably autoreleased) and point to a NSError.
// // pub(crate) unsafe fn make_object_result_unchecked<T: NSObjectInterface>(
// //     raw_ptr: Option<RawObjPtr>,
// //     raw_unowned_error: Option<RawObjPtr>,
// // ) -> Result<T::Owned, NSError> {
// //     // Create the object before checking the error,
// //     // because if both the new object and error are not null,
// //     // we want to the object to be properly released.
// //     let obj = raw_ptr.map(|raw_ptr| T::Owned::from_owned_ptr_unchecked(raw_ptr.consider_owned()));
// //     match raw_unowned_error {
// //         None => Ok(obj.expect("expecting non null return value pointer when no error")),
// //         Some(raw_error) => Err(NSError::from_owned_ptr_unchecked(raw_error.retain())),
// //     }
// // }

// // /// # Safety
// // /// - if non null, raw_unowned_error must not be owned (probably autoreleased) and point to a NSError.
// // pub(crate) unsafe fn make_value_result_unchecked<T>(
// //     value: T,
// //     raw_unowned_error: Option<RawObjPtr>,
// // ) -> Result<T, NSError> {
// //     match raw_unowned_error {
// //         None => Ok(value),
// //         Some(raw_error) => Err(NSError::from_owned_ptr_unchecked(raw_error.retain())),
// //     }
// // }
