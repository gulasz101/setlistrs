use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cover {
    pub url: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub cover: Option<Cover>,
    pub chords: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setlist {
    pub data: Vec<Song>,
}
