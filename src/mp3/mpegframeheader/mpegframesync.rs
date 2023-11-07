use crate::mp3::mpegparserror::MPEGParseError;

const FRAME_SYNC_MASK: u32 = 0b11111111_11100000_00000000_00000000;

/// An enum that represents the MPEG frame sync.
/// This is an 11-bit sequence of 1s that indicates the start of a frame.
pub enum MPEGFrameSync {
    Sync,
}

impl MPEGFrameSync {
    /// Given a 32-bit frame header, parse the MPEG frame sync, or throw an error if the frame sync is not found.
    pub fn parse(data: u32) -> Result<MPEGFrameSync, MPEGParseError> {
        if ((data & FRAME_SYNC_MASK) != FRAME_SYNC_MASK) {
            return Err(MPEGParseError::NoFrameSync);
        }

        Ok(MPEGFrameSync::Sync)
    }

    /// Returns a new 32-bit frame header with the frame sync bits set.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & !FRAME_SYNC_MASK;
        result | FRAME_SYNC_MASK
    }
}
