use self::copyright::Copyright;
use self::crcprotection::CRCProtection;
use self::framebitrate::FrameBitrate;
use self::framepadding::FramePadding;
use self::mpegframesync::MPEGFrameSync;
use self::mpeglayer::MPEGLayer;
use self::mpegversion::MPEGVersion;
use self::samplerate::SampleRate;

use super::mpegparserror::MPEGParseError;

mod copyright;
mod crcprotection;
mod framebitrate;
mod framepadding;
mod mpegframesync;
mod mpeglayer;
mod mpegversion;
mod samplerate;

const PRIVATE_BIT: u32 = 0b00000000_00000000_00000001_00000000;
const PRIVATE_BIT_OFFSET: u32 = 8;

const CHANNEL_MODE: u32 = 0b00000000_00000000_00000000_11000000;
const CHANNEL_MODE_OFFSET: u32 = 6;
const CHANNEL_MODE_TABLE: [MP3ChannelMode; 4] = [
    MP3ChannelMode::Stereo,
    MP3ChannelMode::JointStereo,
    MP3ChannelMode::DualChannel,
    MP3ChannelMode::SingleChannel,
];

const MODE_EXTENSION: u32 = 0b00000000_00000000_00000000_00110000;
const MODE_EXTENSION_OFFSET: u32 = 4;
const MODE_EXTENSION_TABLE: [MP3ModeExtension; 4] = [
    MP3ModeExtension::Bands4To31,
    MP3ModeExtension::Bands8To31,
    MP3ModeExtension::Bands12To31,
    MP3ModeExtension::Bands16To31,
];

const ORIGINAL: u32 = 0b00000000_00000000_00000000_00000100;
const ORIGINAL_OFFSET: u32 = 2;
const ORIGINAL_TABLE: [Original; 2] = [Original::Original, Original::Copy];

const EMPHASIS: u32 = 0b00000000_00000000_00000000_00000011;
const EMPHASIS_OFFSET: u32 = 0;
const EMPHASIS_TABLE: [MP3Emphasis; 4] = [
    MP3Emphasis::None,
    MP3Emphasis::FiftyFifteen,
    MP3Emphasis::Reserved,
    MP3Emphasis::CCITJ17,
];

pub struct MPEGFrameHeader {
    pub raw_header: u32,
    pub frame_data: Vec<u8>,
    pub frame_length: u32,
    pub frame_sync: MPEGFrameSync,
    pub version: MPEGVersion,
    pub layer: MPEGLayer,
    pub crc_protection: CRCProtection,
    pub bitrate: FrameBitrate,
    pub sample_rate: SampleRate,
    pub padding: FramePadding,
    pub private_bit: bool,
    pub channel_mode: MP3ChannelMode,
    pub mode_extension: MP3ModeExtension,
    pub copyright: Copyright,
    pub original: Original,
    pub emphasis: MP3Emphasis,
}

impl MPEGFrameHeader {
    pub fn parse(data: &Vec<u8>) -> Result<MPEGFrameHeader, MPEGParseError> {
        let raw_header = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

        let frame_sync = MPEGFrameSync::parse(raw_header)?;
        let version = MPEGVersion::parse(raw_header)?;
        let layer = MPEGLayer::parse(raw_header)?;
        let crc_protection = CRCProtection::parse(raw_header, data)?;
        let padding = FramePadding::parse(raw_header)?;
        let copyright = Copyright::parse(raw_header)?;
        let bitrate = FrameBitrate::parse(raw_header)?;
        let sample_rate = SampleRate::parse(raw_header)?;

        let private_bit = (raw_header & PRIVATE_BIT) >> PRIVATE_BIT_OFFSET;
        let channel_mode = (raw_header & CHANNEL_MODE) >> CHANNEL_MODE_OFFSET;
        let mode_extension = (raw_header & MODE_EXTENSION) >> MODE_EXTENSION_OFFSET;
        let original = (raw_header & ORIGINAL) >> ORIGINAL_OFFSET;
        let emphasis = (raw_header & EMPHASIS) >> EMPHASIS_OFFSET;

        let channel_mode = CHANNEL_MODE_TABLE[channel_mode as usize];
        let mode_extension = MODE_EXTENSION_TABLE[mode_extension as usize];
        let original = ORIGINAL_TABLE[original as usize];
        let emphasis = EMPHASIS_TABLE[emphasis as usize];

        let frame_length = {
            match bitrate {
                // If the bitrate is free, we need to search for the next frame sync.
                // This iterates through the bytes until a valid frame sync is found, marking the frame length.
                // Note that padding and CRC checksum byte count is not necessary if the frame length is calculated this way.
                FrameBitrate::Free => {
                    let i = 5;
                    while i < data.len() {
                        if MPEGFrameSync::has_frame_sync(&data[i..i + 4]) {
                            break;
                        }
                    }
                    i as u32
                }
                FrameBitrate::Bitrate(bitrate) => {
                    let padding = match padding {
                        FramePadding::Enabled => 1,
                        FramePadding::Disabled => 0,
                    };

                    let sample_rate = match sample_rate {
                        SampleRate::Hz44100 => 44100,
                        SampleRate::Hz48000 => 48000,
                        SampleRate::Hz32000 => 32000,
                    };

                    let crc_checksum = match crc_protection {
                        CRCProtection::Enabled { checksum } => 2,
                        CRCProtection::Disabled => 0,
                    };

                    ((144 * bitrate * 1000) / sample_rate) + padding + crc_checksum
                }
            }
        };

        let frame_data = data[0..frame_length as usize].to_vec();

        Ok(MPEGFrameHeader {
            raw_header,
            frame_data,
            frame_length,
            frame_sync,
            version,
            layer,
            crc_protection,
            bitrate,
            sample_rate,
            padding,
            private_bit: private_bit == 1,
            channel_mode,
            mode_extension,
            copyright,
            emphasis,
            original,
        })
    }
}

#[derive(Copy, Clone)]

pub enum MP3ModeExtension {
    Bands4To31,
    Bands8To31,
    Bands12To31,
    Bands16To31,
}

#[derive(Copy, Clone)]

pub enum Original {
    Original,
    Copy,
}

#[derive(Copy, Clone)]

pub enum MP3Emphasis {
    None,
    FiftyFifteen,
    Reserved,
    CCITJ17,
}

#[derive(Copy, Clone)]

pub enum MP3ChannelMode {
    Stereo,
    JointStereo,
    DualChannel,
    SingleChannel,
}
