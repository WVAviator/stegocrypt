use crate::mp3::mpegparserror::MPEGParseError;

const CRC_PROTECTION_MASK: u32 = 0b00000000_00000001_00000000_00000000;
const CRC_PROTECTION_MASK_OFFSET: u32 = 16;

/// An enum that represents the CRC protection of the frame.
/// If CRC protection is enabled, a 16-bit CRC checksum is appended to the end of the frame.
/// Most files will have CRC protection disabled.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CRCProtection {
    Disabled,
    Enabled { checksum: u16 },
}

impl CRCProtection {
    /// Given a 32-bit frame header, parse the CRC protection, or throw an error if the CRC protection is invalid.
    pub fn parse(header: u32) -> Result<CRCProtection, MPEGParseError> {
        let crc_protection = (header & CRC_PROTECTION_MASK) >> CRC_PROTECTION_MASK_OFFSET;
        match crc_protection {
            0b1 => Ok(CRCProtection::Disabled),
            0b0 => Ok(CRCProtection::Enabled { checksum: 0 }),
            _ => Err(MPEGParseError::GenericInvalidFrameHeader {
                info: format!("Invalid CRC protection: {}", crc_protection),
            }),
        }
    }

    /// After parsing the frame, extract the checksum from the last two bytes of the frame data.
    pub fn extract_checksum(&mut self, frame_data: &Vec<u8>) {
        let new_checksum = u16::from_be_bytes([
            frame_data[frame_data.len() - 2],
            frame_data[frame_data.len() - 1],
        ]);
        match self {
            CRCProtection::Disabled => {}
            CRCProtection::Enabled { checksum } => {
                *checksum = new_checksum;
            }
        }
    }

    /// Returnsupdated frame data and a new 32-bit frame header with the CRC protection bit set.
    /// If the CRC protection is enabled, the checksum in the frame's last two bytes is updated.
    pub fn apply(&self, header: u32, data: &mut Vec<u8>) -> u32 {
        let result = header & !CRC_PROTECTION_MASK;
        result
            | match self {
                CRCProtection::Disabled => 0b00000000_00000001_00000000_00000000,
                CRCProtection::Enabled { checksum } => {
                    let length = data.len();
                    data[length - 2] = checksum.to_be_bytes()[0];
                    data[length - 1] = checksum.to_be_bytes()[1];
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
        let result = CRCProtection::parse(header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), CRCProtection::Disabled);
    }

    #[test]
    fn parses_enabled_crc_protection() {
        let header = 0b00000000_00000000_00000000_00000000;
        let result = CRCProtection::parse(header);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CRCProtection::Enabled {
                checksum: 0b00000000_00000000
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
        data[8] = 0b00000000;
        data[9] = 0b00000000;
        let result = CRCProtection::Enabled {
            checksum: 0b10000000_00000001,
        }
        .apply(header, &mut data);
        assert_eq!(result, 0b00000000_00000000_00000000_00000000);
        assert_eq!(data[8], 0b10000000);
        assert_eq!(data[9], 0b00000001);
    }

    #[test]
    fn extracts_checksum() {
        let mut data = vec![0; 10];
        data[8] = 0b10000000;
        data[9] = 0b00000001;
        let mut crc_protection = CRCProtection::Enabled { checksum: 0 };
        crc_protection.extract_checksum(&data);
        assert_eq!(
            crc_protection,
            CRCProtection::Enabled {
                checksum: 0b10000000_00000001
            }
        );
    }
}
