const MPEG_VERSION_ID: u32 = 0b00000000_00011000_00000000_00000000;
const MPEG_VERSION_ID_OFFSET: u32 = 19;

/// The MPEG Version of the file. Most files will be MPEG Version 1.
#[derive(Copy, Clone)]
pub enum MPEGVersion {
    Version1,
    Version2,
    Version2_5,
    VersionReserved,
}

impl MPEGVersion {
    /// Given a 32-bit frame header, parse the MPEG version.
    /// The version ID is extracted from bits 19 and 20 and then mapped to the appropriate MPEGVersion enum.
    pub fn parse(data: u32) -> MPEGVersion {
        let version_id = (data & MPEG_VERSION_ID) >> MPEG_VERSION_ID_OFFSET;
        match version_id {
            0b00 => MPEGVersion::Version2_5,
            0b01 => MPEGVersion::VersionReserved,
            0b10 => MPEGVersion::Version2,
            0b11 => MPEGVersion::Version1,
            _ => panic!("Invalid MPEG version ID"),
        }
    }

    /// Returns a new 32-bit frame header with this version applied.
    /// Bits 19 and 20 from the provided header are overridden, and then the appropriate bits for this version ID applied in the new frame header.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & 0b11111111_11100111_11111111_11111111;
        result
            | match self {
                MPEGVersion::Version1 => 0b00000000_00011000_00000000_00000000,
                MPEGVersion::Version2 => 0b00000000_00010000_00000000_00000000,
                MPEGVersion::Version2_5 => 0b00000000_00000000_00000000_00000000,
                MPEGVersion::VersionReserved => 0b00000000_00001000_00000000_00000000,
            }
    }
}
