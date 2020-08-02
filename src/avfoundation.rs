use crate::base::block::*;
use crate::base::objc::*;
use crate::core_media::*;
use crate::foundation::*;
use choco_macro::NSObjectProtocol;

pub(crate) mod prelude {
    pub use super::AVAssetInterface;
    pub use super::AVAssetReaderInterface;
    pub use super::AVAssetReaderOutputInterface;
    pub use super::AVAssetReaderSampleReferenceOutputInterface;
    pub use super::AVAssetReaderTrackOutputInterface;
    pub use super::AVAssetTrackInterface;
    pub use super::AVAsynchronousKeyValueLoadingProtocol;
    pub use super::AVAudioPlayerInterface;
    pub use super::AVPlayerInterface;
    pub use super::AVPlayerItemInterface;
    pub use super::AVURLAssetInterface;
}

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
        error: *mut NullableRawObjCPtr,
    ) -> AVKeyValueStatus;

    fn choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_loadValuesAsynchronouslyForKeys_completionHandler(
        self_: RawObjCPtr,
        keys: RawObjCPtr,
        completion_handler: *mut crate::base::block::BlockHeader,
    );
}

pub trait AVAsynchronousKeyValueLoadingProtocol: NSObjectProtocol {
    fn status_of_value_for_key(
        &self,
        key: &impl NSStringInterface,
    ) -> Result<AVKeyValueStatus, NSError> {
        let self_raw = self.as_raw();
        let mut raw_unowned_error = NullableRawObjCPtr::empty();
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
        Key: NSStringInterface + ValidObjCGeneric,
        Keys: NSArrayInterface<Key>,
        CompletionHandler: Fn() + Send + 'static,
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
        -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVAssetInterface_instance_playable(self_: RawObjCPtr) -> BOOL;
}

pub trait AVAssetInterface: NSObjectInterface
where
    Self: NSCopyingProtocol + AVAsynchronousKeyValueLoadingProtocol,
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
#[choco(framework = AVFoundation)]
pub struct AVAsset {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVAsset {}
impl AVAsynchronousKeyValueLoadingProtocol for AVAsset {}
impl AVAssetInterface for AVAsset {}
impl NSCopyingProtocol for AVAsset {
    type Immutable = Self;
}
impl ValidObjCGeneric for AVAsset {}

impl From<AVAsset> for NSObject {
    fn from(obj: AVAsset) -> Self {
        unsafe { NSObject::from_owned_unchecked(obj.ptr) }
    }
}
impl IsKindOf<NSObject> for AVAsset {}

// AVAsset should be mostly fine to use from multiple threads at the same time.
unsafe impl Send for AVAsset {}
unsafe impl Sync for AVAsset {}

//-------------------------------------------------------------------
// AVURLAsset

extern "C" {
    fn choco_AVFoundation_AVURLAsset_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVURLAssetInterface_class_newWithURL_options(
        class: ObjCClassPtr,
        url: RawObjCPtr,
        options: RawObjCPtr,
    ) -> NullableRawObjCPtr;
}

pub trait AVURLAssetInterface: AVAssetInterface {
    fn new_with_url_options<Options, K, V>(
        url: &impl NSURLInterface,
        options: &Options,
    ) -> Self::Owned
    where
        K: ValidObjCGeneric + NSStringInterface,
        V: ValidObjCGeneric,
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
        unsafe { Self::Owned::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = AVFoundation)]
pub struct AVURLAsset {
    ptr: ObjCPtr,
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
impl NSCopyingProtocol for AVURLAsset {
    type Immutable = Self;
}
impl ValidObjCGeneric for AVURLAsset {}

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

// AVAsset should be mostly fine to use from multiple threads at the same time.
unsafe impl Send for AVURLAsset {}
unsafe impl Sync for AVURLAsset {}

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
    ) -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVAssetTrackInterface_instance_formatDescriptions(
        self_: RawObjCPtr,
    ) -> NullableRawObjCPtr;
}

pub trait AVAssetTrackInterface: NSObjectInterface
where
    Self: NSCopyingProtocol + AVAsynchronousKeyValueLoadingProtocol,
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

