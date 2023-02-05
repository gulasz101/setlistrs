use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct YTLink {
    pub url: String,
    pub display_title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Song {
    pub name: String,
    pub source: Vec<YTLink>,
    pub cover: Option<Vec<YTLink>>,
    pub chords: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setlist {
    pub data: Vec<Song>,
}
