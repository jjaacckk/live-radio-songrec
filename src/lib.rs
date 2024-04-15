mod fingerprinting {
    pub mod algorithm;
    pub mod communication;
    mod hanning;
    pub mod signature_format;
    mod user_agent;
}

mod radio_helpers;
pub mod shazam_result;

use std::error::Error;

use crate::fingerprinting::algorithm::SignatureGenerator;
use crate::fingerprinting::communication::recognize_song_from_signature;
use crate::fingerprinting::signature_format::DecodedSignature;

pub async fn recognize_song_from_live_stream(
    stream_url: &str,
) -> Result<shazam_result::RecognizedTrack, Box<dyn Error>> {
    /// Downloads 12 seconds from stream_url and searches for match on Shazam
    ///

    const BUFFER_FILE_PATH: &str = "./rsrc/data.mp3";

    radio_helpers::download_12_seconds_of_audio_stream(stream_url, BUFFER_FILE_PATH).await?;
    let signature: DecodedSignature =
        SignatureGenerator::make_signature_from_file(BUFFER_FILE_PATH)?;

    let response = recognize_song_from_signature(&signature).await?;
    let track: shazam_result::RecognizedTrack =
        shazam_result::RecognizedTrack::create_from_result(response)?;

    Ok(track)
}
