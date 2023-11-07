/// Error type for MPEG parsing.
#[derive(Debug, PartialEq)]
pub enum MPEGParseError {
    NoFrameSync,
    GenericInvalidFrameHeader { info: String },
}

impl MPEGParseError {
    pub fn description(&self) -> String {
        match self {
            MPEGParseError::NoFrameSync => {
                String::from("Header frame sync expected but not found.")
            }
            MPEGParseError::GenericInvalidFrameHeader { info } => {
                format!("Error parsing MPEG frame: {}", info)
            }
        }
    }
}
