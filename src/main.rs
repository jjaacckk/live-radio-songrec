use live_radio_songrec::shazam_result::ShazamResult;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const KUCI_STREAM_URL: &str = "https://streamer.kuci.org:8088/high"; // KUCI 88.9 FM, Irvine, CA

    match live_radio_songrec::recognize_song_from_live_stream(KUCI_STREAM_URL, "./temp/stream_data")
        .await
    {
        Ok(result) => {
            println!(
                "Match found:\n\t{} by {}",
                result.track.title, result.track.subtitle
            );
            // println!("{:?}", result);
            // println!(
            //     "\nAll Info:\n\ttitle:\t{}\n\tartist:\t{}\n\talbum:\t{}\n\tdate:\t{}\n\tgenre:\t{}\n\tkey:\t{}",
            //     track.title,
            //     track.subtitle,
            //     artist,
            //     track..unwrap_or("null".to_string()),
            //     track.genre.unwrap_or("null".to_string()),
            //     track.track_key
            // );
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    Ok(())
}
