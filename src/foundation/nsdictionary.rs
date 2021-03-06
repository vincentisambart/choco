use super::*;
use crate::base::{objc, ptr};
use objc::{IsKindOf, NSObjectInterface, NSObjectInterfaceClassMethods, NSObjectProtocol};
use ptr::{AsRaw, ObjCClass};

//-------------------------------------------------------------------
// NSDictionary

extern "C" {
    fn choco_Foundation_NSDictionary_class() -> ptr::ClassPtr;
    fn choco_Foundation_NSDictionaryInterface_instance_count(self_: ptr::RawPtr) -> usize;
    fn choco_Foundation_NSDictionaryInterface_instance_objectForKey(
        self_: ptr::RawPtr,
        key: ptr::RawPtr,
    ) -> Option<ptr::RawPtr>;
}

pub trait NSDictionaryInterface<K, V>: NSObjectInterface
where
    K: ptr::Type, // + NSCopyingProtocol,
    V: ptr::Type,
{
    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDictionaryInterface_instance_count(raw_self) }
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn get<Ptr, Key>(&self, key: &Ptr) -> Option<ptr::OwnedPtr<V>>
    where
        Key: IsKindOf<K>,
        Ptr: ptr::PtrHolder<Key>,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        unsafe {
            choco_Foundation_NSDictionaryInterface_instance_objectForKey(raw_self, raw_key)
                .map(|raw| ptr::OwnedPtr::from_owned_raw_unchecked(raw))
        }
    }
}

pub struct NSDictionary<K, V>
where
    K: ptr::Type, // + NSCopyingProtocol,
    V: ptr::Type,
{
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

// impl<K, V> ptr::FromOwned for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
//         Self {
//             ptr,
//             _marker_k: std::marker::PhantomData,
//             _marker_v: std::marker::PhantomData,
//         }
//     }
// }

// impl<K, V> NSObjectProtocol for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Owned = Self;

//     fn class() -> ptr::ClassPtr {
//         unsafe { choco_Foundation_NSDictionary_class() }
//     }
// }

// impl<K, V> NSObjectInterface for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

// impl<K, V> NSDictionaryInterface<K, V> for NSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

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

// //-------------------------------------------------------------------
// // ImmutableNSDictionary

// pub struct ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     ptr: ptr::objc::OwnedPtr,
//     _marker_k: std::marker::PhantomData<K>,
//     _marker_v: std::marker::PhantomData<V>,
// }

// impl<K, V> ptr::FromOwned for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
//         Self {
//             ptr,
//             _marker_k: std::marker::PhantomData,
//             _marker_v: std::marker::PhantomData,
//         }
//     }
// }

// impl<K, V> NSObjectProtocol for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Owned = Self;

//     fn class() -> ptr::ClassPtr {
//         unsafe { choco_Foundation_NSDictionary_class() }
//     }
// }

// impl<K, V> NSObjectInterface for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }
// impl<K, V> NSDictionaryInterface<K, V> for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }
// impl<K, V> NSFastEnumerationProtocol<K> for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

// impl<K, V> NSCopyingProtocol for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Immutable = Self;
// }

// impl<K, V> NSMutableCopyingProtocol for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Mutable = NSMutableDictionary<K, V>;
// }

// // An ImmutableNSString is known to be immutable so can be shared between threads.
// unsafe impl<K, V> Send for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }
// unsafe impl<K, V> Sync for ImmutableNSDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

// //-------------------------------------------------------------------
// // NSMutableDictionary

// extern "C" {
//     fn choco_Foundation_NSMutableDictionary_class() -> ptr::ClassPtr;
//     fn choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(
//         self_: ptr::RawPtr,
//         object: ptr::RawPtr,
//         key: ptr::RawPtr,
//     );
//     fn choco_Foundation_NSMutableDictionaryInterface_instance_removeObjectForKey(
//         self_: ptr::RawPtr,
//         key: ptr::RawPtr,
//     );
//     fn choco_Foundation_NSMutableDictionaryInterface_instance_removeAllObjects(self_: ptr::RawPtr);
// }

// pub trait NSMutableDictionaryInterface<K, V>: NSDictionaryInterface<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     fn set<Key, Value>(&self, key: &Key, value: &Value)
//     where
//         Key: IsKindOf<K>,
//         Value: IsKindOf<V>,
//     {
//         let raw_self = self.as_raw();
//         let raw_key = key.as_raw();
//         let raw_value = value.as_raw();
//         unsafe {
//             choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(
//                 raw_self, raw_value, raw_key,
//             )
//         }
//     }

//     fn remove<Key>(&self, key: &Key)
//     where
//         Key: IsKindOf<K>,
//     {
//         let raw_self = self.as_raw();
//         let raw_key = key.as_raw();
//         unsafe {
//             choco_Foundation_NSMutableDictionaryInterface_instance_removeObjectForKey(
//                 raw_self, raw_key,
//             )
//         }
//     }

//     fn remove_all(&self) {
//         let raw_self = self.as_raw();
//         unsafe { choco_Foundation_NSMutableDictionaryInterface_instance_removeAllObjects(raw_self) }
//     }
// }

// pub struct NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     ptr: ptr::objc::OwnedPtr,
//     _marker_k: std::marker::PhantomData<K>,
//     _marker_v: std::marker::PhantomData<V>,
// }

// impl<K, V> ptr::FromOwned for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
//         Self {
//             ptr,
//             _marker_k: std::marker::PhantomData,
//             _marker_v: std::marker::PhantomData,
//         }
//     }
// }

// impl<K, V> NSObjectProtocol for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
//     type Owned = Self;

//     fn class() -> ptr::ClassPtr {
//         unsafe { choco_Foundation_NSMutableDictionary_class() }
//     }
// }

// impl<K, V> NSObjectInterface for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

// impl<K, V> NSDictionaryInterface<K, V> for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

// impl<K, V> NSMutableDictionaryInterface<K, V> for NSMutableDictionary<K, V>
// where
//     K: ValidObjCGeneric + NSCopyingProtocol,
//     V: ValidObjCGeneric,
// {
// }

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

// #[cfg(test)]
// mod mutable_dictionary_tests {
//     use super::*;

//     #[test]
//     fn simple_dictionary() {
//         let dic: NSMutableDictionary<NSString, NSDate> = NSMutableDictionary::new();
//         assert!(dic.is_empty());
//         assert_eq!(dic.count(), 0);
//         let date = NSDate::new();
//         let key = NSString::new_with_str("abcd");
//         dic.set(&key, &date);
//         assert_eq!(dic.count(), 1);
//         let got = dic.get(&key).unwrap();
//         assert!(got.is_equal(&date));
//     }
// }
