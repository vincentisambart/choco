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
