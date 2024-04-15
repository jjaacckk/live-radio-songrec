mod fingerprinting {
    pub mod algorithm;
    pub mod communication;
    mod hanning;
    pub mod signature_format;
    mod user_agent;
}

mod radio_helpers;
mod shazam_result;

use std::error::Error;

use crate::fingerprinting::algorithm::SignatureGenerator;
use crate::fingerprinting::communication::recognize_song_from_signature;
use crate::fingerprinting::signature_format::DecodedSignature;

async fn match_audio(buffer_file_path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let signature: DecodedSignature =
        SignatureGenerator::make_signature_from_file(buffer_file_path)?;

    recognize_song_from_signature(&signature).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const STREAM_URL: &str = "https://streamer.kuci.org:8088/high"; // KUCI 88.9 FM

    const BUFFER_FILE_PATH: &str = "./rsrc/data.mp3";

    println!("downloading 12 seconds from {}....", STREAM_URL);

    radio_helpers::download_12_seconds_of_audio_stream(STREAM_URL, BUFFER_FILE_PATH).await?;

    println!("matching audio....");

    let response = match_audio(BUFFER_FILE_PATH).await?;

    let track: shazam_result::SongRecognizedMessage = shazam_result::format_track_result(response)?;

    println!("{} by {}", track.song_name, track.artist_name);

    // match response_value {
    //     Ok(v) => {
    //         // println!("{:?}", v["track"]);
    //         if v["track"] != serde_json::Value::Null {
    //             println!(
    //                 "track: {} by {}",
    //                 v["track"]["title"].as_str().unwrap(),
    //                 v["track"]["subtitle"].as_str().unwrap()
    //             );
    //         } else {
    //             println!("no track found.")
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("error: {e}");
    //     }
    // }

    Ok(())
}
