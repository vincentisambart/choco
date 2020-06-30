use crate::base::*;
use crate::foundation::*;
use choco_macro::NSObjectProtocol;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVURLAssetPreferPreciseDurationAndTimingKey: RawObjCPtr;
    static AVMediaTypeAudio: RawObjCPtr;
    static AVMediaTypeClosedCaption: RawObjCPtr;
    static AVMediaTypeDepthData: RawObjCPtr;
    static AVMediaTypeMetadata: RawObjCPtr;
    static AVMediaTypeMetadataObject: RawObjCPtr;
    static AVMediaTypeMuxed: RawObjCPtr;
    static AVMediaTypeSubtitle: RawObjCPtr;
    static AVMediaTypeText: RawObjCPtr;
    static AVMediaTypeTimecode: RawObjCPtr;
    static AVMediaTypeVideo: RawObjCPtr;
}

//-------------------------------------------------------------------
// AVAsset

extern "C" {
    fn choco_AVFoundation_AVAsset_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVAssetInterface_instance_tracks(self_: RawObjCPtr)
        -> RawNullableObjCPtr;
}

pub trait AVAssetInterface: NSObjectInterface {
    fn tracks(&self) -> NSArray<AVAssetTrack> {
        let self_raw = self.as_raw();
        let raw_ptr = unsafe { choco_AVFoundation_AVAssetInterface_instance_tracks(self_raw) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[AVAsset tracks] to return a non null pointer");
        unsafe { NSArray::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAsset {
    ptr: OwnedObjCPtr,
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
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVURLAsset {
    ptr: OwnedObjCPtr,
}

impl AVURLAsset {
    pub fn prefer_precise_duration_and_timing_key() -> StaticNSString {
        unsafe { StaticNSString::from_static(AVURLAssetPreferPreciseDurationAndTimingKey) }
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
// AVAssetTrack

extern "C" {
    fn choco_AVFoundation_AVAssetTrack_class() -> NullableObjCClassPtr;
}

pub trait AVAssetTrackInterface: NSObjectInterface {}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAssetTrack {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for AVAssetTrack {}
impl AVAssetTrackInterface for AVAssetTrack {}

impl From<AVAssetTrack> for NSObject {
    fn from(obj: AVAssetTrack) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAssetTrack {}

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
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAssetReader {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for AVAssetReader {}
impl AVAssetReaderInterface for AVAssetReader {}

impl From<AVAssetReader> for NSObject {
    fn from(obj: AVAssetReader) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAssetReader {}

//-------------------------------------------------------------------
// AVAssetReaderTrackOutput

extern "C" {
    fn choco_AVFoundation_AVAssetReaderTrackOutput_class() -> NullableObjCClassPtr;
}

pub trait AVAssetReaderTrackOutputInterface: NSObjectInterface {}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAssetReaderTrackOutput {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for AVAssetReaderTrackOutput {}
impl AVAssetReaderTrackOutputInterface for AVAssetReaderTrackOutput {}

impl From<AVAssetReaderTrackOutput> for NSObject {
    fn from(obj: AVAssetReaderTrackOutput) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAssetReaderTrackOutput {}

//-------------------------------------------------------------------
// AVAssetReaderOutput

extern "C" {
    fn choco_AVFoundation_AVAssetReaderOutput_class() -> NullableObjCClassPtr;
}

pub trait AVAssetReaderOutputInterface: NSObjectInterface {}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAssetReaderOutput {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for AVAssetReaderOutput {}
impl AVAssetReaderOutputInterface for AVAssetReaderOutput {}

impl From<AVAssetReaderOutput> for NSObject {
    fn from(obj: AVAssetReaderOutput) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAssetReaderOutput {}

//-------------------------------------------------------------------
// AVAssetReaderSampleReferenceOutput

extern "C" {
    fn choco_AVFoundation_AVAssetReaderSampleReferenceOutput_class() -> NullableObjCClassPtr;
}

pub trait AVAssetReaderSampleReferenceOutputInterface: AVAssetReaderOutputInterface {}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAssetReaderSampleReferenceOutput {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for AVAssetReaderSampleReferenceOutput {}
impl AVAssetReaderOutputInterface for AVAssetReaderSampleReferenceOutput {}
impl AVAssetReaderSampleReferenceOutputInterface for AVAssetReaderSampleReferenceOutput {}

impl From<AVAssetReaderSampleReferenceOutput> for NSObject {
    fn from(obj: AVAssetReaderSampleReferenceOutput) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAssetReaderSampleReferenceOutput {}
