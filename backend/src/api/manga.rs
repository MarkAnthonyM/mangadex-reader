#[derive(Deserialize)]
struct DexManga {
    id: i32,
    chapter: String,
    cover_url: String,
    group: String,
    timestamp: String,
}

impl DexManga {
    fn create_mock() -> Self {
        DexManga {
            id: 0,
            chapter: "fake chapter".to_string(),
            cover_url: "fake cover".to_string(),
            group: "fake group".to_string(),
            timestamp: "fake timestamp".to_string(),
        }
    }
}