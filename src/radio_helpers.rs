use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::result::Result;

use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::source::SourceStream;
use stream_download::storage::temp::TempStorageProvider;
use stream_download::{Settings, StreamDownload};

pub async fn download_12_seconds_of_audio_stream(
    stream_url: &str,
    download_path: &str,
) -> Result<(), Box<dyn Error>> {
    let stream = HttpStream::<Client>::create(stream_url.parse()?).await?;
    // let content_length = stream.content_length();

    let mut reader =
        StreamDownload::from_stream(stream, TempStorageProvider::default(), Settings::default())
            .await?;

    const TWELVE_SECONDS_IN_BYTES: usize = 491520;
    let mut buf: [u8; TWELVE_SECONDS_IN_BYTES] = [0; TWELVE_SECONDS_IN_BYTES];

    reader.read_exact(&mut buf)?;

    let mut file = File::create(download_path)?;
    file.write_all(&buf)?;

    Ok(())
}
