use crate::base::block::*;
use crate::base::objc::*;
use crate::base::ptr::{self, FromOwned};
use crate::core_media::*;
use crate::foundation::*;

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
    static AVURLAssetPreferPreciseDurationAndTimingKey: ptr::objc::StaticPtr;
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
        self_: ptr::objc::RawPtr,
        key: ptr::objc::RawPtr,
        error: *mut ptr::objc::NullableRawPtr,
    ) -> AVKeyValueStatus;

    fn choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_loadValuesAsynchronouslyForKeys_completionHandler(
        self_: ptr::objc::RawPtr,
        keys: ptr::objc::RawPtr,
        completion_handler: *mut crate::base::block::BlockHeader,
    );
}

pub trait AVAsynchronousKeyValueLoadingProtocol: NSObjectProtocol {
    fn status_of_value_for_key(
        &self,
        key: &impl NSStringInterface,
    ) -> Result<AVKeyValueStatus, NSError> {
        let self_raw = self.as_raw_ptr();
        let mut raw_unowned_error = ptr::objc::NullableRawPtr::default();
        unsafe {
            let status = choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_statusOfValueForKey_error(
                self_raw,
                key.as_raw_ptr(),
                &mut raw_unowned_error,
            );
            make_value_result_unchecked(status, raw_unowned_error)
        }
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
        let self_raw = self.as_raw_ptr();
        let block = HeapBlock::new(handler);
        unsafe {
            choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_loadValuesAsynchronouslyForKeys_completionHandler(
                self_raw,
                keys.as_raw_ptr(),
                block.block_ref().get()
            )
        }
    }
}

//-------------------------------------------------------------------
// AVAsset

extern "C" {
    fn choco_AVFoundation_AVAsset_class() -> ptr::objc::ClassPtr;
    fn choco_AVFoundation_AVAssetInterface_instance_tracks(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVAssetInterface_instance_playable(self_: ptr::objc::RawPtr) -> BOOL;
}

pub trait AVAssetInterface: NSObjectInterface
where
    Self: NSCopyingProtocol + AVAsynchronousKeyValueLoadingProtocol,
{
    fn tracks(&self) -> NSArray<AVAssetTrack> {
        let self_raw = self.as_raw_ptr();
        unsafe {
            let owned_ptr = choco_AVFoundation_AVAssetInterface_instance_tracks(self_raw)
                .unwrap()
                .consider_owned();
            NSArray::from_owned_ptr_unchecked(owned_ptr)
        }
    }

    // Named "playable" and not "is_playable" to be the same as the key to pass to AVAsynchronousKeyValueLoading.
    fn playable(&self) -> bool {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVAssetInterface_instance_playable(self_raw) }.into()
    }
}

