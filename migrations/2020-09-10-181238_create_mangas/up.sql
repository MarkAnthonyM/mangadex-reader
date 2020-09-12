CREATE TABLE manga (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    authors TEXT [],
    artists TEXT [],
    genre_ids INT [],
    genre_names TEXT [],
    url_link TEXT NOT NULL
)