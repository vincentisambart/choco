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

pub trait NSDictionaryInterface<K: LikeObjCPtr, V: LikeObjCPtr>: NSObjectInterface {
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
pub struct NSDictionary<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> {
    ptr: ObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSObjectInterface for NSDictionary<K, V> {}
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSDictionaryInterface<K, V>
    for NSDictionary<K, V>
{
}
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSFastEnumeration<K>
    for NSDictionary<K, V>
{
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> From<NSDictionary<K, V>> for NSObject {
    fn from(obj: NSDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSCopyingProtocol for NSDictionary<K, V> {
    type Immutable = ImmutableNSDictionary<K, V>;
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSMutableCopyingProtocol
    for NSDictionary<K, V>
{
    type Mutable = NSMutableDictionary<K, V>;
}

//-------------------------------------------------------------------
// ImmutableNSDictionary

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = Foundation, objc_class = NSDictionary)]
pub struct ImmutableNSDictionary<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> {
    ptr: ObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSObjectInterface
    for ImmutableNSDictionary<K, V>
{
}
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSDictionaryInterface<K, V>
    for ImmutableNSDictionary<K, V>
{
}
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSFastEnumeration<K>
    for ImmutableNSDictionary<K, V>
{
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> From<ImmutableNSDictionary<K, V>>
    for NSObject
{
    fn from(obj: ImmutableNSDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

// A NSDictionary known to be immutable can be used as a normal NSDictionary.
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> From<ImmutableNSDictionary<K, V>>
    for NSDictionary<K, V>
{
    fn from(obj: ImmutableNSDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSMutableCopyingProtocol
    for ImmutableNSDictionary<K, V>
{
    type Mutable = NSMutableDictionary<K, V>;
}

// An ImmutableNSString is known to be immutable so can be shared between threads.
unsafe impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> Send
    for ImmutableNSDictionary<K, V>
{
}
unsafe impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> Sync
    for ImmutableNSDictionary<K, V>
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

pub trait NSMutableDictionaryInterface<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr>:
    NSDictionaryInterface<K, V>
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
pub struct NSMutableDictionary<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> {
    ptr: ObjCPtr,
    _marker_k: std::marker::PhantomData<K>,
    _marker_v: std::marker::PhantomData<V>,
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSObjectInterface
    for NSMutableDictionary<K, V>
{
}
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSDictionaryInterface<K, V>
    for NSMutableDictionary<K, V>
{
}
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSMutableDictionaryInterface<K, V>
    for NSMutableDictionary<K, V>
{
}
impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSFastEnumeration<K>
    for NSMutableDictionary<K, V>
{
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> From<NSMutableDictionary<K, V>>
    for NSObject
{
    fn from(obj: NSMutableDictionary<K, V>) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> NSCopyingProtocol
    for NSMutableDictionary<K, V>
{
    type Immutable = ImmutableNSDictionary<K, V>;
}

impl<K: NSObjectProtocol + LikeObjCPtr, V: LikeObjCPtr> From<NSMutableDictionary<K, V>>
    for NSDictionary<K, V>
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
