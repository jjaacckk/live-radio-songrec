// code from sources below with modification
// https://github.com/marin-m/SongRec/blob/master/src/core/thread_messages.rs
// https://github.com/marin-m/SongRec/blob/master/src/core/http_thread.rs

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct ShazamResult {
    pub location: Location,
    pub matches: Vec<Match>,
    pub tagid: String,
    pub timestamp: i64,
    pub timezone: String,
    pub track: Track,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Location {
    pub accuracy: f64,
    pub altitude: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Match {
    pub frequencyskew: f64,
    pub id: String,
    pub offset: f64,
    pub timeskew: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Track {
    pub albumadamid: String,
    pub artists: Vec<Artist>,
    pub genres: Value,
    pub highlightsurls: Value,
    pub hub: Value,
    pub images: Value,
    pub isrc: String,
    pub key: String,
    pub layout: String,
    pub sections: Value,
    pub share: Value,
    pub subtitle: String,
    pub title: String,
    pub r#type: String,
    pub url: String,
    pub urlparams: UrlParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    pub adamid: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UrlParams {
    #[serde(rename = "{trackartist}")]
    pub trackartist: String,
    #[serde(rename = "{tracktitle}")]
    pub tracktitle: String,
}

impl ShazamResult {
    pub fn create_from_result(
        json_object: serde_json::Value,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        match json_object["matches"].as_array() {
            Some(a) => {
                if a.len() == 0 {
                    return Ok(None);
                }
            }
            None => return Ok(None),
        }

        let result: ShazamResult = serde_json::from_value(json_object)?;

        Ok(Some(result))
    }
}
