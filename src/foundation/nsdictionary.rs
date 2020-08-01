use super::*;
use choco_macro::NSObjectProtocol;

//-------------------------------------------------------------------
// NSDictionary interface

extern "C" {
    fn choco_Foundation_NSDictionary_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSDictionaryInterface_instance_count(self_: RawObjCPtr) -> usize;
    fn choco_Foundation_NSDictionaryInterface_instance_objectForKey(
        self_: RawObjCPtr,
        key: RawObjCPtr,
    ) -> NullableRawObjCPtr;
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
        let raw_ptr = unsafe {
            choco_Foundation_NSDictionaryInterface_instance_objectForKey(raw_self, raw_key)
        };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { V::from_owned_raw_unchecked(raw) })
    }
}

//-------------------------------------------------------------------
// NSDictionary

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation)]
pub struct NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    ptr: ObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
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

impl<K, V> From<NSDictionary<K, V>> for NSObject
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn from(obj: NSDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

//-------------------------------------------------------------------
// ImmutableNSDictionary

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation, objc_class = NSDictionary)]
pub struct ImmutableNSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    ptr: ObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
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

impl<K, V> From<ImmutableNSDictionary<K, V>> for NSObject
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn from(obj: ImmutableNSDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

// A NSDictionary known to be immutable can be used as a normal NSDictionary.
impl<K, V> From<ImmutableNSDictionary<K, V>> for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn from(obj: ImmutableNSDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
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
    fn choco_Foundation_NSMutableDictionary_class() -> NullableObjCClassPtr;
    fn choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(
        self_: RawObjCPtr,
        object: RawObjCPtr,
        key: RawObjCPtr,
    );
    fn choco_Foundation_NSMutableDictionaryInterface_instance_removeObjectForKey(
        self_: RawObjCPtr,
        key: RawObjCPtr,
    );
    fn choco_Foundation_NSMutableDictionaryInterface_instance_removeAllObjects(self_: RawObjCPtr);
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

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation)]
pub struct NSMutableDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    ptr: ObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
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

impl<K, V> From<NSMutableDictionary<K, V>> for NSObject
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn from(obj: NSMutableDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<K, V> From<NSMutableDictionary<K, V>> for NSDictionary<K, V>
where
    K: ValidObjCGeneric + NSCopyingProtocol,
    V: ValidObjCGeneric,
{
    fn from(obj: NSMutableDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
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