    fn format_descriptions(&self) -> NSArray<CMFormatDescription> {
        let raw_self = self.as_raw();
        let raw_ptr = unsafe {
            choco_AVFoundation_AVAssetTrackInterface_instance_formatDescriptions(raw_self)
        };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[AVAssetTrack formatDescriptions] to return a non null pointer");
        unsafe { NSArray::from_owned_raw_unchecked(raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = AVFoundation)]
pub struct AVAssetTrack {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVAssetTrack {}
impl AVAsynchronousKeyValueLoadingProtocol for AVAssetTrack {}
impl AVAssetTrackInterface for AVAssetTrack {}
impl NSCopyingProtocol for AVAssetTrack {
    type Immutable = Self;
}
impl ValidObjCGeneric for AVAssetTrack {}

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
        error: *mut NullableRawObjCPtr,
    ) -> NullableRawObjCPtr;
}

pub trait AVAssetReaderInterface: NSObjectInterface {
    fn new_with_asset(asset: &impl AVAssetInterface) -> Result<Self::Owned, NSError> {
        let mut raw_unowned_error = NullableRawObjCPtr::empty();
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
#[choco(framework = AVFoundation)]
pub struct AVAssetReader {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVAssetReader {}
impl AVAssetReaderInterface for AVAssetReader {}
impl ValidObjCGeneric for AVAssetReader {}

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
#[choco(framework = AVFoundation)]
pub struct AVAssetReaderTrackOutput {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVAssetReaderTrackOutput {}
impl AVAssetReaderTrackOutputInterface for AVAssetReaderTrackOutput {}
impl ValidObjCGeneric for AVAssetReaderTrackOutput {}

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
#[choco(framework = AVFoundation)]
pub struct AVAssetReaderOutput {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVAssetReaderOutput {}
impl AVAssetReaderOutputInterface for AVAssetReaderOutput {}
impl ValidObjCGeneric for AVAssetReaderOutput {}

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
#[choco(framework = AVFoundation)]
pub struct AVAssetReaderSampleReferenceOutput {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVAssetReaderSampleReferenceOutput {}
impl AVAssetReaderOutputInterface for AVAssetReaderSampleReferenceOutput {}
impl AVAssetReaderSampleReferenceOutputInterface for AVAssetReaderSampleReferenceOutput {}
impl ValidObjCGeneric for AVAssetReaderSampleReferenceOutput {}

impl From<AVAssetReaderSampleReferenceOutput> for NSObject {
    fn from(obj: AVAssetReaderSampleReferenceOutput) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl From<AVAssetReaderSampleReferenceOutput> for AVAssetReaderOutput {
    fn from(obj: AVAssetReaderSampleReferenceOutput) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAssetReaderSampleReferenceOutput {}
impl IsKindOf<AVAssetReaderOutput> for AVAssetReaderSampleReferenceOutput {}

//-------------------------------------------------------------------
// AVPlayerItem

extern "C" {
    fn choco_AVFoundation_AVPlayerItem_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVPlayerItemInterface_class_newWithURL(
        class: ObjCClassPtr,
        url: RawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVPlayerItemInterface_class_newWithAsset(
        class: ObjCClassPtr,
        asset: RawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVPlayerItemInterface_instance_error(
        self_: RawObjCPtr,
    ) -> NullableRawObjCPtr;
}

pub trait AVPlayerItemInterface: NSObjectInterface
where
    Self: NSCopyingProtocol,
{
    fn new_with_url(url: &impl NSURLInterface) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_AVFoundation_AVPlayerItemInterface_class_newWithURL(Self::class(), url.as_raw())
        };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[[<class> alloc] initWithURL:] to return a non null pointer");
        unsafe { Self::Owned::from_owned_raw_unchecked(raw) }
    }

    fn new_with_asset(asset: &impl AVAssetInterface) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_AVFoundation_AVPlayerItemInterface_class_newWithAsset(
                Self::class(),
                asset.as_raw(),
            )
        };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[[<class> alloc] initWithAsset:] to return a non null pointer");
        unsafe { Self::Owned::from_owned_raw_unchecked(raw) }
    }

    fn error(&self) -> Option<NSError> {
        let self_raw = self.as_raw();
        let raw_ptr = unsafe { choco_AVFoundation_AVPlayerItemInterface_instance_error(self_raw) };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { NSError::from_owned_raw_unchecked(raw) })
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = AVFoundation)]
pub struct AVPlayerItem {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVPlayerItem {}
impl AVPlayerItemInterface for AVPlayerItem {}
impl NSCopyingProtocol for AVPlayerItem {
    type Immutable = Self;
}
impl ValidObjCGeneric for AVPlayerItem {}

impl From<AVPlayerItem> for NSObject {
    fn from(obj: AVPlayerItem) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVPlayerItem {}

//-------------------------------------------------------------------
// AVPlayer

extern "C" {
    fn choco_AVFoundation_AVPlayer_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVPlayerInterface_class_newWithURL(
        class: ObjCClassPtr,
        url: RawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVPlayerInterface_class_newWithPlayerItem(
        class: ObjCClassPtr,
        item: RawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVPlayerInterface_instance_play(self_: RawObjCPtr);
    fn choco_AVFoundation_AVPlayerInterface_instance_pause(self_: RawObjCPtr);
    fn choco_AVFoundation_AVPlayerInterface_instance_rate(self_: RawObjCPtr) -> f32;
    fn choco_AVFoundation_AVPlayerInterface_instance_currentItem(
        self_: RawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVPlayerInterface_instance_error(self_: RawObjCPtr)
        -> NullableRawObjCPtr;
}

pub trait AVPlayerInterface: NSObjectInterface {
    fn new_with_url(url: &impl NSURLInterface) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_AVFoundation_AVPlayerInterface_class_newWithURL(Self::class(), url.as_raw())
        };
        let raw = raw_ptr
            .into_opt()
            .expect("expecting -[[<class> alloc] initWithURL:] to return a non null pointer");
        unsafe { Self::Owned::from_owned_raw_unchecked(raw) }
    }

    fn new_with_player_item(url: &impl AVPlayerItemInterface) -> Self::Owned {
        let raw_ptr = unsafe {
            choco_AVFoundation_AVPlayerInterface_class_newWithPlayerItem(
                Self::class(),
                url.as_raw(),
            )
        };
        let raw = raw_ptr.into_opt().expect(
            "expecting -[[<class> alloc] initWithPlayerItem:] to return a non null pointer",
        );
        unsafe { Self::Owned::from_owned_raw_unchecked(raw) }
    }

    fn error(&self) -> Option<NSError> {
        let self_raw = self.as_raw();
        let raw_ptr = unsafe { choco_AVFoundation_AVPlayerInterface_instance_error(self_raw) };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { NSError::from_owned_raw_unchecked(raw) })
    }

    fn current_item(&self) -> Option<AVPlayerItem> {
        let self_raw = self.as_raw();
        let raw_ptr =
            unsafe { choco_AVFoundation_AVPlayerInterface_instance_currentItem(self_raw) };
        raw_ptr
            .into_opt()
            .map(|raw| unsafe { AVPlayerItem::from_owned_raw_unchecked(raw) })
    }

    fn play(&self) {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVPlayerInterface_instance_play(self_raw) }
    }

    fn pause(&self) {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVPlayerInterface_instance_pause(self_raw) }
    }

    fn rate(&self) -> f32 {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVPlayerInterface_instance_rate(self_raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = AVFoundation)]
pub struct AVPlayer {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVPlayer {}
impl AVAssetReaderOutputInterface for AVPlayer {}
impl AVPlayerInterface for AVPlayer {}
impl ValidObjCGeneric for AVPlayer {}

impl From<AVPlayer> for NSObject {
    fn from(obj: AVPlayer) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVPlayer {}

//-------------------------------------------------------------------
// AVAudioPlayer

extern "C" {
    fn choco_AVFoundation_AVAudioPlayer_class() -> NullableObjCClassPtr;
    fn choco_AVFoundation_AVAudioPlayerInterface_class_newWithContentsOfURL_error(
        class: ObjCClassPtr,
        url: RawObjCPtr,
        error: *mut NullableRawObjCPtr,
    ) -> NullableRawObjCPtr;
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_play(self_: RawObjCPtr) -> BOOL;
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_pause(self_: RawObjCPtr);
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_stop(self_: RawObjCPtr);
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_rate(self_: RawObjCPtr) -> f32;
}

pub trait AVAudioPlayerInterface: NSObjectInterface {
    fn new_with_contents_of_url(url: &impl NSURLInterface) -> Result<Self::Owned, NSError> {
        let mut raw_unowned_error = NullableRawObjCPtr::empty();
        let raw_ptr = unsafe {
            choco_AVFoundation_AVAudioPlayerInterface_class_newWithContentsOfURL_error(
                Self::class(),
                url.as_raw(),
                &mut raw_unowned_error,
            )
        };
        unsafe { make_object_result_unchecked::<Self>(raw_ptr, raw_unowned_error) }
    }

    fn play(&self) -> bool {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_play(self_raw) }.into()
    }

    fn pause(&self) {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_pause(self_raw) }
    }

    fn stop(&self) {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_stop(self_raw) }
    }

    fn rate(&self) -> f32 {
        let self_raw = self.as_raw();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_rate(self_raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, NSObjectProtocol)]
#[choco(framework = AVFoundation)]
pub struct AVAudioPlayer {
    ptr: ObjCPtr,
}

impl NSObjectInterface for AVAudioPlayer {}
impl AVAssetReaderOutputInterface for AVAudioPlayer {}
impl AVAudioPlayerInterface for AVAudioPlayer {}
impl ValidObjCGeneric for AVAudioPlayer {}

impl From<AVAudioPlayer> for NSObject {
    fn from(obj: AVAudioPlayer) -> Self {
        unsafe { Self::from_owned_unchecked(obj.ptr) }
    }
}

impl IsKindOf<NSObject> for AVAudioPlayer {}
