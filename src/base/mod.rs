pub(crate) mod block;
pub(crate) mod core_foundation;
pub(crate) mod objc;
pub(crate) mod ptr;

pub use objc::{autorelease_pool, NSObject};

/// Computes the FourCC for the string passed.
///
/// Expecting the &str passed to be all ASCII of length 4.
/// No explicit check is done (though a shorter length will end up with a panic).
const fn fourcc_unchecked(text: &str) -> u32 {
    let bytes = text.as_bytes();

    ((bytes[0] & 0x7F) as u32) << 24
        | ((bytes[1] & 0x7F) as u32) << 16
        | ((bytes[2] & 0x7F) as u32) << 8
        | ((bytes[3] & 0x7F) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fourcc() {
        assert_eq!(fourcc_unchecked("soun"), 0x736F756Eu32);
        assert_eq!(fourcc_unchecked("text"), 0x74657874u32);
    }
}
