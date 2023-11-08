use crate::mp3::mpegparserror::MPEGParseError;

const COPYRIGHT_MASK: u32 = 0b00000000_00000000_00000000_00001000;
const COPYRIGHT_MASK_OFFSET: u32 = 3;

/// The copy permitted bit from the frame header.
/// This bit is set to 1 if the file is copyright protected.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Copyright {
    Unprotected,
    Protected,
}

impl Copyright {
    /// Given a 32-bit frame header, parse the copy permitted bit.
    pub fn parse(data: u32) -> Result<Copyright, MPEGParseError> {
        let copy = (data & COPYRIGHT_MASK) >> COPYRIGHT_MASK_OFFSET;
        match copy {
            0b0 => Ok(Copyright::Unprotected),
            0b1 => Ok(Copyright::Protected),
            _ => Err(MPEGParseError::GenericInvalidFrameHeader {
                info: format!("Invalid copy permitted: {}", copy),
            }),
        }
    }

    /// Returns a new 32-bit frame header with this copy permitted bit applied.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & !COPYRIGHT_MASK;
        result
            | match self {
                Copyright::Unprotected => 0b00000000_00000000_00000000_00000000,
                Copyright::Protected => 0b00000000_00000000_00000000_00001000,
            }
    }
}

mod test {
    use super::*;

    #[test]
    fn parses_copy_unprotected() {
        let header = 0b00000000_00000000_00000000_00000000;
        let result = Copyright::parse(header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Copyright::Unprotected);
    }

    #[test]
    fn applies_copyright_protected() {
        let header = 0b00000000_00000000_00000000_00000000;
        let result = Copyright::Protected.apply(header);
        assert_eq!(result, 0b00000000_00000000_00000000_00001000);
    }
}
