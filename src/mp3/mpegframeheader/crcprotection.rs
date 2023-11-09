use crate::mp3::mpegparserror::MPEGParseError;

const CRC_PROTECTION_MASK: u32 = 0b00000000_00000001_00000000_00000000;
const CRC_PROTECTION_MASK_OFFSET: u32 = 16;

/// An enum that represents the CRC (Cyclic Redundancy Check) protection of the frame.
/// If CRC protection is enabled, a 16-bit CRC checksum follows the frame header.
/// Most files will have CRC protection disabled.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CRCProtection {
    Disabled,
    Enabled { checksum: u16 },
}

impl CRCProtection {
    /// Given a 32-bit frame header and a reference to the frame data, parse the CRC protection and checksum, or throw an error if the CRC protection is invalid.
    pub fn parse(header: u32, frame_data: &[u8]) -> Result<CRCProtection, MPEGParseError> {
        let crc_protection = (header & CRC_PROTECTION_MASK) >> CRC_PROTECTION_MASK_OFFSET;
        let checksum: u16 = u16::from_be_bytes([frame_data[4], frame_data[5]]);
        match crc_protection {
            0b1 => Ok(CRCProtection::Disabled),
            0b0 => Ok(CRCProtection::Enabled { checksum }),
            _ => Err(MPEGParseError::GenericInvalidFrameHeader {
                info: format!("Invalid CRC protection: {}", crc_protection),
            }),
        }
    }

    /// Returns updated frame data and a new 32-bit frame header with the CRC protection bit set.
    /// If the CRC protection is enabled, the checksum is stored in the two bytes following the frame header.
    pub fn apply(&self, header: u32, data: &mut Vec<u8>) -> u32 {
        let result = header & !CRC_PROTECTION_MASK;
        result
            | match self {
                CRCProtection::Disabled => 0b00000000_00000001_00000000_00000000,
                CRCProtection::Enabled { checksum } => {
                    let length = data.len();
                    data[0] = checksum.to_be_bytes()[0];
                    data[1] = checksum.to_be_bytes()[1];
                    0b00000000_00000000_00000000_00000000
                }
            }
    }
}

mod test {
    use super::*;

    #[test]
    fn parses_disabled_crc_protection() {
        let header = 0b00000000_00000001_00000000_00000000;
        let data = vec![0; 10];
        let result = CRCProtection::parse(header, &data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), CRCProtection::Disabled);
    }

    #[test]
    fn parses_enabled_crc_protection() {
        let header = 0b00000000_00000000_00000000_00000000;
        let mut data = vec![0; 10];
        data[4] = 0b10000000;
        data[5] = 0b00000001;
        let result = CRCProtection::parse(header, &data);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CRCProtection::Enabled {
                checksum: 0b10000000_00000001
            }
        );
    }

    #[test]
    fn applies_disabled_crc_protection() {
        let header = 0b00000000_00000000_00000000_00000000;
        let mut data = vec![0; 10];
        let result = CRCProtection::Disabled.apply(header, &mut data);
        assert_eq!(result, 0b00000000_00000001_00000000_00000000);
    }

    #[test]
    fn applies_enabled_crc_protection() {
        let header = 0b00000000_00000000_00000000_00000000;
        let mut data = vec![0; 10];
        data[0] = 0b00000000;
        data[1] = 0b00000000;
        let result = CRCProtection::Enabled {
            checksum: 0b10000000_00000001,
        }
        .apply(header, &mut data);
        assert_eq!(result, 0b00000000_00000000_00000000_00000000);
        assert_eq!(data[0], 0b10000000);
        assert_eq!(data[1], 0b00000001);
    }
}
