use crate::mp3::mpegparserror::MPEGParseError;

const LAYER_MASK: u32 = 0b00000000_00000110_00000000_00000000;
const LAYER_MASK_OFFSET: u32 = 17;

/// The MPEG Layer of the file. Most files will be Layer 3 (hence the designation "MP3").
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MPEGLayer {
    Reserved,
    Layer3,
    Layer2,
    Layer1,
}

impl MPEGLayer {
    /// Given a 32-bit frame header, parse the MPEG layer.
    /// The layer ID is extracted from bits 17 and 18 and then mapped to the appropriate MPEGLayer enum.
    pub fn parse(data: u32) -> Result<MPEGLayer, MPEGParseError> {
        let layer_id = (data & LAYER_MASK) >> LAYER_MASK_OFFSET;
        match layer_id {
            0b00 => Ok(MPEGLayer::Reserved),
            0b01 => Ok(MPEGLayer::Layer3),
            0b10 => Ok(MPEGLayer::Layer2),
            0b11 => Ok(MPEGLayer::Layer1),
            _ => Err(MPEGParseError::GenericInvalidFrameHeader {
                info: format!("Invalid MPEG layer ID: {}", layer_id),
            }),
        }
    }

    /// Returns a new 32-bit frame header with this layer applied.
    /// Bits 17 and 18 from the provided header are overridden, and then the appropriate bits for this layer ID applied in the new frame header.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & !LAYER_MASK;
        result
            | match self {
                MPEGLayer::Reserved => 0b00000000_00000000_00000000_00000000,
                MPEGLayer::Layer3 => 0b00000000_00000010_00000000_00000000,
                MPEGLayer::Layer2 => 0b00000000_00000100_00000000_00000000,
                MPEGLayer::Layer1 => 0b00000000_00000110_00000000_00000000,
            }
    }
}

mod test {
    use super::*;

    #[test]
    fn parses_mpeg_layer_3() {
        let header = 0b00000000_00000010_00000000_00000000;
        let result = MPEGLayer::parse(header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MPEGLayer::Layer3);
    }

    #[test]
    fn applies_mpeg_layer_3() {
        let header = 0b00000000_00000000_00000000_00000000;
        let result = MPEGLayer::Layer3.apply(header);
        assert_eq!(result, 0b00000000_00000010_00000000_00000000);
    }
}
