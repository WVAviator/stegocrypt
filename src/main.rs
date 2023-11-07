mod mp3;

use clap::{command, Arg};
use std::path::Path;

fn main() {
    let matches = command!()
        .arg(Arg::new("input").required(true).index(1))
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();

    let path = Path::new(input);
    let raw_mp3_bytes = std::fs::read(path).unwrap();
    let mp3 = mp3::MP3::parse(raw_mp3_bytes).unwrap_or_else(|err| {
        panic!(
            "Encountered error while parsing MP3 file.\n{}",
            err.description()
        )
    });

    println!("Frames: {}", mp3.frames.len());
}
