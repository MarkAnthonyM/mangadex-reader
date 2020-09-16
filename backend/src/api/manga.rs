#[derive(Deserialize)]
struct Manga {
    id: String,
    chapter: String,
    cover_url: String,
    group: String,
    timestamp: String,
}

impl Manga {
    fn create_mock() -> Self {
        Manga {
            id: "fake id",
            chapter: "fake chapter",
            cover_url: "fake cover",
            group: "fake group",
            timestamp: "fake timestamp",
        }
    }
}