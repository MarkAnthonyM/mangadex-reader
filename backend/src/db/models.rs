use super::schema::manga;

// Insertable struct representing manga item
#[derive(Insertable)]
#[table_name = "manga"]
pub struct NewManga {
    pub alt_names: Vec<String>,
    pub artists: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    // Find out what type this should be
    // pub chapters: Vec<Chapters>,
    pub comments: i32,
    pub cover_url: String,
    pub covers: Vec<String>,
    pub demographic: i32,
    pub description: String,
    pub follows: i32,
    pub genres: Vec<i32>,
    pub hentai: i32,
    pub lang_flag: String,
    pub lang_name: String,
    pub manga_id: String,
    pub status: i32,
    pub title: String,
    pub views: i32,
}

#[derive(Queryable, Serialize)]
pub struct Manga {
    pub id: i32,
    pub alt_names: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    // Find out what type this should be
    // pub chapters: Vec<Chapters>,
    pub comments: i32,
    pub cover_url: String,
    pub covers: Vec<String>,
    pub demographic: i32,
    pub description: String,
    pub follows: i32,
    pub genres: Vec<i32>,
    pub hentai: i32,
    pub lang_flag: String,
    pub lang_name: String,
    pub manga_id: String,
    pub status: i32,
    pub title: String,
    pub views: i32,
}