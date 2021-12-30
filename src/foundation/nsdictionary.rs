use super::*;
use crate::base::{IsKindOf, Ptr, RawClassPtr, RawObjPtr, Retained, Type};

//-------------------------------------------------------------------
// NSDictionary

extern "C" {
    fn choco_Foundation_NSDictionary_class() -> RawClassPtr;
    fn choco_Foundation_NSDictionaryInterface_instance_count(self_: RawObjPtr) -> usize;
    fn choco_Foundation_NSDictionaryInterface_instance_objectForKey(
        self_: RawObjPtr,
        key: RawObjPtr,
    ) -> Option<RawObjPtr>;
}

pub trait NSDictionaryInterface: NSObjectInterface
// Self: NSCopyingProtocol + NSMutableCopyingProtocol + NSFastEnumerationProtocol<T>,
// Self: NSFastEnumerationProtocol<T>,
{
    type Key: Type; // + NSCopyingProtocol,
    type Value: Type;
}

pub trait NSDictionaryInterfaceInstanceMethods: AsRaw {
    type Key: Type; // + NSCopyingProtocol,
    type Value: Type;

    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDictionaryInterface_instance_count(raw_self) }
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn get<PassedKey, PassedKeyOwnership>(
        &self,
        key: &Ptr<PassedKey, PassedKeyOwnership>,
    ) -> Option<Ptr<Self::Value, Retained>>
    where
        PassedKey: IsKindOf<Self::Key>,
        PassedKeyOwnership: Ownership,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        unsafe {
            choco_Foundation_NSDictionaryInterface_instance_objectForKey(raw_self, raw_key)
                .map(|raw| Ptr::from_raw_unchecked(raw))
        }
    }
}

impl<T, O> NSDictionaryInterfaceInstanceMethods for Ptr<T, O>
where
    T: NSDictionaryInterface,
    O: Ownership,
{
    type Key = T::Key;
    type Value = T::Value;
}

pub struct NSDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K, V> Type for NSDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    const KIND: TypeKind = TypeKind::ObjC;
}

impl<K, V> ObjCClass for NSDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    fn class() -> RawClassPtr {
        unsafe { choco_Foundation_NSDictionary_class() }
    }
}

unsafe impl<K, V> IsKindOf<NSObject> for NSDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

impl<K, V> NSObjectProtocol for NSDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

impl<K, V> NSObjectInterface for NSDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

impl<K, V> NSDictionaryInterface for NSDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    type Key = K;
    type Value = V;
}

// impl<K, V> NSCopyingProtocol for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Immutable = ImmutableNSDictionary<K, V>;
// }

// impl<K, V> NSMutableCopyingProtocol for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Mutable = NSMutableDictionary<K, V>;
// }

// impl<K, V> NSFastEnumerationProtocol<K> for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

// impl<K, V> ValidObjCGeneric for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

//-------------------------------------------------------------------
// NSMutableDictionary

extern "C" {
    fn choco_Foundation_NSMutableDictionary_class() -> RawClassPtr;
    fn choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(
        self_: RawObjPtr,
        object: RawObjPtr,
        key: RawObjPtr,
    );
    fn choco_Foundation_NSMutableDictionaryInterface_instance_removeObjectForKey(
        self_: RawObjPtr,
        key: RawObjPtr,
    );
    fn choco_Foundation_NSMutableDictionaryInterface_instance_removeAllObjects(self_: RawObjPtr);
}

pub trait NSMutableDictionaryInterface: NSDictionaryInterface {}

pub trait NSMutableDictionaryInterfaceInstanceMethods:
    NSDictionaryInterfaceInstanceMethods
{
    fn set<PassedKey, PassedValue, PassedKeyOwnership, PassedValueOwnership>(
        &self,
        key: &Ptr<PassedKey, PassedKeyOwnership>,
        value: &Ptr<PassedValue, PassedValueOwnership>,
    ) where
        PassedKey: IsKindOf<Self::Key>,
        PassedValue: IsKindOf<Self::Value>,
        PassedKeyOwnership: Ownership,
        PassedValueOwnership: Ownership,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        let raw_value = value.as_raw();
        unsafe {
            choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(
                raw_self, raw_value, raw_key,
            )
        }
    }

    fn remove<PassedKey, PassedKeyOwnership>(&self, key: &Ptr<PassedKey, PassedKeyOwnership>)
    where
        PassedKey: IsKindOf<Self::Key>,
        PassedKeyOwnership: Ownership,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        unsafe {
            choco_Foundation_NSMutableDictionaryInterface_instance_removeObjectForKey(
                raw_self, raw_key,
            )
        }
    }

    fn remove_all(&self) {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSMutableDictionaryInterface_instance_removeAllObjects(raw_self) }
    }
}

pub struct NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K, V> Type for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    const KIND: TypeKind = TypeKind::ObjC;
}

impl<K, V> ObjCClass for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    fn class() -> RawClassPtr {
        unsafe { choco_Foundation_NSMutableDictionary_class() }
    }
}

unsafe impl<K, V> IsKindOf<NSObject> for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

// TODO: Should allow passing a dictionary with NSString values to something
// accepting NSObject value.
unsafe impl<K, V> IsKindOf<NSDictionary<K, V>> for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

impl<K, V> NSObjectProtocol for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

impl<K, V> NSObjectInterface for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

impl<K, V> NSDictionaryInterface for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
    type Key = K;
    type Value = V;
}

impl<K, V> NSMutableDictionaryInterface for NSMutableDictionary<K, V>
where
    K: Type, // + NSCopyingProtocol,
    V: Type,
{
}

impl<T, O> NSMutableDictionaryInterfaceInstanceMethods for Ptr<T, O>
where
    T: NSDictionaryInterface,
    O: Ownership,
{
}

// impl<K, V> NSCopyingProtocol for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Immutable = ImmutableNSDictionary<K, V>;
// }

// impl<K, V> NSMutableCopyingProtocol for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Mutable = Self;
// }

// impl<K, V> NSFastEnumerationProtocol<K> for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

// impl<K, V> ValidObjCGeneric for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

#[cfg(test)]
mod mutable_dictionary_tests {
    use super::*;

    #[test]
    fn simple_dictionary() {
        let dic = NSMutableDictionary::<NSString, NSDate>::new();
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
