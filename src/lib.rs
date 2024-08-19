mod fingerprinting {
    pub mod algorithm;
    pub mod communication;
    mod hanning;
    pub mod signature_format;
    mod user_agent;
}

// mod shazam_result_test;

mod radio_helpers;
pub mod shazam_result;

use std::error::Error;
use std::path::PathBuf;

use crate::fingerprinting::algorithm::SignatureGenerator;
use crate::fingerprinting::communication::recognize_song_from_signature;
use crate::fingerprinting::signature_format::DecodedSignature;

/// Downloads 12 seconds from stream_url and searches for match on Shazam
pub async fn recognize_song_from_live_stream(
    stream_url: &str,
) -> Result<Option<shazam_result::ShazamResult>, Box<dyn Error>> {
    let temp_dir_path: PathBuf = PathBuf::from("./temp/stream_data");

    let temp_file_path: PathBuf =
        radio_helpers::download_12_seconds_of_audio_stream(stream_url, &temp_dir_path).await?;

    let signature: DecodedSignature =
        SignatureGenerator::make_signature_from_file(&temp_file_path)?;

    let response = recognize_song_from_signature(&signature).await?;
    let track_result = shazam_result::ShazamResult::create_from_result(response);

    Ok(match track_result {
        Ok(track) => Some(track),
        Err(_) => None,
    })
}
