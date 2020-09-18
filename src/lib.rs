#[macro_use]
extern crate serde;

pub mod api;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manga {
    pub id: i32,
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
    pub genre_ids: Option<Vec<i32>>,
    pub genre_names: Option<Vec<String>>,
    pub url_link: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MangaJsonWrapper<T> {
    pub _type: String,
    pub id: String,
    pub attributes: T,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonApiResponse<T> {
    pub data: Vec<MangaJsonWrapper<T>>,
}