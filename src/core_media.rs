use crate::base::core_foundation::*;
use crate::base::objc::*;
use crate::base::{fourcc, ptr};
use crate::core_graphics::CGRect;

pub(crate) mod prelude {
    pub use super::CMAudioFormatDescriptionInterface;
    pub use super::CMClosedCaptionFormatDescriptionInterface;
    pub use super::CMFormatDescriptionInterface;
    pub use super::CMMetadataFormatDescriptionInterface;
    pub use super::CMMuxedFormatDescriptionInterface;
    pub use super::CMTextFormatDescriptionInterface;
    pub use super::CMTimeCodeFormatDescriptionInterface;
    pub use super::CMVideoFormatDescriptionInterface;
}

//-------------------------------------------------------------------
// CMTime

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CMTimeValue(pub i64);
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CMTimeScale(pub i32);
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CMTimeEpoch(pub i64);

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CMTimeFlags(pub u32);

impl CMTimeFlags {
    pub const VALID: Self = Self(1 << 0);
    pub const HAS_BEEN_ROUNDED: Self = Self(1 << 1);
    pub const POSITIVE_INFINITY: Self = Self(1 << 2);
    pub const NEGATIVE_INFINITY: Self = Self(1 << 3);
    pub const INDEFINITE: Self = Self(1 << 4);
    pub const IMPLIED_VALUE_FLAGS_MASK: Self =
        Self(Self::POSITIVE_INFINITY.0 | Self::NEGATIVE_INFINITY.0 | Self::INDEFINITE.0);
}

impl std::ops::BitOr for CMTimeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for CMTimeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}

#[cfg(test)]
mod cmtime_tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(std::mem::size_of::<CMTime>(), 24);
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMTimeRange {
    pub start: CMTime,
    pub duration: CMTime,
}

