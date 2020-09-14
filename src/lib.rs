#[macro_use]
extern crate serde;

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
