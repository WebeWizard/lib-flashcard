// Test Deck CRUD operations
extern crate dotenv;
extern crate webe_auth;
extern crate webe_id;

use std::env;
use std::time::{Duration, SystemTime};

use lib_flashcard::FlashManager;
use webe_auth::session::Session;
use webe_auth::{AuthError, AuthManager, WebeAuth};
use webe_id::WebeIDFactory;

use std::sync::Mutex;

#[test]
fn deck_tests() {
    dotenv::dotenv().unwrap();
    // create the unique ID factory
    let node_id = 0u8;
    let epoch = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_millis(1546300800000)) // 01-01-2019 12:00:00 AM GMT
        .expect("failed to create custom epoch");
    let id_factory = Mutex::new(
        webe_id::WebeIDFactory::new(epoch, node_id).expect("Failed to create ID generator"),
    );

    // create the auth manager

    // create the flashcard manager

    // prepare the two test accounts

    // create a new deck with the first account

    // try to fetch the new deck with the first account

    // try to update the name with the first account

    // try to update the name with the second account

    // try to delete the deck with the second account

    // delete the deck with the first account
}

fn prepare_auth_manager(id_factory: &Mutex<WebeIDFactory>) -> WebeAuth {
    // create the email pool
    print!("Building Email Connection pool......");
    let smtp_address = env::var("SMTP_ADDRESS").expect("Failed to load SMTP Address from .env");
    let smtp_user = env::var("SMTP_USER").expect("Failed to load SMTP User from .env");
    let smtp_pass = env::var("SMTP_PASS").expect("Failed to load SMTP Password from .env");
    let email_pool = webe_auth::email::create_smtp_pool(smtp_address, smtp_user, smtp_pass)
        .expect("Failed to create SMTP pool");
    println!("Done");

    // create the database pool
    print!("Building Database Connection Pool......");
    let db_connect_string =
        env::var("DATABASE_URL").expect("Failed to load DB Connect string from .env");
    let db_pool = webe_auth::db::new_manager(db_connect_string)
        .expect("Failed to create Database connection pool");
    println!("Done");

    // create the auth manager
    webe_auth::WebeAuth {
        db_manager: db_pool,
        email_manager: email_pool,
        id_factory: id_factory,
    }
}

fn prepare_flash_manager(id_factory: &Mutex<WebeIDFactory>) -> FlashManager {
    // create the Flash database pool
    print!("Building FLASH Database Connection Pool......");
    let db_connect_string =
        env::var("FLASH_DATABASE_URL").expect("Failed to load Flash DB Connect string from .env");
    let flash_db_manager = webe_auth::db::new_manager(db_connect_string)
        .expect("Failed to create Flash Database connection pool");
    println!("Done");

    // create the flash manager
    lib_flashcard::FlashManager {
        db_manager: flash_db_manager,
        id_factory: id_factory,
    }
}

fn prepare_sessions(auth_manager: &WebeAuth) -> (Session, Session) {}
