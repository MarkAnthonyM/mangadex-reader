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
    // Create mock data for testing purposes.
    pub fn create_mock() -> Self {
        DexManga {
            id: 0,
            chapter: "fake chapter".to_string(),
            cover_url: "fake cover".to_string(),
            group: "fake group".to_string(),
            timestamp: "fake timestamp".to_string(),
        }
    }

    // Make request for manga, fill struct with result. Currently sync, aim to make async.
    pub fn populate() -> Result<Manga, reqwest::Error> {
        let request = reqwest::blocking::get("https://www.mangadex.org/api/manga/42186")?
            .json::<Manga>();

        request
    }
}