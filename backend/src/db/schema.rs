table! {
    mangas (id) {
        id -> Int4,
        title -> Text,
        authors -> Nullable<Array<Text>>,
        artists -> Nullable<Array<Text>>,
        genre_ids -> Nullable<Array<Int4>>,
        genre_names -> Nullable<Array<Text>>,
        url_link -> Text,
    }
}
