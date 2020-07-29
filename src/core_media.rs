use crate::base::*;
use crate::core_graphics::CGRect;
use choco_macro::fourcc;

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
    pub const VIDEO: Self = Self(fourcc!("vide"));
    pub const AUDIO: Self = Self(fourcc!("soun"));
    pub const MUXED: Self = Self(fourcc!("muxx"));
    pub const TEXT: Self = Self(fourcc!("text"));
    pub const CLOSED_CAPTION: Self = Self(fourcc!("clcp"));
    pub const SUBTITLE: Self = Self(fourcc!("sbtl"));
    pub const TIME_CODE: Self = Self(fourcc!("tmcd"));
    pub const METADATA: Self = Self(fourcc!("meta"));
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
#[derive(Clone)]
pub struct CMFormatDescription {
    ptr: OwnedCFTypeRef,
}

impl AsRawObjCPtr for CMFormatDescription {
    fn as_raw(&self) -> RawObjCPtr {
        self.ptr.as_raw().into()
    }
}

impl TypedOwnedObjCPtr for CMFormatDescription {
    unsafe fn from_owned_unchecked(ptr: OwnedObjCPtr) -> Self {
        Self { ptr: ptr.into() }
    }
}

impl CFTypeInterface for CMFormatDescription {
    fn as_raw(&self) -> RawCFTypeRef {
        self.ptr.as_raw()
    }
}

impl CMFormatDescriptionInterface for CMFormatDescription {}

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
