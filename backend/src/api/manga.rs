use reqwest;
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Manga {
    pub manga: MangaData,
    pub chapter: HashMap<u64, ChapterData>,
}

#[derive(Deserialize, Serialize)]
pub struct MangaData {
    pub alt_names: Vec<String>,
    pub artist: String,
    pub author: String,
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
}

// Represents chapter data associated with api manga struct
#[derive(Deserialize, Serialize)]
pub struct ChapterData {
    pub chapter: String,
    pub comments: i32,
    pub group_id: i32,
    pub group_id_2: i32,
    pub group_id_3: i32,
    pub group_name: Option<String>,
    pub group_name_2: Option<String>,
    pub group_name_3: Option<String>,
    pub lang_code: String,
    pub lang_name: String,
    pub time_stamp: i64,
    pub title: String,
    pub volume: String,
}

impl Manga {
    // Make request for manga, deserizalize json object data into struct.
    //TODO: Make async when async hits stable rocket version
    pub fn populate(id: &String) -> Result<Manga, reqwest::Error> {
        let request_url = format!("https://mangadex.org/api/manga/{}", id);
        let request = reqwest::blocking::get(request_url.as_str())?
            .json::<Manga>();

        request
    }
}