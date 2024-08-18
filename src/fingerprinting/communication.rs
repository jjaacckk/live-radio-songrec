// source: https://github.com/marin-m/SongRec/tree/master/src/fingerprinting/

use rand::seq::SliceRandom;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use std::error::Error;
use std::time::Duration;
use std::time::SystemTime;
use uuid::Uuid;

use crate::fingerprinting::signature_format::DecodedSignature;
use crate::fingerprinting::user_agent::USER_AGENTS;

pub async fn recognize_song_from_signature(
    signature: &DecodedSignature,
) -> Result<Value, Box<dyn Error>> {
    let timestamp_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();

    let post_data = json!({
        "geolocation": {
            "altitude": 0,
            "latitude": 39.09868,
            "longitude": 120.02344
        },
        "signature": {
            "samplems": (signature.number_samples as f32 / signature.sample_rate_hz as f32 * 1000.) as u32,
            "timestamp": timestamp_ms as u32,
            "uri": signature.encode_to_uri()?
        },
        "timestamp": timestamp_ms as u32,
        "timezone": "America/Los_Angeles"
    });

    let uuid_1 = Uuid::new_v4().to_hyphenated().to_string().to_uppercase();
    let uuid_2 = Uuid::new_v4().to_hyphenated().to_string();

    let url = format!(
        "https://amp.shazam.com/discovery/v5/en/US/android/-/tag/{}/{}",
        uuid_1, uuid_2
    );

    let mut headers = HeaderMap::new();

    headers.insert(
        "User-Agent",
        USER_AGENTS
            .choose(&mut rand::thread_rng())
            .unwrap()
            .parse()?,
    );
    headers.insert("Content-Language", "en_US".parse()?);

    // let client = reqwest::blocking::Client::new();
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .timeout(Duration::from_secs(20))
        .query(&[
            ("sync", "true"),
            ("webv3", "true"),
            ("sampling", "true"),
            ("connected", ""),
            ("shazamapiversion", "v3"),
            ("sharehub", "true"),
            ("video", "v3"),
        ])
        .headers(headers)
        .json(&post_data)
        .send()
        .await?;

    Ok(response.json().await?)
}

// pub fn obtain_raw_cover_image(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
//     let mut headers = HeaderMap::new();

//     headers.insert(
//         "User-Agent",
//         USER_AGENTS
//             .choose(&mut rand::thread_rng())
//             .unwrap()
//             .parse()?,
//     );
//     headers.insert("Content-Language", "en_US".parse()?);

//     let client = reqwest::blocking::Client::new();
//     let response = client
//         .get(url)
//         .timeout(Duration::from_secs(20))
//         .headers(headers)
//         .send()?;

//     Ok(response.bytes()?.as_ref().to_vec())
// }