//-------------------------------------------------------------------
// CMFormatDescriptionRef

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {
    fn CMFormatDescriptionGetTypeID() -> CFTypeID;
    fn CMFormatDescriptionGetMediaType(desc: ptr::cf::RawRef) -> CMMediaType;
    fn CMVideoFormatDescriptionGetCleanAperture(
        video_desc: ptr::cf::RawRef,
        origin_is_at_top_left: Boolean,
    ) -> CGRect;
    fn CMVideoFormatDescriptionGetDimensions(video_desc: ptr::cf::RawRef) -> CMVideoDimensions;
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CMMediaType(pub u32);

impl CMMediaType {
    pub const AUDIO: Self = Self(fourcc("soun"));
    pub const VIDEO: Self = Self(fourcc("vide"));
    pub const MUXED: Self = Self(fourcc("muxx"));
    pub const METADATA: Self = Self(fourcc("meta"));
    pub const TEXT: Self = Self(fourcc("text"));
    pub const TIME_CODE: Self = Self(fourcc("tmcd"));
    pub const CLOSED_CAPTION: Self = Self(fourcc("clcp"));
    pub const SUBTITLE: Self = Self(fourcc("sbtl"));
}

pub enum TypedCMFormatDescription {
    Audio(CMAudioFormatDescription),
    Video(CMVideoFormatDescription),
    Muxed(CMMuxedFormatDescription),
    Metadata(CMMetadataFormatDescription),
    Text(CMTextFormatDescription),
    TimeCode(CMTimeCodeFormatDescription),
    ClosedCaption(CMClosedCaptionFormatDescription),
    /// `Unknown` includes the subtitle media type Apple doesn't have a type for.
    Unknown(CMFormatDescription),
}

impl From<CMFormatDescription> for TypedCMFormatDescription {
    fn from(desc: CMFormatDescription) -> Self {
        use TypedCMFormatDescription::*;
        match desc.media_type() {
            CMMediaType::AUDIO => Audio(CMAudioFormatDescription { ptr: desc.ptr }),
            CMMediaType::VIDEO => Video(CMVideoFormatDescription { ptr: desc.ptr }),
            CMMediaType::MUXED => Muxed(CMMuxedFormatDescription { ptr: desc.ptr }),
            CMMediaType::METADATA => Metadata(CMMetadataFormatDescription { ptr: desc.ptr }),
            CMMediaType::TEXT => Text(CMTextFormatDescription { ptr: desc.ptr }),
            CMMediaType::TIME_CODE => TimeCode(CMTimeCodeFormatDescription { ptr: desc.ptr }),
            CMMediaType::CLOSED_CAPTION => {
                ClosedCaption(CMClosedCaptionFormatDescription { ptr: desc.ptr })
            }
            _ => Unknown(desc),
        }
    }
}

pub trait CMFormatDescriptionInterface: CFTypeInterface {
    fn type_id() -> CFTypeID {
        unsafe { CMFormatDescriptionGetTypeID() }
    }

    fn media_type(&self) -> CMMediaType {
        let self_raw = self.as_raw_ref();
        unsafe { CMFormatDescriptionGetMediaType(self_raw) }
    }
}

pub struct CMFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl CFTypeInterface for CMFormatDescription {}
impl CMFormatDescriptionInterface for CMFormatDescription {}
pub trait CMAudioFormatDescriptionInterface: CMFormatDescriptionInterface {}
impl ValidObjCGeneric for CMFormatDescription {}

pub struct CMAudioFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMAudioFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl ptr::FromOwned for CMFormatDescription {
    unsafe fn from_owned_ref_unchecked(owned_ref: ptr::cf::OwnedRef) -> Self {
        Self { ptr: owned_ref }
    }
}

impl CFTypeInterface for CMAudioFormatDescription {}
impl CMFormatDescriptionInterface for CMAudioFormatDescription {}
impl CMAudioFormatDescriptionInterface for CMAudioFormatDescription {}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
#[repr(C)]
pub struct CMVideoDimensions {
    width: i32,
    height: i32,
}

pub trait CMVideoFormatDescriptionInterface: CMFormatDescriptionInterface {
    fn clean_aperture(&self, origin_is_at_top_left: bool) -> CGRect {
        let self_raw = self.as_raw_ref();
        unsafe { CMVideoFormatDescriptionGetCleanAperture(self_raw, origin_is_at_top_left.into()) }
    }

    fn dimensions(&self) -> CMVideoDimensions {
        let self_raw = self.as_raw_ref();
        unsafe { CMVideoFormatDescriptionGetDimensions(self_raw) }
    }
}

pub struct CMVideoFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMVideoFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl CFTypeInterface for CMVideoFormatDescription {}
impl CMFormatDescriptionInterface for CMVideoFormatDescription {}
impl CMVideoFormatDescriptionInterface for CMVideoFormatDescription {}

pub trait CMMuxedFormatDescriptionInterface: CMFormatDescriptionInterface {}

pub struct CMMuxedFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMMuxedFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl CFTypeInterface for CMMuxedFormatDescription {}
impl CMFormatDescriptionInterface for CMMuxedFormatDescription {}
impl CMMuxedFormatDescriptionInterface for CMMuxedFormatDescription {}

pub trait CMMetadataFormatDescriptionInterface: CMFormatDescriptionInterface {}

pub struct CMMetadataFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMMetadataFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl CFTypeInterface for CMMetadataFormatDescription {}
impl CMFormatDescriptionInterface for CMMetadataFormatDescription {}
impl CMMetadataFormatDescriptionInterface for CMMetadataFormatDescription {}

pub trait CMTextFormatDescriptionInterface: CMFormatDescriptionInterface {}

pub struct CMTextFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMTextFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl CFTypeInterface for CMTextFormatDescription {}
impl CMFormatDescriptionInterface for CMTextFormatDescription {}
impl CMTextFormatDescriptionInterface for CMTextFormatDescription {}

pub trait CMTimeCodeFormatDescriptionInterface: CMFormatDescriptionInterface {}

pub struct CMTimeCodeFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMTimeCodeFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl CFTypeInterface for CMTimeCodeFormatDescription {}
impl CMFormatDescriptionInterface for CMTimeCodeFormatDescription {}
impl CMTimeCodeFormatDescriptionInterface for CMTimeCodeFormatDescription {}

pub trait CMClosedCaptionFormatDescriptionInterface: CMFormatDescriptionInterface {}

pub struct CMClosedCaptionFormatDescription {
    ptr: ptr::cf::OwnedRef,
}

impl ptr::AsRaw for CMClosedCaptionFormatDescription {
    fn as_raw_ref(&self) -> ptr::cf::RawRef {
        self.ptr.as_raw_ref()
    }
}

impl CFTypeInterface for CMClosedCaptionFormatDescription {}
impl CMFormatDescriptionInterface for CMClosedCaptionFormatDescription {}
impl CMClosedCaptionFormatDescriptionInterface for CMClosedCaptionFormatDescription {}
