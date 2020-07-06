use crate::base::*;
use crate::block::*;
use crate::foundation::*;
use choco_macro::NSObjectProtocol;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVURLAssetPreferPreciseDurationAndTimingKey: RawObjCPtr;
}

//-------------------------------------------------------------------
// AVAsynchronousKeyValueLoading

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AVKeyValueStatus(isize);

impl AVKeyValueStatus {
    pub const UNKNOWN: Self = Self(0);
    pub const LOADING: Self = Self(1);
    pub const LOADED: Self = Self(2);
    pub const FAILED: Self = Self(3);
    pub const CANCELLED: Self = Self(4);
}

extern "C" {
    fn choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_statusOfValueForKey_error(
        self_: RawObjCPtr,
        key: RawObjCPtr,
        error: *mut RawNullableObjCPtr,
    ) -> AVKeyValueStatus;

    fn choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_loadValuesAsynchronouslyForKeys_completionHandler(
        self_: RawObjCPtr,
        keys: RawObjCPtr,
        completion_handler: *mut crate::block::BlockHeader,
    );
}

pub trait AVAsynchronousKeyValueLoadingProtocol: NSObjectProtocol {
    fn status_of_value_for_key(
        &self,
        key: &impl NSStringInterface,
    ) -> Result<AVKeyValueStatus, NSError> {
        let self_raw = self.as_raw();
        let mut raw_unowned_error = RawNullableObjCPtr::empty();
        let status = unsafe {
            choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_statusOfValueForKey_error(
                self_raw,
                key.as_raw(),
                &mut raw_unowned_error,
            )
        };
        unsafe { make_value_result_unchecked(status, raw_unowned_error) }
    }

    fn load_values_async_for_keys<Key, Keys, CompletionHandler>(
        &self,
        keys: Keys,
        handler: CompletionHandler,
    ) where
        Key: NSStringInterface,
        Keys: NSArrayInterface<Key>,
        CompletionHandler: Fn() + Send + Sync + 'static,
    {
        let self_raw = self.as_raw();
        let block = HeapBlock::new(handler);
        unsafe {
            choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_loadValuesAsynchronouslyForKeys_completionHandler(
                self_raw,
                keys.as_raw(),
                block.block_ref().get()
            )
        }
    }
}

//-------------------------------------------------------------------
// AVAsset

extern "C" {
    fn choco_AVFoundation_AVAsset_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVAssetInterface_instance_tracks(self_: RawObjCPtr)
        -> RawNullableObjCPtr;
    fn choco_AVFoundation_AVAssetInterface_instance_playable(self_: RawObjCPtr) -> BOOL;
}

pub trait AVAssetInterface: NSObjectInterface
where
    Self: AVAsynchronousKeyValueLoadingProtocol,
{
    fn tracks(&self) -> NSArray<AVAssetTrack> {
        let self_raw = self.as_raw();
        let raw_ptr = unsafe { choco_AVFoundation_AVAssetInterface_instance_tracks(self_raw) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[AVAsset tracks] to return a non null pointer");
        unsafe { NSArray::from_owned_raw_unchecked(raw) }
    }

    // Named "playable" and not "is_playable" to be the same as the key to pass to AVAsynchronousKeyValueLoading.
    fn playable(&self) -> bool {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVAssetInterface_instance_playable(self_raw) }.into()
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAsset {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for AVAsset {}
impl AVAsynchronousKeyValueLoadingProtocol for AVAsset {}
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
impl AVAsynchronousKeyValueLoadingProtocol for AVURLAsset {}
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
// AVMediaType

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
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

pub struct StaticAVMediaType {
    raw: StaticNSString,
}

impl From<StaticAVMediaType> for StaticNSString {
    fn from(media_type: StaticAVMediaType) -> Self {
        media_type.raw
    }
}

pub struct AVMediaType {
    raw: NSString,
}

impl From<AVMediaType> for NSString {
    fn from(media_type: AVMediaType) -> Self {
        media_type.raw
    }
}

impl AVMediaType {
    pub fn new(raw: NSString) -> Self {
        AVMediaType { raw }
    }

    pub fn audio() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeAudio) },
        }
    }
    pub fn closed_caption() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeClosedCaption) },
        }
    }
    pub fn depth_data() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeDepthData) },
        }
    }
    pub fn metadata() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeMetadata) },
        }
    }
    pub fn metadata_object() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeMetadataObject) },
        }
    }
    pub fn muxed() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeMuxed) },
        }
    }
    pub fn subtitle() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeSubtitle) },
        }
    }
    pub fn text() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeText) },
        }
    }
    pub fn timecode() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeTimecode) },
        }
    }
    pub fn video() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static(AVMediaTypeVideo) },
        }
    }
}

impl std::cmp::PartialEq<StaticAVMediaType> for AVMediaType {
    fn eq(&self, other: &StaticAVMediaType) -> bool {
        self.raw.is_equal_to_string(&other.raw)
    }
}

impl std::cmp::PartialEq<AVMediaType> for StaticAVMediaType {
    fn eq(&self, other: &AVMediaType) -> bool {
        self.raw.is_equal_to_string(&other.raw)
    }
}

//-------------------------------------------------------------------
// AVAssetTrack

extern "C" {
    fn choco_AVFoundation_AVAssetTrack_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVAssetTrackInterface_instance_mediaType(
        self_: RawObjCPtr,
    ) -> RawNullableObjCPtr;
}

pub trait AVAssetTrackInterface: NSObjectInterface
where
    Self: AVAsynchronousKeyValueLoadingProtocol,
{
    fn media_type(&self) -> AVMediaType {
        let raw_self = self.as_raw();
        let raw_ptr =
            unsafe { choco_AVFoundation_AVAssetTrackInterface_instance_mediaType(raw_self) };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[AVAssetTrack mediaType] to return a non null pointer");
        AVMediaType::new(unsafe { NSString::from_owned_raw_unchecked(raw) })
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = "AVFoundation")]
pub struct AVAssetTrack {
    ptr: OwnedObjCPtr,
}

impl NSObjectInterface for AVAssetTrack {}
impl AVAsynchronousKeyValueLoadingProtocol for AVAssetTrack {}
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
        unsafe { make_object_result_unchecked::<Self>(raw_ptr, raw_unowned_error) }
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