pub struct AVAsset {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVAsset {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVAsset {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVAsset {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVAsset_class() }
    }
}

impl NSObjectInterface for AVAsset {}
impl AVAsynchronousKeyValueLoadingProtocol for AVAsset {}
impl AVAssetInterface for AVAsset {}
impl NSCopyingProtocol for AVAsset {
    type Immutable = Self;
}
impl ValidObjCGeneric for AVAsset {}

impl IsKindOf<NSObject> for AVAsset {}

// AVAsset should be mostly fine to use from multiple threads at the same time.
unsafe impl Send for AVAsset {}
unsafe impl Sync for AVAsset {}

//-------------------------------------------------------------------
// AVURLAsset

extern "C" {
    fn choco_AVFoundation_AVURLAsset_class() -> ptr::objc::ClassPtr;
    fn choco_AVFoundation_AVURLAssetInterface_class_newWithURL_options(
        class: ptr::objc::ClassPtr,
        url: ptr::objc::RawPtr,
        options: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
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
        unsafe {
            let owned_ptr = choco_AVFoundation_AVURLAssetInterface_class_newWithURL_options(
                Self::class(),
                url.as_raw_ptr(),
                options.as_raw_ptr(),
            )
            .unwrap()
            .consider_owned();
            Self::Owned::from_owned_ptr_unchecked(owned_ptr)
        }
    }
}

pub struct AVURLAsset {
    ptr: ptr::objc::OwnedPtr,
}

impl AVURLAsset {
    pub fn prefer_precise_duration_and_timing_key() -> StaticNSString {
        unsafe {
            StaticNSString::from_static_unchecked(AVURLAssetPreferPreciseDurationAndTimingKey)
        }
    }
}

impl ptr::AsRaw for AVURLAsset {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVURLAsset {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVURLAsset {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVURLAsset_class() }
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

impl IsKindOf<NSObject> for AVURLAsset {}
impl IsKindOf<AVAsset> for AVURLAsset {}

// AVAsset should be mostly fine to use from multiple threads at the same time.
unsafe impl Send for AVURLAsset {}
unsafe impl Sync for AVURLAsset {}

//-------------------------------------------------------------------
// AVMediaType

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVMediaTypeAudio: ptr::objc::StaticPtr;
    static AVMediaTypeClosedCaption: ptr::objc::StaticPtr;
    static AVMediaTypeDepthData: ptr::objc::StaticPtr;
    static AVMediaTypeMetadata: ptr::objc::StaticPtr;
    static AVMediaTypeMetadataObject: ptr::objc::StaticPtr;
    static AVMediaTypeMuxed: ptr::objc::StaticPtr;
    static AVMediaTypeSubtitle: ptr::objc::StaticPtr;
    static AVMediaTypeText: ptr::objc::StaticPtr;
    static AVMediaTypeTimecode: ptr::objc::StaticPtr;
    static AVMediaTypeVideo: ptr::objc::StaticPtr;
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
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeAudio) },
        }
    }
    pub fn closed_caption() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeClosedCaption) },
        }
    }
    pub fn depth_data() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeDepthData) },
        }
    }
    pub fn metadata() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeMetadata) },
        }
    }
    pub fn metadata_object() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeMetadataObject) },
        }
    }
    pub fn muxed() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeMuxed) },
        }
    }
    pub fn subtitle() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeSubtitle) },
        }
    }
    pub fn text() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeText) },
        }
    }
    pub fn timecode() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeTimecode) },
        }
    }
    pub fn video() -> StaticAVMediaType {
        StaticAVMediaType {
            raw: unsafe { StaticNSString::from_static_unchecked(AVMediaTypeVideo) },
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
    fn choco_AVFoundation_AVAssetTrack_class() -> ptr::objc::ClassPtr;
    fn choco_AVFoundation_AVAssetTrackInterface_instance_mediaType(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVAssetTrackInterface_instance_formatDescriptions(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
}

pub trait AVAssetTrackInterface: NSObjectInterface
where
    Self: NSCopyingProtocol + AVAsynchronousKeyValueLoadingProtocol,
{
    fn media_type(&self) -> AVMediaType {
        let raw_self = self.as_raw_ptr();
        unsafe {
            let owned_ptr = choco_AVFoundation_AVAssetTrackInterface_instance_mediaType(raw_self)
                .unwrap()
                .consider_owned();
            AVMediaType::new(NSString::from_owned_ptr_unchecked(owned_ptr))
        }
    }

    fn format_descriptions(&self) -> NSArray<CMFormatDescription> {
        let raw_self = self.as_raw_ptr();
        unsafe {
            let owned_ptr =
                choco_AVFoundation_AVAssetTrackInterface_instance_formatDescriptions(raw_self)
                    .unwrap()
                    .consider_owned();
            NSArray::from_owned_ptr_unchecked(owned_ptr)
        }
    }
}

pub struct AVAssetTrack {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVAssetTrack {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVAssetTrack {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVAssetTrack {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVAssetTrack_class() }
    }
}

impl NSObjectInterface for AVAssetTrack {}
impl AVAsynchronousKeyValueLoadingProtocol for AVAssetTrack {}
impl AVAssetTrackInterface for AVAssetTrack {}
impl NSCopyingProtocol for AVAssetTrack {
    type Immutable = Self;
}
impl ValidObjCGeneric for AVAssetTrack {}

impl IsKindOf<NSObject> for AVAssetTrack {}

//-------------------------------------------------------------------
// AVAssetReader

extern "C" {
    fn choco_AVFoundation_AVAssetReader_class() -> ptr::objc::ClassPtr;
    fn choco_AVFoundation_AVAssetReaderInterface_class_newWithAsset_error(
        class: ptr::objc::ClassPtr,
        asset: ptr::objc::RawPtr,
        error: *mut ptr::objc::NullableRawPtr,
    ) -> ptr::objc::NullableRawPtr;
}

pub trait AVAssetReaderInterface: NSObjectInterface {
    fn new_with_asset(asset: &impl AVAssetInterface) -> Result<Self::Owned, NSError> {
        let mut raw_unowned_error = ptr::objc::NullableRawPtr::default();
        let raw_ptr = unsafe {
            choco_AVFoundation_AVAssetReaderInterface_class_newWithAsset_error(
                Self::class(),
                asset.as_raw_ptr(),
                &mut raw_unowned_error,
            )
        };
        unsafe { make_object_result_unchecked::<Self>(raw_ptr, raw_unowned_error) }
    }
}

pub struct AVAssetReader {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVAssetReader {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVAssetReader {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVAssetReader {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVAssetReader_class() }
    }
}

impl NSObjectInterface for AVAssetReader {}
impl AVAssetReaderInterface for AVAssetReader {}
impl ValidObjCGeneric for AVAssetReader {}

impl IsKindOf<NSObject> for AVAssetReader {}

//-------------------------------------------------------------------
// AVAssetReaderTrackOutput

extern "C" {
    fn choco_AVFoundation_AVAssetReaderTrackOutput_class() -> ptr::objc::ClassPtr;
}

pub trait AVAssetReaderTrackOutputInterface: NSObjectInterface {}

pub struct AVAssetReaderTrackOutput {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVAssetReaderTrackOutput {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVAssetReaderTrackOutput {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVAssetReaderTrackOutput {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVAssetReaderTrackOutput_class() }
    }
}

impl NSObjectInterface for AVAssetReaderTrackOutput {}
impl AVAssetReaderTrackOutputInterface for AVAssetReaderTrackOutput {}
impl ValidObjCGeneric for AVAssetReaderTrackOutput {}

impl IsKindOf<NSObject> for AVAssetReaderTrackOutput {}

//-------------------------------------------------------------------
// AVAssetReaderOutput

extern "C" {
    fn choco_AVFoundation_AVAssetReaderOutput_class() -> ptr::objc::ClassPtr;
}

pub trait AVAssetReaderOutputInterface: NSObjectInterface {}

pub struct AVAssetReaderOutput {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVAssetReaderOutput {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVAssetReaderOutput {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVAssetReaderOutput {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVAssetReaderOutput_class() }
    }
}

impl NSObjectInterface for AVAssetReaderOutput {}
impl AVAssetReaderOutputInterface for AVAssetReaderOutput {}
impl ValidObjCGeneric for AVAssetReaderOutput {}

impl IsKindOf<NSObject> for AVAssetReaderOutput {}

//-------------------------------------------------------------------
// AVAssetReaderSampleReferenceOutput

extern "C" {
    fn choco_AVFoundation_AVAssetReaderSampleReferenceOutput_class() -> ptr::objc::ClassPtr;
}

pub trait AVAssetReaderSampleReferenceOutputInterface: AVAssetReaderOutputInterface {}

pub struct AVAssetReaderSampleReferenceOutput {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVAssetReaderSampleReferenceOutput {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVAssetReaderSampleReferenceOutput {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVAssetReaderSampleReferenceOutput {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVAssetReaderSampleReferenceOutput_class() }
    }
}

impl NSObjectInterface for AVAssetReaderSampleReferenceOutput {}
impl AVAssetReaderOutputInterface for AVAssetReaderSampleReferenceOutput {}
impl AVAssetReaderSampleReferenceOutputInterface for AVAssetReaderSampleReferenceOutput {}
impl ValidObjCGeneric for AVAssetReaderSampleReferenceOutput {}

impl IsKindOf<NSObject> for AVAssetReaderSampleReferenceOutput {}
impl IsKindOf<AVAssetReaderOutput> for AVAssetReaderSampleReferenceOutput {}

//-------------------------------------------------------------------
// AVPlayerItem

extern "C" {
    fn choco_AVFoundation_AVPlayerItem_class() -> ptr::objc::ClassPtr;
    fn choco_AVFoundation_AVPlayerItemInterface_class_newWithURL(
        class: ptr::objc::ClassPtr,
        url: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVPlayerItemInterface_class_newWithAsset(
        class: ptr::objc::ClassPtr,
        asset: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVPlayerItemInterface_instance_error(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
}

pub trait AVPlayerItemInterface: NSObjectInterface
where
    Self: NSCopyingProtocol,
{
    fn new_with_url(url: &impl NSURLInterface) -> Self::Owned {
        unsafe {
            let owned_ptr = choco_AVFoundation_AVPlayerItemInterface_class_newWithURL(
                Self::class(),
                url.as_raw_ptr(),
            )
            .unwrap()
            .consider_owned();
            Self::Owned::from_owned_ptr_unchecked(owned_ptr)
        }
    }

    fn new_with_asset(asset: &impl AVAssetInterface) -> Self::Owned {
        unsafe {
            let owned_ptr = choco_AVFoundation_AVPlayerItemInterface_class_newWithAsset(
                Self::class(),
                asset.as_raw_ptr(),
            )
            .unwrap()
            .consider_owned();
            Self::Owned::from_owned_ptr_unchecked(owned_ptr)
        }
    }

    fn error(&self) -> Option<NSError> {
        let self_raw = self.as_raw_ptr();
        unsafe {
            choco_AVFoundation_AVPlayerItemInterface_instance_error(self_raw)
                .into_opt()
                .map(|raw| NSError::from_owned_ptr_unchecked(raw.consider_owned()))
        }
    }
}

pub struct AVPlayerItem {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVPlayerItem {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVPlayerItem {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVPlayerItem {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVPlayerItem_class() }
    }
}

impl NSObjectInterface for AVPlayerItem {}
impl AVPlayerItemInterface for AVPlayerItem {}
impl NSCopyingProtocol for AVPlayerItem {
    type Immutable = Self;
}
impl ValidObjCGeneric for AVPlayerItem {}

impl IsKindOf<NSObject> for AVPlayerItem {}

//-------------------------------------------------------------------
// AVPlayer

extern "C" {
    fn choco_AVFoundation_AVPlayer_class() -> ptr::objc::ClassPtr;
    fn choco_AVFoundation_AVPlayerInterface_class_newWithURL(
        class: ptr::objc::ClassPtr,
        url: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVPlayerInterface_class_newWithPlayerItem(
        class: ptr::objc::ClassPtr,
        item: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVPlayerInterface_instance_play(self_: ptr::objc::RawPtr);
    fn choco_AVFoundation_AVPlayerInterface_instance_pause(self_: ptr::objc::RawPtr);
    fn choco_AVFoundation_AVPlayerInterface_instance_rate(self_: ptr::objc::RawPtr) -> f32;
    fn choco_AVFoundation_AVPlayerInterface_instance_currentItem(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVPlayerInterface_instance_error(
        self_: ptr::objc::RawPtr,
    ) -> ptr::objc::NullableRawPtr;
}

pub trait AVPlayerInterface: NSObjectInterface {
    fn new_with_url(url: &impl NSURLInterface) -> Self::Owned {
        unsafe {
            let owned_ptr = choco_AVFoundation_AVPlayerInterface_class_newWithURL(
                Self::class(),
                url.as_raw_ptr(),
            )
            .unwrap()
            .consider_owned();
            Self::Owned::from_owned_ptr_unchecked(owned_ptr)
        }
    }

    fn new_with_player_item(url: &impl AVPlayerItemInterface) -> Self::Owned {
        unsafe {
            let owned_ptr = choco_AVFoundation_AVPlayerInterface_class_newWithPlayerItem(
                Self::class(),
                url.as_raw_ptr(),
            )
            .unwrap()
            .consider_owned();
            Self::Owned::from_owned_ptr_unchecked(owned_ptr)
        }
    }

    fn error(&self) -> Option<NSError> {
        let self_raw = self.as_raw_ptr();
        unsafe {
            choco_AVFoundation_AVPlayerInterface_instance_error(self_raw)
                .into_opt()
                .map(|raw| NSError::from_owned_ptr_unchecked(raw.consider_owned()))
        }
    }

    fn current_item(&self) -> Option<AVPlayerItem> {
        let self_raw = self.as_raw_ptr();
        unsafe {
            choco_AVFoundation_AVPlayerInterface_instance_currentItem(self_raw)
                .into_opt()
                .map(|raw| AVPlayerItem::from_owned_ptr_unchecked(raw.consider_owned()))
        }
    }

    fn play(&self) {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVPlayerInterface_instance_play(self_raw) }
    }

    fn pause(&self) {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVPlayerInterface_instance_pause(self_raw) }
    }

    fn rate(&self) -> f32 {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVPlayerInterface_instance_rate(self_raw) }
    }
}

pub struct AVPlayer {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVPlayer {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVPlayer {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVPlayer {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVPlayer_class() }
    }
}

impl NSObjectInterface for AVPlayer {}
impl AVAssetReaderOutputInterface for AVPlayer {}
impl AVPlayerInterface for AVPlayer {}
impl ValidObjCGeneric for AVPlayer {}

impl IsKindOf<NSObject> for AVPlayer {}

//-------------------------------------------------------------------
// AVAudioPlayer

extern "C" {
    fn choco_AVFoundation_AVAudioPlayer_class() -> ptr::objc::ClassPtr;
    fn choco_AVFoundation_AVAudioPlayerInterface_class_newWithContentsOfURL_error(
        class: ptr::objc::ClassPtr,
        url: ptr::objc::RawPtr,
        error: *mut ptr::objc::NullableRawPtr,
    ) -> ptr::objc::NullableRawPtr;
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_play(self_: ptr::objc::RawPtr) -> BOOL;
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_pause(self_: ptr::objc::RawPtr);
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_stop(self_: ptr::objc::RawPtr);
    fn choco_AVFoundation_AVAudioPlayerInterface_instance_rate(self_: ptr::objc::RawPtr) -> f32;
}

pub trait AVAudioPlayerInterface: NSObjectInterface {
    fn new_with_contents_of_url(url: &impl NSURLInterface) -> Result<Self::Owned, NSError> {
        let mut raw_unowned_error = ptr::objc::NullableRawPtr::default();
        let raw_ptr = unsafe {
            choco_AVFoundation_AVAudioPlayerInterface_class_newWithContentsOfURL_error(
                Self::class(),
                url.as_raw_ptr(),
                &mut raw_unowned_error,
            )
        };
        unsafe { make_object_result_unchecked::<Self>(raw_ptr, raw_unowned_error) }
    }

    fn play(&self) -> bool {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_play(self_raw) }.into()
    }

    fn pause(&self) {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_pause(self_raw) }
    }

    fn stop(&self) {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_stop(self_raw) }
    }

    fn rate(&self) -> f32 {
        let self_raw = self.as_raw_ptr();
        unsafe { choco_AVFoundation_AVAudioPlayerInterface_instance_rate(self_raw) }
    }
}

pub struct AVAudioPlayer {
    ptr: ptr::objc::OwnedPtr,
}

impl ptr::AsRaw for AVAudioPlayer {
    fn as_raw_ptr(&self) -> ptr::objc::RawPtr {
        self.ptr.as_raw_ptr()
    }
}

impl ptr::FromOwned for AVAudioPlayer {
    unsafe fn from_owned_ptr_unchecked(ptr: ptr::objc::OwnedPtr) -> Self {
        Self { ptr }
    }
}

impl NSObjectProtocol for AVAudioPlayer {
    type Owned = Self;

    fn class() -> ptr::objc::ClassPtr {
        unsafe { choco_AVFoundation_AVAudioPlayer_class() }
    }
}

impl NSObjectInterface for AVAudioPlayer {}
impl AVAssetReaderOutputInterface for AVAudioPlayer {}
impl AVAudioPlayerInterface for AVAudioPlayer {}
impl ValidObjCGeneric for AVAudioPlayer {}

impl IsKindOf<NSObject> for AVAudioPlayer {}
