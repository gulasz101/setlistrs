use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YTLink {
    pub url: String,
    pub display_title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct YTLinkDetails {
    pub id: i64,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct SongDetails {
    pub id: i64,
    pub name: String,
    pub source: Vec<YTLinkDetails>,
    pub cover: Option<Vec<YTLinkDetails>>,
    pub chords: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongList {
    pub data: Vec<(i64, Song)>,
}

#[derive(Serialize, Deserialize)]
pub struct Setlist {
    pub display_title: String,
    pub songs: Vec<(i64, SetlistSong)>,
}

#[derive(Serialize, Deserialize)]
pub struct SetlistSong {
    pub display_title: String,
    pub chords: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewSetlist {
    pub display_title: String,
    pub songs: Vec<i64>,
}
#[derive(Serialize, Deserialize)]
pub struct SetlistList {
    pub data: Vec<(i64, String)>,
}
