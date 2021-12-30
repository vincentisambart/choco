use super::{NSObject, NSObjectInterface, NSObjectProtocol};
use crate::base::{
    AsRaw, IsKindOf, ObjCClass, Ptr, Ownership, RawClassPtr, RawObjPtr, Retained, Type, TypeKind,
};

//-------------------------------------------------------------------
// NSArray

extern "C" {
    fn choco_Foundation_NSArray_class() -> RawClassPtr;
    fn choco_Foundation_NSArrayInterface_instance_count(self_: RawObjPtr) -> usize;
    fn choco_Foundation_NSArrayInterface_instance_firstObject(
        self_: RawObjPtr,
    ) -> Option<RawObjPtr>;
    fn choco_Foundation_NSArrayInterface_instance_lastObject(self_: RawObjPtr)
        -> Option<RawObjPtr>;
    fn choco_Foundation_NSArrayInterface_instance_objectAtIndex(
        self_: RawObjPtr,
        index: usize,
    ) -> Option<RawObjPtr>;
    fn choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(
        self_: RawObjPtr,
        obj: RawObjPtr,
    ) -> Option<RawObjPtr>;
}

pub trait NSArrayInterface: NSObjectInterface
// Self: NSCopyingProtocol + NSMutableCopyingProtocol + NSFastEnumerationProtocol<T>,
// Self: NSFastEnumerationProtocol<T>,
{
    type Item: Type;
}

trait NSArrayInterfaceInstanceMethods: AsRaw {
    type Item: Type;

    fn first(&self) -> Option<Ptr<Self::Item, Retained>> {
        let raw_self = self.as_raw();
        unsafe {
            choco_Foundation_NSArrayInterface_instance_firstObject(raw_self)
                .map(|raw| Ptr::from_raw_unchecked(raw))
        }
    }
    fn last(&self) -> Option<Ptr<Self::Item, Retained>> {
        let raw_self = self.as_raw();
        unsafe {
            choco_Foundation_NSArrayInterface_instance_lastObject(raw_self)
                .map(|raw| Ptr::from_raw_unchecked(raw))
        }
    }

    fn object_at(&self, index: usize) -> Ptr<Self::Item, Retained> {
        let raw_self = self.as_raw();
        unsafe {
            let raw =
                choco_Foundation_NSArrayInterface_instance_objectAtIndex(raw_self, index).unwrap();
            Ptr::from_raw_unchecked(raw)
        }
    }

    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSArrayInterface_instance_count(raw_self) }
    }

    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    // TODO: Check if arrayByAddingObject on a NSMutableArray returns a NSArray or NSMutableArray
    #[must_use]
    fn adding_object<OtherT, OtherOwnership>(
        &self,
        object: &Ptr<OtherT, OtherOwnership>,
    ) -> Ptr<NSArray<Self::Item>, Retained>
    where
        OtherT: IsKindOf<Self::Item>,
        OtherOwnership: Ownership,
    {
        let raw_self = self.as_raw();
        let raw_obj = object.as_raw();
        unsafe {
            let raw =
                choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(raw_self, raw_obj)
                    .unwrap();
            Ptr::from_raw_unchecked(raw)
        }
    }
}

impl<T, O> NSArrayInterfaceInstanceMethods for Ptr<T, O>
where
    T: NSArrayInterface,
    O: Ownership,
{
    type Item = T::Item;
}

pub struct NSArray<T: Type> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Type> Type for NSArray<T> {
    const KIND: TypeKind = TypeKind::ObjC;
}

unsafe impl<T: Type> IsKindOf<NSObject> for NSArray<T> {}

impl<T: Type> ObjCClass for NSArray<T> {
    fn class() -> RawClassPtr {
        unsafe { choco_Foundation_NSArray_class() }
    }
}
impl<T: Type> NSObjectProtocol for NSArray<T> {}
impl<T: Type> NSObjectInterface for NSArray<T> {}
impl<T: Type> NSArrayInterface for NSArray<T> {
    type Item = T;
}

