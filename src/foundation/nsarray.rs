use super::*;
use choco_macro::NSObjectProtocol;

//-------------------------------------------------------------------
// NSArray interface

extern "C" {
    fn choco_Foundation_NSArray_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSArrayInterface_instance_count(self_: RawObjCPtr) -> usize;
    fn choco_Foundation_NSArrayInterface_instance_firstObject(
        self_: RawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_Foundation_NSArrayInterface_instance_lastObject(
        self_: RawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_Foundation_NSArrayInterface_instance_objectAtIndex(
        self_: RawObjCPtr,
        index: usize,
    ) -> NullableRawObjCPtr;
    fn choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(
        self_: RawObjCPtr,
        obj: RawObjCPtr,
    ) -> NullableRawObjCPtr;
}

pub trait NSArrayInterface<T: LikeObjCPtr>: NSObjectInterface {
    fn first(&self) -> Option<T> {
        let raw_self = self.as_raw();
        let raw_ptr = unsafe { choco_Foundation_NSArrayInterface_instance_firstObject(raw_self) };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { T::from_owned_raw_unchecked(raw) })
    }
    fn last(&self) -> Option<T> {
        let raw_self = self.as_raw();
        let raw_ptr = unsafe { choco_Foundation_NSArrayInterface_instance_lastObject(raw_self) };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { T::from_owned_raw_unchecked(raw) })
    }

    fn object_at(&self, index: usize) -> T {
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

    #[must_use]
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

//-------------------------------------------------------------------
// NSArray

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation)]
pub struct NSArray<T: LikeObjCPtr> {
    ptr: ObjCPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: LikeObjCPtr> NSObjectInterface for NSArray<T> {}
impl<T: LikeObjCPtr> NSArrayInterface<T> for NSArray<T> {}
impl<T: LikeObjCPtr> NSFastEnumeration<T> for NSArray<T> {}

impl<T: LikeObjCPtr> From<NSArray<T>> for NSObject {
    fn from(obj: NSArray<T>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<T: LikeObjCPtr> NSCopyingProtocol for NSArray<T> {
    type Immutable = ImmutableNSArray<T>;
}

impl<T: LikeObjCPtr> NSMutableCopyingProtocol for NSArray<T> {
    type Mutable = NSMutableArray<T>;
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
// ImmutableNSArray

/// Version of NSArray we are statically sure to be immutable.
#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation, objc_class = NSArray)]
pub struct ImmutableNSArray<T: LikeObjCPtr> {
    ptr: ObjCPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: LikeObjCPtr> NSObjectInterface for ImmutableNSArray<T> {}
impl<T: LikeObjCPtr> NSArrayInterface<T> for ImmutableNSArray<T> {}
impl<T: LikeObjCPtr> NSFastEnumeration<T> for ImmutableNSArray<T> {}

impl<T: LikeObjCPtr> From<ImmutableNSArray<T>> for NSObject {
    fn from(obj: ImmutableNSArray<T>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

// A NSArray known to be immutable can be used as a normal NSArray.
impl<T: LikeObjCPtr> From<ImmutableNSArray<T>> for NSArray<T> {
    fn from(obj: ImmutableNSArray<T>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<T: LikeObjCPtr> NSMutableCopyingProtocol for ImmutableNSArray<T> {
    type Mutable = NSMutableArray<T>;
}

// An ImmutableNSArray is known to be immutable so can be shared between threads.
unsafe impl<T: LikeObjCPtr> Send for ImmutableNSArray<T> {}
unsafe impl<T: LikeObjCPtr> Sync for ImmutableNSArray<T> {}

//-------------------------------------------------------------------
// NSMutableArray interface

extern "C" {
    fn choco_Foundation_NSMutableArray_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSMutableArrayInterface_instance_addObject(
        self_: RawObjCPtr,
        object: RawObjCPtr,
    );
}

pub trait NSMutableArrayInterface<T: LikeObjCPtr>: NSArrayInterface<T> {
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

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation)]
pub struct NSMutableArray<T: LikeObjCPtr> {
    ptr: ObjCPtr,
    _marker: std::marker::PhantomData<T>,
}

impl<T: LikeObjCPtr> NSObjectInterface for NSMutableArray<T> {}
impl<T: LikeObjCPtr> NSArrayInterface<T> for NSMutableArray<T> {}
impl<T: LikeObjCPtr> NSMutableArrayInterface<T> for NSMutableArray<T> {}
impl<T: LikeObjCPtr> NSFastEnumeration<T> for NSMutableArray<T> {}

impl<T: LikeObjCPtr> From<NSMutableArray<T>> for NSObject {
    fn from(obj: NSMutableArray<T>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<T: LikeObjCPtr> From<NSMutableArray<T>> for NSArray<T> {
    fn from(obj: NSMutableArray<T>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<T: LikeObjCPtr> NSCopyingProtocol for NSMutableArray<T> {
    type Immutable = ImmutableNSArray<T>;
}

impl<T: LikeObjCPtr> NSMutableCopyingProtocol for NSMutableArray<T> {
    type Mutable = NSMutableArray<T>;
}

#[cfg(test)]
mod mutable_array_tests {
    use super::*;

    #[test]
    fn simple_array() {
        let array: NSMutableArray<NSDate> = NSMutableArray::new();
        assert!(array.is_empty());
        assert_eq!(array.count(), 0);
        let value = NSString::new_with_str("abcd");
        array.add_object(&value);
        assert!(!array.is_empty());
        assert_eq!(array.count(), 1);
        assert!(array.object_at(0).is_equal(&value));
    }
}
