pub struct ID3v2 {
    pub version: u16,
    pub size: u32,
    pub flags: u8,
    pub frames: Vec<ID3v2Frame>,
}

impl ID3v2 {
    pub fn parse(data: Vec<u8>) -> Result<ID3v2, ID3v2ParseError> {
        let identifier = String::from_utf8(data[0..3].to_vec()).unwrap();
        if identifier != "ID3" {
            println!("ID3v2 tag did is missing ID3 identifier.");
            return Err(ID3v2ParseError::InvalidHeader);
        }

        let version = u16::from_be_bytes([data[3], data[4]]);
        let flags = data[5];

        // the most significant bit in each Byte is set to 0 and ignored. Only remaining 7 bits are used. The reason is to avoid mismatch with audio frame header which has the first synchro Byte FF).
        // Eg. TAG len 257 is encoded as 00 00 02 01.
        let mut size = 0;
        for i in 0..4 {
            size <<= 7;
            size |= (data[6 + i]) as u32;
        }
        size += 10;

        let mut frames = Vec::new();
        let mut current_index = 10;

        println!("ID3v2 Metdata:");
        while current_index < size as usize {
            let frame = ID3v2Frame::parse(data[current_index..].to_vec());
            current_index += frame.size as usize;

            println!(
                " - {:?}: {}",
                frame.id,
                String::from_utf8(frame.data.clone()).unwrap(),
            );

            frames.push(frame);
        }

        Ok(ID3v2 {
            version,
            size,
            flags,
            frames,
        })
    }

    pub fn has_id3v2_tag(data: Vec<u8>) -> bool {
        let identifier = String::from_utf8(data[0..3].to_vec()).unwrap();
        identifier == "ID3"
    }
}

#[derive(Debug)]
pub enum ID3v2ParseError {
    InvalidHeader,
}

pub struct ID3v2Frame {
    pub id: FrameIdentifier,
    pub size: u32,
    pub flags: u16,
    pub data: Vec<u8>,
}

impl ID3v2Frame {
    pub fn parse(data: Vec<u8>) -> ID3v2Frame {
        let id = FrameIdentifier::parse(data[0..4].to_vec());
        let size = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) + 10;
        let flags = u16::from_be_bytes([data[8], data[9]]);
        let data = data[10..size as usize].to_vec();

        ID3v2Frame {
            id,
            size,
            flags,
            data,
        }
    }
}

#[derive(Debug)]
pub enum FrameIdentifier {
    TrackNumber,
    EncodedBy,
    URL,
    FrameIdentifier,
    OriginalArtist,
    Composer,
    Genre,
    Comments,
    Year,
    Album,
    Artist,
    SongName,
    Other(String),
}

impl FrameIdentifier {
    pub fn parse(bytes: Vec<u8>) -> FrameIdentifier {
        let id = String::from_utf8(bytes[0..4].to_vec()).unwrap();
        match id.as_str() {
            "TRCK" => FrameIdentifier::TrackNumber,
            "TENC" => FrameIdentifier::EncodedBy,
            "WXXX" => FrameIdentifier::URL,
            "TCOP" => FrameIdentifier::FrameIdentifier,
            "TOPE" => FrameIdentifier::OriginalArtist,
            "TCOM" => FrameIdentifier::Composer,
            "TCON" => FrameIdentifier::Genre,
            "COMM" => FrameIdentifier::Comments,
            "TYER" => FrameIdentifier::Year,
            "TALB" => FrameIdentifier::Album,
            "TPE1" => FrameIdentifier::Artist,
            "TIT2" => FrameIdentifier::SongName,
            _ => FrameIdentifier::Other(id.to_string()),
        }
    }
}
