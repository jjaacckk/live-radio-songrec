// code from sources below with modification
// https://github.com/marin-m/SongRec/blob/master/src/core/thread_messages.rs
// https://github.com/marin-m/SongRec/blob/master/src/core/http_thread.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub fn create_from_result(json_object: serde_json::Value) -> Result<Self, Box<dyn Error>> {
        let result: ShazamResult = json_object;

        Ok(result)
        // let mut album_name: Option<String> = None;
        // let mut release_year: Option<String> = None;

        // // Sometimes the idea of trying to write functional poetry hurts

        // if let Value::Array(sections) = &json_object["track"]["sections"] {
        //     for section in sections {
        //         if let Value::String(string) = &section["type"] {
        //             if string == "SONG" {
        //                 if let Value::Array(metadata) = &section["metadata"] {
        //                     for metadatum in metadata {
        //                         if let Value::String(title) = &metadatum["title"] {
        //                             if title == "Album" {
        //                                 if let Value::String(text) = &metadatum["text"] {
        //                                     album_name = Some(text.to_string());
        //                                 }
        //                             } else if title == "Released" {
        //                                 if let Value::String(text) = &metadatum["text"] {
        //                                     release_year = Some(text.to_string());
        //                                 }
        //                             }
        //                         }
        //                     }
        //                     break;
        //                 }
        //             }
        //         }
        //     }
        // }

        // Ok(RecognizedTrack {
        //     artist_name: match &json_object["track"]["subtitle"] {
        //         Value::String(string) => string.to_string(),
        //         _ => {
        //             return Err(Box::new(std::io::Error::new(
        //                 std::io::ErrorKind::Other,
        //                 "No match for this song",
        //             )))
        //         }
        //     },
        //     album_name: album_name,
        //     song_name: match &json_object["track"]["title"] {
        //         Value::String(string) => string.to_string(),
        //         _ => {
        //             return Err(Box::new(std::io::Error::new(
        //                 std::io::ErrorKind::Other,
        //                 "No match for this song",
        //             )))
        //         }
        //     },
        //     // cover_image: match &json_object["track"]["images"]["coverart"] {
        //     //     Value::String(string) => Some(obtain_raw_cover_image(string)?),
        //     //     _ => None,
        //     // },
        //     // signature: Box::new(signature),
        //     track_key: match &json_object["track"]["key"] {
        //         Value::String(string) => string.to_string(),
        //         _ => {
        //             return Err(Box::new(std::io::Error::new(
        //                 std::io::ErrorKind::Other,
        //                 "No match for this song",
        //             )))
        //         }
        //     },
        //     release_year: release_year,
        //     genre: match &json_object["track"]["genres"]["primary"] {
        //         Value::String(string) => Some(string.to_string()),
        //         _ => None,
        //     },
        //     // shazam_json: Regex::new("\n *")
        //     //     .unwrap()
        //     //     .replace_all(
        //     //         &Regex::new("([,:])\n *")
        //     //             .unwrap()
        //     //             .replace_all(&to_string_pretty(&json_object).unwrap(), "$1 ")
        //     //             .into_owned(),
        //     //         "",
        //     //     )
        //     //     .into_owned(),
        // })
    }
}
