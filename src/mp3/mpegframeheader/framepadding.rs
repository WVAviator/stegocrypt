use crate::mp3::mpegparserror::MPEGParseError;

const PADDING_MASK: u32 = 0b00000000_00000000_00000010_00000000;
const PADDING_MASK_OFFSET: u32 = 9;

/// An enum that represents the padding of the frame.
/// If padding is enabled, the frame is padded with an extra byte at the end.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FramePadding {
    Enabled,
    Disabled,
}

impl FramePadding {
    /// Given a 32-bit frame header, parse the padding, or throw an error if the padding is invalid.
    pub fn parse(data: u32) -> Result<FramePadding, MPEGParseError> {
        let padding = (data & PADDING_MASK) >> PADDING_MASK_OFFSET;
        match padding {
            0b0 => Ok(FramePadding::Disabled),
            0b1 => Ok(FramePadding::Enabled),
            _ => Err(MPEGParseError::GenericInvalidFrameHeader {
                info: format!("Invalid padding: {}", padding),
            }),
        }
    }

    /// Returns a new 32-bit frame header with this padding applied.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & !PADDING_MASK;
        result
            | match self {
                FramePadding::Disabled => 0b00000000_00000000_00000000_00000000,
                FramePadding::Enabled => 0b00000000_00000000_00000010_00000000,
            }
    }
}

mod test {
    use super::*;

    #[test]
    fn parses_padding_enabled() {
        let header = 0b00000000_00000000_00000010_00000000;
        let result = FramePadding::parse(header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FramePadding::Enabled);
    }

    #[test]
    fn applies_padding_enabled() {
        let header = 0b00000000_00000000_00000000_00000000;
        let result = FramePadding::Enabled.apply(header);
        assert_eq!(result, 0b00000000_00000000_00000010_00000000);
    }
}
