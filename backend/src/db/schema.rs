table! {
    manga (id) {
        id -> Int4,
        alt_names -> Nullable<Array<Text>>,
        artists -> Nullable<Array<Text>>,
        authors -> Nullable<Array<Text>>,
        comments -> Int4,
        cover_url -> Text,
        covers -> Array<Text>,
        demographic -> Int4,
        follows -> Int4,
        genres -> Array<Int4>,
        hentai -> Int4,
        lang_flag -> Text,
        lang_name -> Text,
        manga_description -> Text,
        manga_id -> Text,
        manga_status -> Int4,
        title -> Text,
        views -> Int4,
    }
}
