use reqwest;
use serde:: Deserialize;

#[derive(Deserialize, Serialize)]
pub struct Manga {
    pub manga: MangaData,
}

#[derive(Deserialize, Serialize)]
pub struct MangaData {
    pub artist: String,
    pub author: String,
    pub cover_url: String,
    pub title: String,
}

impl Manga {
    // Make request for manga, fill struct with result. Currently sync, aim to make async.
    pub fn populate() -> Result<Manga, reqwest::Error> {
        let request = reqwest::blocking::get("https://www.mangadex.org/api/manga/42186")?
            .json::<Manga>();

        request
    }
}