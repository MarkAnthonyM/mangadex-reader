use diesel::{ prelude::*, pg::PgConnection };
use dotenv::dotenv;
use std::env;

// Establish connection to postgres database
pub fn establish_connection() -> PgConnection {
    // Load env file in root directory
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
