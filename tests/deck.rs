// Test Deck CRUD operations
extern crate dotenv;
extern crate webe_auth;
extern crate webe_id;

use std::env;
use std::time::{Duration, SystemTime};

use lib_flashcard::{FlashError, FlashManager};
use webe_auth::session::Session;
use webe_auth::{AuthManager, WebeAuth};
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
    let auth_manager = prepare_auth_manager(&id_factory);

    // create the flashcard manager
    let flash_manager = prepare_flash_manager(&id_factory);

    // prepare the three test accounts - valid, fake, expired
    let (valid, fake, expired) = prepare_sessions(&auth_manager);

    // verify that you can't create a deck with an expired account
    match flash_manager.create_deck(&expired, "expired_test".to_owned()) {
        Ok(_wut) => panic!("was able to create a deck with expired account"),
        Err(error) => match error {
            FlashError::SessionTimeout => {}
            _ => panic!("recieved an unexpected error"),
        },
    }

    // create a new deck with the valid account
    let deck = flash_manager
        .create_deck(&valid, "valid_test".to_owned())
        .unwrap();

    // try to fetch the new deck with the valid account
    if flash_manager.get_decks_for_session(&valid).unwrap().len() != 1 {
        panic!("should have only returned 1 deck for valid account");
    }
    // try to fetch the new deck with the fake account
    if flash_manager.get_decks_for_session(&fake).unwrap().len() != 0 {
        panic!("should have only returned 0 decks for fake account");
    }
    // try to fetch the new deck with the expired account
    match flash_manager.get_decks_for_session(&expired) {
        Ok(_wut) => panic!("was able to create a deck with expired account"),
        Err(error) => match error {
            FlashError::SessionTimeout => {}
            _ => panic!("recieved an unexpected error"),
        },
    }

    // try to update the name with the valid account
    flash_manager
        .rename_deck(&valid, deck.id, "updated_test")
        .unwrap();

    // try to update the name with the fake account
    match flash_manager.rename_deck(&fake, deck.id, "fake_test") {
        Ok(_wut) => panic!("was able to rename a deck with fake account"),
        Err(error) => match error {
            FlashError::PermissionError => {}
            _ => {
                dbg!(error);
                panic!("recieved an unexpected error")
            }
        },
    }

    // try to update with expired account
    match flash_manager.rename_deck(&expired, deck.id, "expired_test") {
        Ok(_wut) => panic!("was able to rename a deck with expired account"),
        Err(error) => match error {
            FlashError::SessionTimeout => {}
            _ => {
                dbg!(error);
                panic!("recieved an unexpected error")
            }
        },
    }

    // try to delete with the expired account
    match flash_manager.delete_deck(&expired, deck.id) {
        Ok(_wut) => panic!("was able to delete a deck with expired account"),
        Err(error) => match error {
            FlashError::SessionTimeout => {}
            _ => {
                dbg!(error);
                panic!("recieved an unexpected error")
            }
        },
    }

    // try to delete the deck with the fake account
    match flash_manager.delete_deck(&fake, deck.id) {
        Ok(_wut) => panic!("was able to delete a deck with fake account"),
        Err(error) => match error {
            FlashError::PermissionError => {}
            _ => {
                dbg!(error);
                panic!("recieved an unexpected error")
            }
        },
    }

    // delete the deck with the valid account
    flash_manager.delete_deck(&valid, deck.id).unwrap();
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
    print!("Building Auth Database Connection Pool......");
    let db_connect_string =
        env::var("AUTH_DATABASE_URL").expect("Failed to load DB Connect string from .env");
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
    print!("Building Flash Database Connection Pool......");
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

fn prepare_sessions(auth_manager: &WebeAuth) -> (Session, Session, Session) {
    let valid_email = "valid";
    let fake_email = "fake";
    let pass = "test";

    create_and_verify_account(auth_manager, valid_email, pass);
    create_and_verify_account(auth_manager, fake_email, pass);

    let valid_session = auth_manager
        .login(&valid_email.to_owned(), &pass.to_owned())
        .unwrap();
    let fake_session = auth_manager
        .login(&fake_email.to_owned(), &pass.to_owned())
        .unwrap();
    let mut expired_session = auth_manager
        .login(&valid_email.to_owned(), &pass.to_owned())
        .unwrap();
    expired_session.timeout = 0;
    return (valid_session, fake_session, expired_session);
}

fn create_and_verify_account(auth_manager: &WebeAuth, email: &str, pass: &str) {
    // if the email is in use, delete it (cleanup from previous test)
    if let Ok(existing) = auth_manager.find_by_email(&email.to_owned()) {
        auth_manager.delete_account(existing).unwrap();
    }

    let account = auth_manager
        .create_account(email.to_owned(), pass.to_owned())
        .unwrap();

    auth_manager
        .verify_account(
            &email.to_owned(),
            &pass.to_owned(),
            &account.verify_code.unwrap(),
        )
        .unwrap();
}
