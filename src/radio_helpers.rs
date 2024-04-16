use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;

use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::source::SourceStream;
use stream_download::storage::temp::TempStorageProvider;
use stream_download::{Settings, StreamDownload};

pub async fn download_12_seconds_of_audio_stream(
    stream_url: &str,
    temp_dir_path: &PathBuf,
) -> Result<PathBuf, Box<dyn Error>> {
    let stream = HttpStream::<Client>::create(stream_url.parse()?).await?;
    // let content_length = stream.content_length();

    let mut reader =
        StreamDownload::from_stream(stream, TempStorageProvider::default(), Settings::default())
            .await?;

    const TWELVE_SECONDS_IN_BYTES: usize = 491520;
    let mut buf: [u8; TWELVE_SECONDS_IN_BYTES] = [0; TWELVE_SECONDS_IN_BYTES];

    reader.read_exact(&mut buf)?;

    let stream_extension = match infer::get(&buf) {
        Some(k) => k.extension(),
        None => "mp3", // default
    };
    let mut final_download_path: PathBuf = temp_dir_path.clone();
    final_download_path.set_extension(stream_extension);

    // let final_download_path: PathBuf =
    //     PathBuf::from(format!("{}.{}", download_path, stream_extension));

    // println!("stream file name: {}", final_download_path.display());

    match final_download_path.parent() {
        Some(p) => fs::create_dir_all(p).unwrap(),
        None => (),
    }

    let mut file = File::create(&final_download_path)?;
    file.write_all(&buf)?;

    Ok(final_download_path)
}
