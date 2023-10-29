mod mp3;

use clap::{command, Arg};
use std::path::Path;

fn main() {
    let matches = command!()
        .arg(Arg::new("input").required(true).index(1))
        .get_matches();

    // Gets a value for INPUT if supplied by the user
    let input = matches.get_one::<String>("input").unwrap();

    // Convert the string to a Path type
    let path = Path::new(input);

    // Now you can perform your operations on the path.
    if path.exists() {
        println!("The provided file path exists: {}", path.display());
        // TODO: Implement the main logic of your program here.
    } else {
        println!("The provided file path does not exist: {}", path.display());
    }

    let raw_mp3_bytes = std::fs::read(path).unwrap();

    let mp3 = mp3::MP3::parse(raw_mp3_bytes);

    println!("Frames: {}", mp3.frames.len());
}
