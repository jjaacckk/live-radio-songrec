use radio_song_recognition::shazam_result::RecognizedTrack;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const STREAM_URL: &str = "https://streamer.kuci.org:8088/high"; // KUCI 88.9 FM

    let track: RecognizedTrack =
        radio_song_recognition::recognize_song_from_live_stream(STREAM_URL).await?;

    println!("{} by {}", track.song_name, track.artist_name);

    Ok(())
}
