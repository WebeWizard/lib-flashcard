// Test GAME CRUD operations
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
fn game_tests() {
  dotenv::dotenv().unwrap();
  // create the unique ID factory
  let node_id = 0u8;
  let epoch = SystemTime::UNIX_EPOCH
    .checked_add(Duration::from_millis(1546300800000)) // 01-01-2019 12:00:00 AM GMT
    .expect("failed to create custom epoch");
  let id_factory =
    Mutex::new(webe_id::WebeIDFactory::new(epoch, node_id).expect("Failed to create ID generator"));

  // create the auth manager
  let auth_manager = prepare_auth_manager(&id_factory);

  // create the flashcard manager
  let flash_manager = prepare_flash_manager(&id_factory);

  // prepare the three test accounts - valid, fake, expired
  let (valid, fake, expired) = prepare_sessions(&auth_manager);

  // create deck
  let deck = flash_manager
    .create_deck(&valid, "valid_test".to_owned())
    .unwrap();

  // create two cards in the deck
  let card = flash_manager
    .create_card(&valid, deck.id, 1, "Q".to_owned(), "A".to_owned())
    .expect("failed to create first card");

  // verify you can't update score using a fake account
  match flash_manager.update_score(&fake, card.id, 1) {
    Ok(_) => panic!("should not be able to update a scure using fake account"),
    Err(error) => match error {
      FlashError::PermissionError => {}
      _ => {
        dbg!(error);
        panic!("recieved an unexpected error")
      }
    },
  }

  // verify you can't update score using an expired account
  match flash_manager.update_score(&expired, card.id, 1) {
    Ok(_) => panic!("should not be able to update a scure using expired account"),
    Err(error) => match error {
      FlashError::SessionTimeout => {}
      _ => {
        dbg!(error);
        panic!("recieved an unexpected error")
      }
    },
  }

  // verify you can update score using a valid account
  flash_manager.update_score(&valid, card.id, 1).unwrap();

  // verify you can't get scores using a fake account
  match flash_manager.get_deck_scores(&fake, deck.id) {
    Ok(_) => panic!("should not be able to get deck scores using fake account"),
    Err(error) => match error {
      FlashError::PermissionError => {}
      _ => {
        dbg!(error);
        panic!("recieved an unexpected error")
      }
    },
  }

  // verify you can't get scores using an expired account
  match flash_manager.get_deck_scores(&expired, deck.id) {
    Ok(_) => panic!("should not be able to get deck scores using expired account"),
    Err(error) => match error {
      FlashError::SessionTimeout => {}
      _ => {
        dbg!(error);
        panic!("recieved an unexpected error")
      }
    },
  }

  // verify you can get scores using a valid account
  let scores = flash_manager.get_deck_scores(&valid, deck.id).unwrap();
  assert_eq!(scores.len(), 1);
  assert_eq!(scores[0].score, 1);

  // clean up the accounts
  delete_account(&auth_manager, "valid");
  delete_account(&auth_manager, "fake");
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
  println!("Preparing test sessions......");
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
  println!("Done");
  return (valid_session, fake_session, expired_session);
}

fn create_and_verify_account(auth_manager: &WebeAuth, email: &str, pass: &str) {
  print!("Creating and Verifying test account: {}......", email);
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
  println!("Done");
}

fn delete_account(auth_manager: &WebeAuth, email: &str) {
  let account = auth_manager.find_by_email(&email.to_owned()).unwrap();
  auth_manager.delete_account(account).unwrap();
}
