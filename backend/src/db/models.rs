use super::schema::manga;

#[derive(Insertable)]
#[table_name = "manga"]
pub struct NewManga<'a> {
    pub title: &'a str,
    pub authors: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
    pub genre_ids: Option<Vec<u8>>,
    pub genre_names: Option<Vec<String>>,
    pub url_link: String,
}