// impl<T> NSFastEnumerationProtocol<T> for ptr::OwnedPtr<NSArray<T>> where T: ptr::Type {}

// unsafe impl<T> IsKindOf<NSObject> for NSArray<T> where T: ptr::Type {}

#[cfg(test)]
mod array_tests {
    use super::*;
    use crate::foundation::NSObjectProtocolInstanceMethods as _;
    use crate::foundation::NSString;

    #[test]
    fn empty_arrays() {
        let array1: Ptr<NSArray<NSObject>> = NSArray::new();
        let array2: Ptr<NSArray<NSObject>> = NSArray::new();
        assert!(array1.is_equal(&array1));
        assert!(array1.is_equal(&array2));
        assert_eq!(array1.count(), 0);
        assert_eq!(array2.count(), 0);
        assert!(array1.first().is_none());
        assert!(array1.last().is_none());
    }

    #[test]
    fn adding() {
        let array1: Ptr<NSArray<NSObject>> = NSArray::new();
        let array2: Ptr<NSArray<NSObject>> = NSArray::new();
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
// NSMutableArray

extern "C" {
    fn choco_Foundation_NSMutableArray_class() -> RawClassPtr;
    fn choco_Foundation_NSMutableArrayInterface_instance_addObject(
        self_: RawObjPtr,
        object: RawObjPtr,
    );
}

pub trait NSMutableArrayInterface: NSArrayInterface {}

trait NSMutableArrayInterfaceInstanceMethods: NSArrayInterfaceInstanceMethods {
    fn add_object<ObjT, ObjOwnership>(&self, obj: &Ptr<ObjT, ObjOwnership>)
    where
        ObjT: IsKindOf<Self::Item>,
        ObjOwnership: Ownership,
    {
        let raw_self = self.as_raw();
        let raw_obj = obj.as_raw();
        unsafe { choco_Foundation_NSMutableArrayInterface_instance_addObject(raw_self, raw_obj) }
    }
}

impl<T, O> NSMutableArrayInterfaceInstanceMethods for Ptr<T, O>
where
    T: NSMutableArrayInterface,
    O: Ownership,
{
}

pub struct NSMutableArray<T: Type> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Type> Type for NSMutableArray<T> {
    const KIND: TypeKind = TypeKind::ObjC;
}

unsafe impl<T: Type> IsKindOf<NSObject> for NSMutableArray<T> {}
unsafe impl<T: Type> IsKindOf<NSArray<T>> for NSMutableArray<T> {}

impl<T: Type> ObjCClass for NSMutableArray<T> {
    fn class() -> RawClassPtr {
        unsafe { choco_Foundation_NSMutableArray_class() }
    }
}

impl<T: Type> NSObjectProtocol for NSMutableArray<T> {}
impl<T: Type> NSObjectInterface for NSMutableArray<T> {}
impl<T: Type> NSArrayInterface for NSMutableArray<T> {
    type Item = T;
}
impl<T: Type> NSMutableArrayInterface for NSMutableArray<T> {}
// impl<T> NSFastEnumerationProtocol<T> for ptr::OwnedPtr<NSMutableArray<T>> where T: ptr::Type {}

#[cfg(test)]
mod mutable_array_tests {
    use super::*;
    use crate::foundation::NSObjectProtocolInstanceMethods as _;
    use crate::foundation::NSString;
    use crate::foundation::NSStringInterface as _;

    #[test]
    fn simple_array() {
        let array: Ptr<NSMutableArray<NSString>> = NSMutableArray::new();
        assert!(array.is_empty());
        assert_eq!(array.count(), 0);
        let value = NSString::new_with_str("abcd");
        array.add_object(&value);
        assert!(!array.is_empty());
        assert_eq!(array.count(), 1);
        assert!(array.object_at(0).is_equal(&value));
    }
}
