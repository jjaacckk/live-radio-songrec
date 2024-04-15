// code from sources below with modification
// https://github.com/marin-m/SongRec/blob/master/src/core/thread_messages.rs
// https://github.com/marin-m/SongRec/blob/master/src/core/http_thread.rs

use serde_json::Value;
use std::error::Error;

pub struct RecognizedTrack {
    pub artist_name: String,
    pub album_name: Option<String>,
    pub song_name: String,
    // pub cover_image: Option<Vec<u8>>,
    // pub signature: Box<DecodedSignature>,

    // Used only in the CSV export for now:
    pub track_key: String,
    pub release_year: Option<String>,
    pub genre: Option<String>,
    // pub shazam_json: String,
}

impl RecognizedTrack {
    pub fn create_from_result(json_object: serde_json::Value) -> Result<Self, Box<dyn Error>> {
        let mut album_name: Option<String> = None;
        let mut release_year: Option<String> = None;

        // Sometimes the idea of trying to write functional poetry hurts

        if let Value::Array(sections) = &json_object["track"]["sections"] {
            for section in sections {
                if let Value::String(string) = &section["type"] {
                    if string == "SONG" {
                        if let Value::Array(metadata) = &section["metadata"] {
                            for metadatum in metadata {
                                if let Value::String(title) = &metadatum["title"] {
                                    if title == "Album" {
                                        if let Value::String(text) = &metadatum["text"] {
                                            album_name = Some(text.to_string());
                                        }
                                    } else if title == "Released" {
                                        if let Value::String(text) = &metadatum["text"] {
                                            release_year = Some(text.to_string());
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        Ok(RecognizedTrack {
            artist_name: match &json_object["track"]["subtitle"] {
                Value::String(string) => string.to_string(),
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        gettextrs::gettext("No match for this song").as_str(),
                    )))
                }
            },
            album_name: album_name,
            song_name: match &json_object["track"]["title"] {
                Value::String(string) => string.to_string(),
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        gettextrs::gettext("No match for this song").as_str(),
                    )))
                }
            },
            // cover_image: match &json_object["track"]["images"]["coverart"] {
            //     Value::String(string) => Some(obtain_raw_cover_image(string)?),
            //     _ => None,
            // },
            // signature: Box::new(signature),
            track_key: match &json_object["track"]["key"] {
                Value::String(string) => string.to_string(),
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        gettextrs::gettext("No match for this song").as_str(),
                    )))
                }
            },
            release_year: release_year,
            genre: match &json_object["track"]["genres"]["primary"] {
                Value::String(string) => Some(string.to_string()),
                _ => None,
            },
            // shazam_json: Regex::new("\n *")
            //     .unwrap()
            //     .replace_all(
            //         &Regex::new("([,:])\n *")
            //             .unwrap()
            //             .replace_all(&to_string_pretty(&json_object).unwrap(), "$1 ")
            //             .into_owned(),
            //         "",
            //     )
            //     .into_owned(),
        })
    }
}
