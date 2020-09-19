#[macro_use]
extern crate serde;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manga {
    pub alt_names: Vec<String>,
    pub artists: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub comments: i32,
    pub cover_url: String,
    pub covers: Vec<String>,
    pub demographic: i32,
    pub description: String,
    pub follows: i32,
    // Maybe use an enum here
    pub genres: Vec<i32>,
    pub hentai: i32,
    pub lang_flag: String,
    pub lang_name: String,
    // Figure out proper field datatypes
    // pub links: MangaLinks
    // pub rating: MangaRating,
    // pub related: SomeType
    pub status: i32,
    pub title: String,
    pub views: i32,
    // These fields may be unnecessary
    // pub genre_ids: Option<Vec<i32>>,
    // pub genre_names: Option<Vec<String>>,
    // pub url_link: String,
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