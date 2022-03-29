use std::{
    fmt::Display, 
    convert::From,
};
use select::{
    document::Document,
    predicate::{Name, Predicate}
};
use serde_json::Value;

use crate::{ 
    media::Media, 
    defs::{ YOUTUBE_BASE_URI, YT_STRIP_LEN, YT_STRIP_PREFIX }
};

pub enum WebTargets {
    YouTube,
}
impl Display for WebTargets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebTargets::YouTube => write!(f, " "),
            //_ => write!(f, "ﮊ ")
        }
    }
}
impl Default for WebTargets {
    fn default() -> Self {
        WebTargets::YouTube
    }
}

impl WebTargets {
    /// ## WebTargets::parsedoc() -::- `hacky`
    /// Parses a DOM to extract media.
    pub fn parsedoc<X: From<String> + Display + Clone>(&self, document: Document, first: bool) -> Option<Vec<Media<X>>> {
        match self {
            WebTargets::YouTube => {
                let mut medias: Vec<Media<X>> = Vec::new();
                let frame = document.find(Name("body").descendant(Name("script")));
                for elem in frame {
                    let txt = elem.text();
                    if txt.len() > YT_STRIP_LEN && &txt.trim_start()[..YT_STRIP_LEN] == YT_STRIP_PREFIX {
                        let raw = txt.trim()[YT_STRIP_LEN + 3..(txt.trim().len() - 1)].to_owned();
                        let json: Value = serde_json::from_str(&raw).expect("Failed to serialize json");
                        // HACK: This indexing chain is the uncertain part. If youtube changes its structure this bit will be the one to get affected.
                        // HACK: The main goal of this chain is to reach to an Array with elements as `videoRenderer`, `playlistRenderer`.
                        let results: &Value = &json["contents"]
                                                    ["twoColumnSearchResultsRenderer"]
                                                     ["primaryContents"]
                                                      ["sectionListRenderer"]
                                                       ["contents"];
                        match results {
                            // NOTE: Proceed parsing only if res was a json array.
                            Value::Array(res) => {
                                for obj in res {
                                    match &obj["itemSectionRenderer"]["contents"] {
                                        Value::Array(res2) => {
                                            for item in res2 {
                                                if item["videoRenderer"].is_object() {
                                                    let vrender = &item["videoRenderer"];
                                                    // println!("{:#?}\n--------", vrender["title"]);
                                                    // Alternate more descriptive title; includes channel name, duration & views
                                                    // let _title = &obj["videoRenderer"]["title"]["accessibility"]["accessibilityData"]["label"];
                                                    let unquote = |s: String| s[1..s.len()-1].to_string();
                                                    let _title = &vrender["title"]["runs"][0]["text"];
                                                    let watchv = &vrender["videoId"];
                                                    if _title.is_string() && watchv.is_string() {
                                                        let wv = watchv.to_string();
                                                        let url = format!("{}/watch?v={}", YOUTUBE_BASE_URI, &wv[1..wv.len()-1]).into();
                                                        let tt = _title.to_string();
                                                        let title = unquote(tt).into();
                                                        //println!("{}\n{}\n--------", title, url);
                                                        medias.push(Media::new(title, url));
                                                    }
                                                    if first { break }
                                                }
                                            }
                                        },
                                        _ => {}/*eprintln!("Not a videolist")*/,
                                    }
                                }
                            },
                            _ => eprintln!("Structurization-failure :: Failed to read json!"),
                            
                        }
                    }
                }
                Some(medias)
            },
            _ => None,
        }
    }
}
