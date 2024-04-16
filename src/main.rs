use live_radio_songrec::shazam_result::RecognizedTrack;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const KUCI_STREAM_URL: &str = "https://streamer.kuci.org:8088/high"; // KUCI 88.9 FM, Irvine, CA

    let track_option: Option<RecognizedTrack> =
        live_radio_songrec::recognize_song_from_live_stream(KUCI_STREAM_URL).await?;

    match track_option {
        Some(track) => {
            println!(
                "Match found:\n\t{} by {}",
                track.song_name, track.artist_name
            );
            println!(
                "\nAll Info:\n\ttitle:\t{}\n\tartist:\t{}\n\talbum:\t{}\n\tdate:\t{}\n\tgenre:\t{}\n\tkey:\t{}",
                track.song_name,
                track.artist_name,
                track.album_name.unwrap_or("null".to_string()),
                track.release_year.unwrap_or("null".to_string()),
                track.genre.unwrap_or("null".to_string()),
                track.track_key
            );
        }
        None => {
            println!("No match found.")
        }
    }

    Ok(())
}
