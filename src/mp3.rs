use self::{
    id3v2::ID3v2,
    mp3header::{MP3Header, MP3HeaderParseError},
};

mod id3v2;
mod mp3header;

pub struct MP3 {
    pub id3v2: Option<ID3v2>,
    // pub id3v1: Option<ID3v1>,
    pub frames: Vec<MP3Frame>,
}

pub struct MP3Frame {
    pub header: MP3Header,
    pub data: Vec<u8>,
}

impl MP3Frame {
    pub fn parse(data: Vec<u8>) -> Result<MP3Frame, MP3HeaderParseError> {
        let header = MP3Header::parse(data[0..4].to_vec())?;
        let data = data[4..header.frame_length() as usize].to_vec();

        Ok(MP3Frame { header, data })
    }
}

impl MP3 {
    pub fn parse(data: Vec<u8>) -> MP3 {
        let id3v2 = if ID3v2::has_id3v2_tag(data.clone()) {
            Some(ID3v2::parse(data.clone()).unwrap())
        } else {
            None
        };

        // let id3v1 = if MP3::has_id3v1_tag(data.clone()) {
        //     Some(ID3v1::parse(data.clone()))
        // } else {
        //     None
        // };

        let mut frames = Vec::new();

        let mut current_index = 0;
        if id3v2.is_some() {
            current_index = id3v2.as_ref().unwrap().size as usize;
        }

        while current_index < data.len() {
            let parsed_frame = MP3Frame::parse(data[current_index..].to_vec());
            if let Ok(frame) = parsed_frame {
                current_index += frame.header.frame_length() as usize;
                frames.push(frame);
            } else {
                println!("Failed to parse frame at index {}", current_index);
                current_index += 1;
            }
        }

        MP3 {
            id3v2,
            // id3v1,
            frames,
        }
    }
}
