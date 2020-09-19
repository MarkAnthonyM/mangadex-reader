use reqwest;

#[derive(Deserialize, Serialize)]
pub struct Manga {
    pub manga: MangaData,
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

impl Manga {
    // Make request for manga, deserizalize json object data into struct.
    //TODO: Make async when async hits stable rocket version
    pub fn populate() -> Result<Manga, reqwest::Error> {
        let request = reqwest::blocking::get("https://www.mangadex.org/api/manga/42186")?
            .json::<Manga>();

        request
    }
}