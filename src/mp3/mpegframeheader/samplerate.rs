use crate::mp3::mpegparserror::MPEGParseError;

const SAMPLE_RATE_INDEX_MASK: u32 = 0b00000000_00000000_00001100_00000000;
const SAMPLE_RATE_INDEX_MASK_OFFSET: u32 = 10;

/// Represents the sampling rate frequency for the audio in Hz.
/// For most MP3 files this should be 44100Hz.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SampleRate {
    Hz44100,
    Hz48000,
    Hz32000,
}

impl SampleRate {
    /// Given a 32-bit header, parse the sampling rate information.
    /// The sample rate index is stored as bits 21 and 22 and maps to four possible frequency values.
    pub fn parse(header: u32) -> Result<SampleRate, MPEGParseError> {
        let sample_rate_index = (header & SAMPLE_RATE_INDEX_MASK) >> SAMPLE_RATE_INDEX_MASK_OFFSET;

        match sample_rate_index {
            0b00 => Ok(SampleRate::Hz44100),
            0b01 => Ok(SampleRate::Hz48000),
            0b10 => Ok(SampleRate::Hz32000),
            _ => Err(MPEGParseError::GenericInvalidFrameHeader {
                info: format!("Invalid sample rate index: {}", sample_rate_index),
            }),
        }
    }

    /// Given an existing 32-bit header, a new header is returned with this sampling rate index applied.
    pub fn apply(&self, header: u32) -> u32 {
        let result = header & !SAMPLE_RATE_INDEX_MASK;

        result
            | match self {
                SampleRate::Hz44100 => 0b00000000_00000000_00000000_00000000,
                SampleRate::Hz48000 => 0b00000000_00000000_00000100_00000000,
                SampleRate::Hz32000 => 0b00000000_00000000_00001000_00000000,
            }
    }
}

mod test {
    use super::*;

    #[test]
    fn parses_correct_sample_rate() {
        let header: u32 = 0b11111111_11100000_00000100_00000000;
        let sample_rate = SampleRate::parse(header).unwrap();
        assert!(sample_rate == SampleRate::Hz48000);

        let header: u32 = 0b11111111_11100000_00110011_00000000;
        let sample_rate = SampleRate::parse(header).unwrap();
        assert!(sample_rate == SampleRate::Hz44100);
    }

    #[test]
    fn applies_sample_rate_to_header() {
        let header: u32 = 0b11111111_11100000_00000100_00000000;
        let sample_rate = SampleRate::Hz32000;
        let header = sample_rate.apply(header);
        assert!(header == 0b11111111_11100000_00001000_00000000);
    }
}
