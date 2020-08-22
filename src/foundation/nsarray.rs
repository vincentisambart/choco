use super::*;
use crate::base::{objc, ptr};
use objc::{IsKindOf, NSObjectInterface, NSObjectInterfaceClassMethods, NSObjectProtocol};
use ptr::{AsRaw, ObjCClass};

//-------------------------------------------------------------------
// NSArray interface

extern "C" {
    fn choco_Foundation_NSArray_class() -> ptr::ClassPtr;
    fn choco_Foundation_NSArrayInterface_instance_count(self_: ptr::RawPtr) -> usize;
    fn choco_Foundation_NSArrayInterface_instance_firstObject(
        self_: ptr::RawPtr,
    ) -> Option<ptr::RawPtr>;
    fn choco_Foundation_NSArrayInterface_instance_lastObject(
        self_: ptr::RawPtr,
    ) -> Option<ptr::RawPtr>;
    fn choco_Foundation_NSArrayInterface_instance_objectAtIndex(
        self_: ptr::RawPtr,
        index: usize,
    ) -> Option<ptr::RawPtr>;
    fn choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(
        self_: ptr::RawPtr,
        obj: ptr::RawPtr,
    ) -> Option<ptr::RawPtr>;
}

pub trait NSArrayInterface<T: ptr::Type>: crate::base::objc::NSObjectInterface
where
    // Self: NSCopyingProtocol + NSMutableCopyingProtocol + NSFastEnumerationProtocol<T>,
    Self: NSFastEnumerationProtocol<T>,
{
    fn first(&self) -> Option<ptr::OwnedPtr<T>> {
        let raw_self = self.as_raw();
        unsafe {
            choco_Foundation_NSArrayInterface_instance_firstObject(raw_self)
                .map(|raw| ptr::OwnedPtr::from_owned_raw_unchecked(raw))
        }
    }
    fn last(&self) -> Option<ptr::OwnedPtr<T>> {
        let raw_self = self.as_raw();
        unsafe {
            choco_Foundation_NSArrayInterface_instance_lastObject(raw_self)
                .map(|raw| ptr::OwnedPtr::from_owned_raw_unchecked(raw))
        }
    }

    fn object_at(&self, index: usize) -> ptr::OwnedPtr<T> {
        let raw_self = self.as_raw();
        unsafe {
            let raw =
                choco_Foundation_NSArrayInterface_instance_objectAtIndex(raw_self, index).unwrap();
            ptr::OwnedPtr::from_owned_raw_unchecked(raw)
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
    fn adding_object<Object, Ptr>(&self, object: &Ptr) -> ptr::OwnedPtr<NSArray<T>>
    where
        Object: IsKindOf<T>,
        Ptr: ptr::PtrHolder<Object>,
    {
        let raw_self = self.as_raw();
        let raw_obj = object.as_raw();
        unsafe {
            let raw =
                choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(raw_self, raw_obj)
                    .unwrap();
            ptr::OwnedPtr::from_owned_raw_unchecked(raw)
        }
    }
}

//-------------------------------------------------------------------
// NSArray

pub struct NSArray<T: ptr::Type> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> ptr::Type for NSArray<T>
where
    T: ptr::Type,
{
    const KIND: ptr::TypeKind = ptr::TypeKind::ObjC;
}

impl<T> ObjCClass for NSArray<T>
where
    T: ptr::Type,
{
    fn class() -> ptr::ClassPtr {
        unsafe { choco_Foundation_NSArray_class() }
    }
}

impl<T> NSObjectInterfaceClassMethods for NSArray<T> where T: ptr::Type {}

impl<T> NSObjectProtocol for ptr::OwnedPtr<NSArray<T>>
where
    T: ptr::Type,
{
    type Class = NSArray<T>;
}
impl<T> NSObjectInterface for ptr::OwnedPtr<NSArray<T>> where T: ptr::Type {}
impl<T> NSArrayInterface<T> for ptr::OwnedPtr<NSArray<T>> where T: ptr::Type {}
impl<T> NSFastEnumerationProtocol<T> for ptr::OwnedPtr<NSArray<T>> where T: ptr::Type {}

unsafe impl<T> IsKindOf<objc::NSObject> for NSArray<T> where T: ptr::Type {}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn empty_arrays() {
        let array1: ptr::OwnedPtr<NSArray<objc::NSObject>> = NSArray::new();
        let array2: ptr::OwnedPtr<NSArray<objc::NSObject>> = NSArray::new();
        assert!(array1.is_equal(&array1));
        assert!(array1.is_equal(&array2));
        assert_eq!(array1.count(), 0);
        assert_eq!(array2.count(), 0);
        assert!(array1.first().is_none());
        assert!(array1.last().is_none());
    }

    #[test]
    fn adding() {
        let array1: ptr::OwnedPtr<NSArray<objc::NSObject>> = NSArray::new();
        let array2: ptr::OwnedPtr<NSArray<objc::NSObject>> = NSArray::new();
        let obj1 = objc::NSObject::new();
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
// NSMutableArray interface

extern "C" {
    fn choco_Foundation_NSMutableArray_class() -> ptr::ClassPtr;
    fn choco_Foundation_NSMutableArrayInterface_instance_addObject(
        self_: ptr::RawPtr,
        object: ptr::RawPtr,
    );
}

pub trait NSMutableArrayInterface<T: ptr::Type>: NSArrayInterface<T> {
    fn add_object<Object, Ptr>(&self, value: &Ptr)
    where
        Object: IsKindOf<T>,
        Ptr: ptr::PtrHolder<Object>,
    {
        let raw_self = self.as_raw();
        let raw_value = value.as_raw();
        unsafe { choco_Foundation_NSMutableArrayInterface_instance_addObject(raw_self, raw_value) }
    }
}

//-------------------------------------------------------------------
// NSMutableArray

pub struct NSMutableArray<T: ptr::Type> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> ptr::Type for NSMutableArray<T>
where
    T: ptr::Type,
{
    const KIND: ptr::TypeKind = ptr::TypeKind::ObjC;
}

impl<T> ObjCClass for NSMutableArray<T>
where
    T: ptr::Type,
{
    fn class() -> ptr::ClassPtr {
        unsafe { choco_Foundation_NSMutableArray_class() }
    }
}

impl<T> NSObjectInterfaceClassMethods for NSMutableArray<T> where T: ptr::Type {}

impl<T> NSObjectProtocol for ptr::OwnedPtr<NSMutableArray<T>>
where
    T: ptr::Type,
{
    type Class = NSMutableArray<T>;
}
impl<T> NSObjectInterface for ptr::OwnedPtr<NSMutableArray<T>> where T: ptr::Type {}
impl<T> NSArrayInterface<T> for ptr::OwnedPtr<NSMutableArray<T>> where T: ptr::Type {}
impl<T> NSMutableArrayInterface<T> for ptr::OwnedPtr<NSMutableArray<T>> where T: ptr::Type {}
impl<T> NSFastEnumerationProtocol<T> for ptr::OwnedPtr<NSMutableArray<T>> where T: ptr::Type {}

unsafe impl<T> IsKindOf<objc::NSObject> for NSMutableArray<T> where T: ptr::Type {}
unsafe impl<T> IsKindOf<NSArray<T>> for NSMutableArray<T> where T: ptr::Type {}

#[cfg(test)]
mod mutable_array_tests {
    use super::*;

    #[test]
    fn simple_array() {
        let array: ptr::OwnedPtr<NSMutableArray<NSString>> = NSMutableArray::new();
        assert!(array.is_empty());
        assert_eq!(array.count(), 0);
        let value = NSString::new_with_str("abcd");
        array.add_object(&value);
        assert!(!array.is_empty());
        assert_eq!(array.count(), 1);
        assert!(array.object_at(0).is_equal(&value));
    }
}
