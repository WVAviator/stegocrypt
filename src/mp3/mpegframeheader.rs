use self::crcprotection::CRCProtection;
use self::mpegframesync::MPEGFrameSync;
use self::mpeglayer::MPEGLayer;
use self::mpegversion::MPEGVersion;

use super::mpegparserror::MPEGParseError;

mod crcprotection;
mod mpegframesync;
mod mpeglayer;
mod mpegversion;

const BITRATE_INDEX: u32 = 0b00000000_00000000_11110000_00000000;
const BITRATE_INDEX_OFFSET: u32 = 12;
const BITRATE_TABLE: [u32; 16] = [
    0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0,
];

const SAMPLE_RATE_INDEX: u32 = 0b00000000_00000000_00001100_00000000;
const SAMPLE_RATE_INDEX_OFFSET: u32 = 10;
const SAMPLE_RATE_TABLE: [u32; 4] = [44100, 48000, 32000, 0];

const PADDING: u32 = 0b00000000_00000000_00000010_00000000;
const PADDING_OFFSET: u32 = 9;
const PADDING_TABLE: [Padding; 2] = [Padding::Disabled, Padding::Enabled];

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

const COPYRIGHT: u32 = 0b00000000_00000000_00000000_00001000;
const COPYRIGHT_OFFSET: u32 = 3;
const COPYRIGHT_TABLE: [Copyright; 2] = [Copyright::CopyPermitted, Copyright::CopyPermitted];

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
    pub frame_sync: MPEGFrameSync,
    pub version: MPEGVersion,
    pub layer: MPEGLayer,
    pub crc_protection: CRCProtection,
    pub bitrate: u32,
    pub sample_rate: u32,
    pub padding: Padding,
    pub private_bit: bool,
    pub channel_mode: MP3ChannelMode,
    pub mode_extension: MP3ModeExtension,
    pub copyright: Copyright,
    pub original: Original,
    pub emphasis: MP3Emphasis,
}

impl MPEGFrameHeader {
    pub fn parse(data: Vec<u8>) -> Result<MPEGFrameHeader, MPEGParseError> {
        let raw_header = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

        let frame_sync = MPEGFrameSync::parse(raw_header)?;
        let version = MPEGVersion::parse(raw_header)?;
        let layer = MPEGLayer::parse(raw_header)?;
        let crc_protection = CRCProtection::parse(raw_header, &data)?;

        let bitrate_index = (raw_header & BITRATE_INDEX) >> BITRATE_INDEX_OFFSET;
        let sample_rate_index = (raw_header & SAMPLE_RATE_INDEX) >> SAMPLE_RATE_INDEX_OFFSET;
        let padding = (raw_header & PADDING) >> PADDING_OFFSET;
        let private_bit = (raw_header & PRIVATE_BIT) >> PRIVATE_BIT_OFFSET;
        let channel_mode = (raw_header & CHANNEL_MODE) >> CHANNEL_MODE_OFFSET;
        let mode_extension = (raw_header & MODE_EXTENSION) >> MODE_EXTENSION_OFFSET;
        let copyright = (raw_header & COPYRIGHT) >> COPYRIGHT_OFFSET;
        let original = (raw_header & ORIGINAL) >> ORIGINAL_OFFSET;
        let emphasis = (raw_header & EMPHASIS) >> EMPHASIS_OFFSET;

        let bitrate = BITRATE_TABLE[bitrate_index as usize];
        let sample_rate = SAMPLE_RATE_TABLE[sample_rate_index as usize];
        let padding = PADDING_TABLE[padding as usize];
        let channel_mode = CHANNEL_MODE_TABLE[channel_mode as usize];
        let mode_extension = MODE_EXTENSION_TABLE[mode_extension as usize];
        let copyright = COPYRIGHT_TABLE[copyright as usize];
        let original = ORIGINAL_TABLE[original as usize];
        let emphasis = EMPHASIS_TABLE[emphasis as usize];

        Ok(MPEGFrameHeader {
            raw_header,
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

    pub fn frame_length(&self) -> u32 {
        // Formula for counting frame length in Bytes:
        // FrameLen = int((144 * BitRate / SampleRate ) + Padding + CRC);

        let padding = match self.padding {
            Padding::Enabled => 1,
            Padding::Disabled => 0,
        };

        let crc_checksum = match self.crc_protection {
            CRCProtection::Enabled { checksum } => 2,
            CRCProtection::Disabled => 0,
        };

        ((144 * self.bitrate * 1000) / self.sample_rate) + padding + crc_checksum
    }
}

#[derive(Copy, Clone)]

pub enum Padding {
    Enabled,
    Disabled,
}

#[derive(Copy, Clone)]

pub enum MP3ModeExtension {
    Bands4To31,
    Bands8To31,
    Bands12To31,
    Bands16To31,
}

#[derive(Copy, Clone)]

pub enum Copyright {
    CopyPermitted,
    CopyForbidden,
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
