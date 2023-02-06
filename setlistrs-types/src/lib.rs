use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct YTLinkPersist {
    pub url: String,
    pub display_title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SongPersist {
    pub name: String,
    pub source: Vec<YTLinkPersist>,
    pub cover: Option<Vec<YTLinkPersist>>,
    pub chords: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setlist {
    pub data: Vec<SongPersist>,
}
