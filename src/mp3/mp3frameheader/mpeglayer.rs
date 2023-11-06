const LAYER_MASK: u32 = 0b00000000_00000110_00000000_00000000;
const LAYER_MASK_OFFSET: u32 = 17;

/// The MPEG Layer of the file. Most files will be Layer 3 (hence the designation "MP3").
#[derive(Copy, Clone)]
pub enum MPEGLayer {
    Reserved,
    Layer3,
    Layer2,
    Layer1,
}

impl MPEGLayer {
    /// Given a 32-bit frame header, parse the MPEG layer.
    /// The layer ID is extracted from bits 17 and 18 and then mapped to the appropriate MPEGLayer enum.
    pub fn parse(data: u32) -> MPEGLayer {
        let layer_id = (data & LAYER_MASK) >> LAYER_MASK_OFFSET;
        match layer_id {
            0b00 => MPEGLayer::Reserved,
            0b01 => MPEGLayer::Layer3,
            0b10 => MPEGLayer::Layer2,
            0b11 => MPEGLayer::Layer1,
            _ => panic!("Invalid MPEG layer ID"),
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
