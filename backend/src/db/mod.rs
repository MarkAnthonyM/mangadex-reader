use diesel::{ prelude::*, pg::PgConnection };
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

use schema::manga;

// Establish connection to postgres database
pub fn establish_connection() -> PgConnection {
    // Load env file in root directory
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// Create new manga item and insert into database
pub fn create_manga(connection: &PgConnection, title: &str) {
    let manga = models::NewManga::create_mock_data(title);

    diesel::insert_into(schema::manga::table)
        .values(&manga)
        .execute(connection)
        .expect("Error inserting new manga");
}

// Query manga item from database
pub fn query_manga(connection: &PgConnection) -> Vec<models::Manga> {
    manga::table.load::<models::Manga>(connection).expect("Error loading manga")
}