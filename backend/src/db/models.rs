use super::schema::manga;

#[derive(Insertable)]
#[table_name = "manga"]
pub struct NewManga<'a> {
    pub title: &'a str,
    pub authors: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
    pub genre_ids: Option<Vec<i32>>,
    pub genre_names: Option<Vec<String>>,
    pub url_link: String,
}

impl<'a> NewManga<'a> {
    pub fn create_mock_data(title: &'a str) -> Self {
        NewManga {
            title,
            authors: Some(vec![String::from("Fake Author")]),
            artists: Some(vec![String::from("Fake Artist")]),
            genre_ids: Some(vec![0]),
            genre_names: Some(vec![String::from("Fake Genre")]),
            url_link: String::from("https://www.fake.com"),
        }
    }
}

#[derive(Queryable)]
pub struct Manga {
    pub id: i32,
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
    pub genre_ids: Option<Vec<i32>>,
    pub genre_names: Option<Vec<String>>,
    pub url_link: String,
}