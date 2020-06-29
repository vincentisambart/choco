use crate::base::*;
use crate::foundation::*;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVURLAssetPreferPreciseDurationAndTimingKey: RawObjCPtr;
}

//-------------------------------------------------------------------
// AVAsset

extern "C" {
    fn choco_AVFoundation_AVAsset_class() -> NullableObjCClassPtr;
}

pub trait AVAssetInterface: NSObjectInterface {}

#[repr(transparent)]
#[derive(Clone)]
pub struct AVAsset {
    ptr: OwnedObjCPtr,
}

impl NSObjectProtocol for AVAsset {
    type Owned = Self;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self { ptr }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw()
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_AVFoundation_AVAsset_class() }
            .into_opt()
            .expect("expecting +[AVAsset class] to return a non null pointer")
    }
}

impl NSObjectInterface for AVAsset {}
impl AVAssetInterface for AVAsset {}

impl From<AVAsset> for NSObject {
    fn from(obj: AVAsset) -> Self {
        unsafe { NSObject::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSObject> for AVAsset {}

//-------------------------------------------------------------------
// AVURLAsset

extern "C" {
    fn choco_AVFoundation_AVURLAsset_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVURLAssetInterface_class_newWithURL_options(
        class: ObjCClassPtr,
        url: RawObjCPtr,
        options: RawObjCPtr,
    ) -> RawNullableObjCPtr;
}

pub trait AVURLAssetInterface: AVAssetInterface {
    fn new_with_url_options<Options, K, V>(
        url: &impl NSURLInterface,
        options: &Options,
    ) -> Self::Owned
    where
        K: NSStringInterface,
        V: NSObjectProtocol,
        Options: NSDictionaryInterface<K, V>,
    {
        let raw_ptr = unsafe {
            choco_AVFoundation_AVURLAssetInterface_class_newWithURL_options(
                Self::class(),
                url.as_raw(),
                options.as_raw(),
            )
        };
        let raw = raw_ptr.into_opt().expect(
            "expecting -[[<class> alloc] initWithURL:options:] to return a non null pointer",
        );
        unsafe { Self::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AVURLAsset {
    ptr: OwnedObjCPtr,
}

impl AVURLAsset {
    pub fn prefer_precise_duration_and_timing_key() -> StaticNSString {
        unsafe { StaticNSString::from_static(AVURLAssetPreferPreciseDurationAndTimingKey) }
    }
}

impl NSObjectProtocol for AVURLAsset {
    type Owned = Self;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self { ptr }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw()
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_AVFoundation_AVURLAsset_class() }
            .into_opt()
            .expect("expecting +[AVURLAsset class] to return a non null pointer")
    }
}

impl NSObjectInterface for AVURLAsset {}
impl AVAssetInterface for AVURLAsset {}
impl AVURLAssetInterface for AVURLAsset {}

impl From<AVURLAsset> for NSObject {
    fn from(obj: AVURLAsset) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl From<AVURLAsset> for AVAsset {
    fn from(obj: AVURLAsset) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVURLAsset {}
impl IsKindOf<AVAsset> for AVURLAsset {}

//-------------------------------------------------------------------
// AVAssetReader

extern "C" {
    fn choco_AVFoundation_AVAssetReader_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVAssetReaderInterface_class_newWithAsset_error(
        class: ObjCClassPtr,
        asset: RawObjCPtr,
        error: *mut RawNullableObjCPtr,
    ) -> RawNullableObjCPtr;
}

pub trait AVAssetReaderInterface: NSObjectInterface {
    fn new_with_asset(asset: &impl AVAssetInterface) -> Result<Self::Owned, NSError> {
        let mut raw_unowned_error = RawNullableObjCPtr::empty();
        let raw_ptr = unsafe {
            choco_AVFoundation_AVAssetReaderInterface_class_newWithAsset_error(
                Self::class(),
                asset.as_raw(),
                &mut raw_unowned_error,
            )
        };
        unsafe { make_result_unchecked::<Self>(raw_ptr, raw_unowned_error) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AVAssetReader {
    ptr: OwnedObjCPtr,
}

impl NSObjectProtocol for AVAssetReader {
    type Owned = Self;

    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self::Owned {
        Self { ptr }
    }

    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw()
    }

    fn class() -> ObjCClassPtr {
        unsafe { choco_AVFoundation_AVAssetReader_class() }
            .into_opt()
            .expect("expecting +[AVAssetReader class] to return a non null pointer")
    }
}

impl NSObjectInterface for AVAssetReader {}
impl AVAssetReaderInterface for AVAssetReader {}

impl From<AVAssetReader> for NSObject {
    fn from(obj: AVAssetReader) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAssetReader {}
