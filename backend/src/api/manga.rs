#[derive(Deserialize)]
struct DexManga {
    id: String,
    chapter: String,
    cover_url: String,
    group: String,
    timestamp: String,
}

impl DexManga {
    fn create_mock() -> Self {
        DexManga {
            id: "fake id",
            chapter: "fake chapter",
            cover_url: "fake cover",
            group: "fake group",
            timestamp: "fake timestamp",
        }
    }
}