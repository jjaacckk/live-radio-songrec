use radio_song_rec::shazam_result::RecognizedTrack;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const KUCI_STREAM_URL: &str = "https://streamer.kuci.org:8088/high"; // KUCI 88.9 FM, Irvine, CA
    const KXLU_STREAM_URL: &str = "http://kxlu.streamguys1.com/kxlu-hi?_ic2=1713223449915"; // KXLU 88.9 FM, Los Angeles, CA

    let track: RecognizedTrack =
        radio_song_rec::recognize_song_from_live_stream(KUCI_STREAM_URL).await?;

    println!("{} by {}", track.song_name, track.artist_name);
    println!(
        "\nOther Info:\n\talbum:\t{}\n\tdate:\t{}\n\tgenre:\t{}\n\tkey:\t{}",
        track.album_name.unwrap_or("null".to_string()),
        track.release_year.unwrap_or("null".to_string()),
        track.genre.unwrap_or("null".to_string()),
        track.track_key
    );

    Ok(())
}
