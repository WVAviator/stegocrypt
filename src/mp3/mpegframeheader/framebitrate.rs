use crate::mp3::mpegparserror::MPEGParseError;

const BITRATE_INDEX_MASK: u32 = 0b00000000_00000000_11110000_00000000;
const BITRATE_INDEX_MASK_OFFSET: u32 = 12;

/// An enum that represents the bitrate of the frame.
/// The bitrate is the number of bits per second of audio.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameBitrate {
    Bitrate(u32),
    Free,
}

impl FrameBitrate {
    /// Given a 32-bit frame header, parse the bitrate, or throw an error if the bitrate is invalid.
    /// The bitrate is represented by a 4-bit index into a table of bitrates.
    /// If the index is 0, the bitrate is free.
    /// If the index is 15, the bitrate is bad and will error.
    pub fn parse(data: u32) -> Result<FrameBitrate, MPEGParseError> {
        let bitrate_index = (data & BITRATE_INDEX_MASK) >> BITRATE_INDEX_MASK_OFFSET;
        match bitrate_index {
            0b0000 => Ok(FrameBitrate::Free),
            0b0001 => Ok(FrameBitrate::Bitrate(32)),
            0b0010 => Ok(FrameBitrate::Bitrate(40)),
            0b0011 => Ok(FrameBitrate::Bitrate(48)),
            0b0100 => Ok(FrameBitrate::Bitrate(56)),
            0b0101 => Ok(FrameBitrate::Bitrate(64)),
            0b0110 => Ok(FrameBitrate::Bitrate(80)),
            0b0111 => Ok(FrameBitrate::Bitrate(96)),
            0b1000 => Ok(FrameBitrate::Bitrate(112)),
            0b1001 => Ok(FrameBitrate::Bitrate(128)),
            0b1010 => Ok(FrameBitrate::Bitrate(160)),
            0b1011 => Ok(FrameBitrate::Bitrate(192)),
            0b1100 => Ok(FrameBitrate::Bitrate(224)),
            0b1101 => Ok(FrameBitrate::Bitrate(256)),
            0b1110 => Ok(FrameBitrate::Bitrate(320)),
            _ => Err(MPEGParseError::BadFrameBitrate),
        }
    }

    /// Returns a new 32-bit frame header with this bitrate applied.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & !BITRATE_INDEX_MASK;
        result
            | match self {
                FrameBitrate::Free => 0b00000000_00000000_00000000_00000000,
                FrameBitrate::Bitrate(32) => 0b00000000_00000000_00000000_00010000,
                FrameBitrate::Bitrate(40) => 0b00000000_00000000_00000000_00100000,
                FrameBitrate::Bitrate(48) => 0b00000000_00000000_00000000_00110000,
                FrameBitrate::Bitrate(56) => 0b00000000_00000000_00000000_01000000,
                FrameBitrate::Bitrate(64) => 0b00000000_00000000_00000000_01010000,
                FrameBitrate::Bitrate(80) => 0b00000000_00000000_00000000_01100000,
                FrameBitrate::Bitrate(96) => 0b00000000_00000000_00000000_01110000,
                FrameBitrate::Bitrate(112) => 0b00000000_00000000_00000000_10000000,
                FrameBitrate::Bitrate(128) => 0b00000000_00000000_00000000_10010000,
                FrameBitrate::Bitrate(160) => 0b00000000_00000000_00000000_10100000,
                FrameBitrate::Bitrate(192) => 0b00000000_00000000_00000000_10110000,
                FrameBitrate::Bitrate(224) => 0b00000000_00000000_00000000_11000000,
                FrameBitrate::Bitrate(256) => 0b00000000_00000000_00000000_11010000,
                FrameBitrate::Bitrate(320) => 0b00000000_00000000_00000000_11100000,
                FrameBitrate::Bitrate(_) => 0b00000000_00000000_00000000_11110000,
            }
    }
}
