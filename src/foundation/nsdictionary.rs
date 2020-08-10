use super::*;
use crate::base::ptr;

//-------------------------------------------------------------------
// NSDictionary interface

extern "C" {
    fn choco_Foundation_NSDictionary_class() -> ptr::objc::ClassPtr;
    fn choco_Foundation_NSDictionaryInterface_instance_count(self_: ptr::objc::RawPtr) -> usize;
    fn choco_Foundation_NSDictionaryInterface_instance_objectForKey(
        self_: ptr::objc::RawPtr,
        key: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
}

pub trait NSDictionaryInterface<K, V>: NSObjectInterface
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
    Self: NSCopyingProtocol + NSMutableCopyingProtocol + NSFastEnumerationProtocol<K>,
{
    fn count(&self) -> usize {
        let raw_self = self.as_raw();
        unsafe { choco_Foundation_NSDictionaryInterface_instance_count(raw_self) }
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn get<Key>(&self, key: &Key) -> Option<V>
    where
        Key: IsKindOf<K>,
    {
        let raw_self = self.as_raw();
        let raw_key = key.as_raw();
        unsafe {
            choco_Foundation_NSDictionaryInterface_instance_objectForKey(raw_self, raw_key)
                .into_opt()
                .map(|raw| V::from_owned_ptr_unchecked(raw.consider_owned()))
        }
    }
}

//-------------------------------------------------------------------
// NSDictionary

pub struct NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    ptr: ptr::objc::OwnedPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K, V> ptr::objc::AsRawPtr for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn as_raw(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw()
    }
}

impl<K, V> FromOwnedPtr for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self {
            ptr,
            _marker_k: std::marker::PhantomData,
            _marker_v: std::marker::PhantomData,
        }
    }
}

impl<K, V> NSObjectProtocol for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_Foundation_NSDictionary_class() }
    }
}

impl<K, V> NSObjectInterface for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> NSDictionaryInterface<K, V> for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> NSCopyingProtocol for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Immutable = ImmutableNSDictionary<K, V>;
}

impl<K, V> NSMutableCopyingProtocol for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Mutable = NSMutableDictionary<K, V>;
}

impl<K, V> NSFastEnumerationProtocol<K> for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> ValidObjCGeneric for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

//-------------------------------------------------------------------
// ImmutableNSDictionary

pub struct ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    ptr: ptr::objc::OwnedPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K, V> ptr::objc::AsRawPtr for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn as_raw(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw()
    }
}

impl<K, V> FromOwnedPtr for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self {
            ptr,
            _marker_k: std::marker::PhantomData,
            _marker_v: std::marker::PhantomData,
        }
    }
}

impl<K, V> NSObjectProtocol for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_Foundation_NSDictionary_class() }
    }
}

impl<K, V> NSObjectInterface for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}
impl<K, V> NSDictionaryInterface<K, V> for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}
impl<K, V> NSFastEnumerationProtocol<K> for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> NSCopyingProtocol for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Immutable = Self;
}

impl<K, V> NSMutableCopyingProtocol for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Mutable = NSMutableDictionary<K, V>;
}

// An ImmutableNSString is known to be immutable so can be shared between threads.
unsafe impl<K, V> Send for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}
unsafe impl<K, V> Sync for ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

//-------------------------------------------------------------------
// NSMutableDictionary

extern "C" {
    fn choco_Foundation_NSMutableDictionary_class() -> ptr::objc::ClassPtr;
    fn choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(
        self_: ptr::objc::RawPtr,
        object: ptr::objc::RawPtr,
        key: ptr::objc::RawPtr,
    );
    fn choco_Foundation_NSMutableDictionaryInterface_instance_removeObjectForKey(
        self_: ptr::objc::RawPtr,
        key: ptr::objc::RawPtr,
    );
    fn choco_Foundation_NSMutableDictionaryInterface_instance_removeAllObjects(
        self_: ptr::objc::RawPtr,
    );
}

pub trait NSMutableDictionaryInterface<K, V>: NSDictionaryInterface<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
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
            choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(
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
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    ptr: ptr::objc::OwnedPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K, V> ptr::objc::AsRawPtr for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn as_raw(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw()
    }
}

impl<K, V> FromOwnedPtr for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self {
            ptr,
            _marker_k: std::marker::PhantomData,
            _marker_v: std::marker::PhantomData,
        }
    }
}

impl<K, V> NSObjectProtocol for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_Foundation_NSMutableDictionary_class() }
    }
}

impl<K, V> NSObjectInterface for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> NSDictionaryInterface<K, V> for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> NSMutableDictionaryInterface<K, V> for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> NSCopyingProtocol for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Immutable = ImmutableNSDictionary<K, V>;
}

impl<K, V> NSMutableCopyingProtocol for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    type Mutable = Self;
}

impl<K, V> NSFastEnumerationProtocol<K> for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

impl<K, V> ValidObjCGeneric for NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
}

#[cfg(test)]
mod mutable_dictionary_tests {
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
