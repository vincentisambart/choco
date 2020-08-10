use super::*;
use crate::base::ptr;

//-------------------------------------------------------------------
// NSArray interface

extern "C" {
    fn choco_Foundation_NSArray_class() -> ptr::objc::ClassPtr;
    fn choco_Foundation_NSArrayInterface_instance_count(self_: ptr::objc::RawPtr) -> usize;
    fn choco_Foundation_NSArrayInterface_instance_firstObject(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_Foundation_NSArrayInterface_instance_lastObject(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_Foundation_NSArrayInterface_instance_objectAtIndex(
        self_: ptr::objc::RawPtr,
        index: usize,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(
        self_: ptr::objc::RawPtr,
        obj: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
}

pub trait NSArrayInterface<T: ValidObjCGeneric>: NSObjectInterface
where
    Self: NSCopyingProtocol + NSMutableCopyingProtocol + NSFastEnumerationProtocol<T>,
{
    fn first(&self) -> Option<T> {
        let raw_self = self.as_raw();
        unsafe {
            choco_Foundation_NSArrayInterface_instance_firstObject(raw_self)
                .into_opt()
                .map(|raw| T::from_owned_ptr_unchecked(raw.consider_owned()))
        }
    }
    fn last(&self) -> Option<T> {
        let raw_self = self.as_raw();
        unsafe {
            choco_Foundation_NSArrayInterface_instance_lastObject(raw_self)
                .into_opt()
                .map(|raw| T::from_owned_ptr_unchecked(raw.consider_owned()))
        }
    }

    fn object_at(&self, index: usize) -> T {
        let raw_self = self.as_raw();
        unsafe {
            let owned_ptr =
                choco_Foundation_NSArrayInterface_instance_objectAtIndex(raw_self, index)
                    .unwrap()
                    .consider_owned();
            T::from_owned_ptr_unchecked(owned_ptr)
        }
    }

    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSArrayInterface_instance_count(raw_self) }
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    #[must_use]
    fn adding_object<Object>(&self, object: &Object) -> NSArray<T>
    where
        Object: IsKindOf<T>,
    {
        let raw_self = self.as_raw();
        let raw_obj = object.as_raw();
        unsafe {
            let owned_ptr =
                choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(raw_self, raw_obj)
                    .unwrap()
                    .consider_owned();
            NSArray::from_owned_ptr_unchecked(owned_ptr)
        }
    }
}

//-------------------------------------------------------------------
// NSArray

pub struct NSArray<T: ValidObjCGeneric> {
    ptr: ptr::objc::OwnedPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ValidObjCGeneric> ptr::objc::AsRawPtr for NSArray<T> {
    fn as_raw(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw()
    }
}

impl<T: ValidObjCGeneric> FromOwnedPtr for NSArray<T> {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: ValidObjCGeneric> NSObjectProtocol for NSArray<T> {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_Foundation_NSArray_class() }
    }
}

impl<T: ValidObjCGeneric> NSObjectInterface for NSArray<T> {}
impl<T: ValidObjCGeneric> NSArrayInterface<T> for NSArray<T> {}

impl<T: ValidObjCGeneric> NSCopyingProtocol for NSArray<T> {
    type Immutable = ImmutableNSArray<T>;
}

impl<T: ValidObjCGeneric> NSMutableCopyingProtocol for NSArray<T> {
    type Mutable = NSMutableArray<T>;
}

impl<T: ValidObjCGeneric> NSFastEnumerationProtocol<T> for NSArray<T> {}
impl<T: ValidObjCGeneric> ValidObjCGeneric for NSArray<T> {}
impl<T: ValidObjCGeneric> IsKindOf<NSObject> for NSArray<T> {}

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
// ImmutableNSArray

/// Version of NSArray we are statically sure to be immutable.
pub struct ImmutableNSArray<T: ValidObjCGeneric> {
    ptr: ptr::objc::OwnedPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ValidObjCGeneric> ptr::objc::AsRawPtr for ImmutableNSArray<T> {
    fn as_raw(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw()
    }
}

impl<T: ValidObjCGeneric> FromOwnedPtr for ImmutableNSArray<T> {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: ValidObjCGeneric> NSObjectProtocol for ImmutableNSArray<T> {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_Foundation_NSArray_class() }
    }
}

impl<T: ValidObjCGeneric> NSObjectInterface for ImmutableNSArray<T> {}
impl<T: ValidObjCGeneric> NSArrayInterface<T> for ImmutableNSArray<T> {}

impl<T: ValidObjCGeneric> NSCopyingProtocol for ImmutableNSArray<T> {
    type Immutable = Self;
}

impl<T: ValidObjCGeneric> NSMutableCopyingProtocol for ImmutableNSArray<T> {
    type Mutable = NSMutableArray<T>;
}

impl<T: ValidObjCGeneric> NSFastEnumerationProtocol<T> for ImmutableNSArray<T> {}
impl<T: ValidObjCGeneric> IsKindOf<NSObject> for ImmutableNSArray<T> {}
impl<T: ValidObjCGeneric> IsKindOf<NSArray<T>> for ImmutableNSArray<T> {}

// An ImmutableNSArray is known to be immutable so can be shared between threads.
unsafe impl<T: ValidObjCGeneric> Send for ImmutableNSArray<T> {}
unsafe impl<T: ValidObjCGeneric> Sync for ImmutableNSArray<T> {}

//-------------------------------------------------------------------
// NSMutableArray interface

extern "C" {
    fn choco_Foundation_NSMutableArray_class() -> ptr::objc::ClassPtr;
    fn choco_Foundation_NSMutableArrayInterface_instance_addObject(
        self_: ptr::objc::RawPtr,
        object: ptr::objc::RawPtr,
    );
}

pub trait NSMutableArrayInterface<T: ValidObjCGeneric>: NSArrayInterface<T> {
    fn add_object<Value>(&self, value: &Value)
    where
        Value: IsKindOf<T>,
    {
        let raw_self = self.as_raw();
        let raw_value = value.as_raw();
        unsafe { choco_Foundation_NSMutableArrayInterface_instance_addObject(raw_self, raw_value) }
    }
}

//-------------------------------------------------------------------
// NSMutableArray

pub struct NSMutableArray<T: ValidObjCGeneric> {
    ptr: ptr::objc::OwnedPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ValidObjCGeneric> ptr::objc::AsRawPtr for NSMutableArray<T> {
    fn as_raw(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw()
    }
}

impl<T: ValidObjCGeneric> FromOwnedPtr for NSMutableArray<T> {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: ValidObjCGeneric> NSObjectProtocol for NSMutableArray<T> {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_Foundation_NSMutableArray_class() }
    }
}

impl<T: ValidObjCGeneric> NSObjectInterface for NSMutableArray<T> {}
impl<T: ValidObjCGeneric> NSArrayInterface<T> for NSMutableArray<T> {}
impl<T: ValidObjCGeneric> NSMutableArrayInterface<T> for NSMutableArray<T> {}

impl<T: ValidObjCGeneric> NSCopyingProtocol for NSMutableArray<T> {
    type Immutable = ImmutableNSArray<T>;
}

impl<T: ValidObjCGeneric> NSMutableCopyingProtocol for NSMutableArray<T> {
    type Mutable = NSMutableArray<T>;
}

impl<T: ValidObjCGeneric> NSFastEnumerationProtocol<T> for NSMutableArray<T> {}
impl<T: ValidObjCGeneric> ValidObjCGeneric for NSMutableArray<T> {}
impl<T: ValidObjCGeneric> IsKindOf<NSObject> for NSMutableArray<T> {}
impl<T: ValidObjCGeneric> IsKindOf<NSArray<T>> for NSMutableArray<T> {}

#[cfg(test)]
mod mutable_array_tests {
    use super::*;

    #[test]
    fn simple_array() {
        let array: NSMutableArray<NSString> = NSMutableArray::new();
        assert!(array.is_empty());
        assert_eq!(array.count(), 0);
        let value = NSString::new_with_str("abcd");
        array.add_object(&value);
        assert!(!array.is_empty());
        assert_eq!(array.count(), 1);
        assert!(array.object_at(0).is_equal(&value));
    }
}
