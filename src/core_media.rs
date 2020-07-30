use crate::base::*;
use crate::core_graphics::CGRect;
use choco_macro::{fourcc, CFType};

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

#[repr(C)]
pub struct CMTime {
    value: CMTimeValue,
    timescale: CMTimeScale,
    flags: CMTimeFlags,
    epoch: CMTimeEpoch,
}

#[cfg(test)]
mod cmtime_tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(std::mem::size_of::<CMTime>(), 24);
    }
}

//-------------------------------------------------------------------
// CMFormatDescriptionRef

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {
    fn CMFormatDescriptionGetTypeID() -> CFTypeID;
    fn CMFormatDescriptionGetMediaType(desc: RawCFTypeRef) -> CMMediaType;
    fn CMVideoFormatDescriptionGetCleanAperture(
        video_desc: RawCFTypeRef,
        origin_is_at_top_left: Boolean,
    ) -> CGRect;
    fn CMVideoFormatDescriptionGetDimensions(video_desc: RawCFTypeRef) -> CMVideoDimensions;
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CMMediaType(pub u32);

impl CMMediaType {
    pub const AUDIO: Self = Self(fourcc!("soun"));
    pub const VIDEO: Self = Self(fourcc!("vide"));
    pub const MUXED: Self = Self(fourcc!("muxx"));
    pub const METADATA: Self = Self(fourcc!("meta"));
    pub const TEXT: Self = Self(fourcc!("text"));
    pub const TIME_CODE: Self = Self(fourcc!("tmcd"));
    pub const CLOSED_CAPTION: Self = Self(fourcc!("clcp"));
    pub const SUBTITLE: Self = Self(fourcc!("sbtl"));
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
        let self_raw = self.as_raw();
        unsafe { CMFormatDescriptionGetMediaType(self_raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl CMFormatDescriptionInterface for CMFormatDescription {}

pub trait CMAudioFormatDescriptionInterface: CMFormatDescriptionInterface {}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMAudioFormatDescription {
    ptr: OwnedCFTypeRef,
}

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
        let self_raw = self.as_raw();
        unsafe { CMVideoFormatDescriptionGetCleanAperture(self_raw, origin_is_at_top_left.into()) }
    }

    fn dimensions(&self) -> CMVideoDimensions {
        let self_raw = self.as_raw();
        unsafe { CMVideoFormatDescriptionGetDimensions(self_raw) }
    }
}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMVideoFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl CMFormatDescriptionInterface for CMVideoFormatDescription {}
impl CMVideoFormatDescriptionInterface for CMVideoFormatDescription {}

pub trait CMMuxedFormatDescriptionInterface: CMFormatDescriptionInterface {}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMMuxedFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl CMFormatDescriptionInterface for CMMuxedFormatDescription {}
impl CMMuxedFormatDescriptionInterface for CMMuxedFormatDescription {}

pub trait CMMetadataFormatDescriptionInterface: CMFormatDescriptionInterface {}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMMetadataFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl CMFormatDescriptionInterface for CMMetadataFormatDescription {}
impl CMMetadataFormatDescriptionInterface for CMMetadataFormatDescription {}

pub trait CMTextFormatDescriptionInterface: CMFormatDescriptionInterface {}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMTextFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl CMFormatDescriptionInterface for CMTextFormatDescription {}
impl CMTextFormatDescriptionInterface for CMTextFormatDescription {}

pub trait CMTimeCodeFormatDescriptionInterface: CMFormatDescriptionInterface {}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMTimeCodeFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl CMFormatDescriptionInterface for CMTimeCodeFormatDescription {}
impl CMTimeCodeFormatDescriptionInterface for CMTimeCodeFormatDescription {}

pub trait CMClosedCaptionFormatDescriptionInterface: CMFormatDescriptionInterface {}

#[repr(transparent)]
#[derive(Clone, CFType)]
pub struct CMClosedCaptionFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl CMFormatDescriptionInterface for CMClosedCaptionFormatDescription {}
impl CMClosedCaptionFormatDescriptionInterface for CMClosedCaptionFormatDescription {}
