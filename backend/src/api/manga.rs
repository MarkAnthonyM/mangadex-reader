use reqwest;

#[derive(Deserialize)]
pub struct DexManga {
    id: i32,
    chapter: String,
    cover_url: String,
    group: String,
    timestamp: String,
}

impl DexManga {
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
    pub fn fill(self) -> Result<String, reqwest::Error> {
        let req = reqwest::blocking::get("https://www.mangadex.org/api/manga/42186")?
            .text();

        req
    }
}