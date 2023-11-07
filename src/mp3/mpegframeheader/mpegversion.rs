use crate::mp3::mpegparserror::MPEGParseError;

const MPEG_VERSION_ID_MASK: u32 = 0b00000000_00011000_00000000_00000000;
const MPEG_VERSION_ID_MASK_OFFSET: u32 = 19;

/// The MPEG Version of the file. Most files will be MPEG Version 1.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MPEGVersion {
    Version1,
    Version2,
    Version2_5,
    VersionReserved,
}

impl MPEGVersion {
    /// Given a 32-bit frame header, parse the MPEG version.
    /// The version ID is extracted from bits 19 and 20 and then mapped to the appropriate MPEGVersion enum.
    pub fn parse(data: u32) -> Result<MPEGVersion, MPEGParseError> {
        let version_id = (data & MPEG_VERSION_ID_MASK) >> MPEG_VERSION_ID_MASK_OFFSET;
        match version_id {
            0b00 => Ok(MPEGVersion::Version2_5),
            0b01 => Ok(MPEGVersion::VersionReserved),
            0b10 => Ok(MPEGVersion::Version2),
            0b11 => Ok(MPEGVersion::Version1),
            _ => Err(MPEGParseError::GenericInvalidFrameHeader {
                info: format!("Invalid MPEG version ID: {}", version_id),
            }),
        }
    }

    /// Returns a new 32-bit frame header with this version applied.
    /// Bits 19 and 20 from the provided header are overridden, and then the appropriate bits for this version ID applied in the new frame header.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & !MPEG_VERSION_ID_MASK;
        result
            | match self {
                MPEGVersion::Version1 => 0b00000000_00011000_00000000_00000000,
                MPEGVersion::Version2 => 0b00000000_00010000_00000000_00000000,
                MPEGVersion::Version2_5 => 0b00000000_00000000_00000000_00000000,
                MPEGVersion::VersionReserved => 0b00000000_00001000_00000000_00000000,
            }
    }
}

mod test {
    use super::*;

    #[test]
    fn parses_mpeg_version_1() {
        let header = 0b00000000_00011000_00000000_00000000;
        let result = MPEGVersion::parse(header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MPEGVersion::Version1);
    }

    #[test]
    fn applies_mpeg_version_1() {
        let header = 0b00000000_00000000_00000000_00000000;
        let result = MPEGVersion::Version1.apply(header);
        assert_eq!(result, 0b00000000_00011000_00000000_00000000);
    }
}
