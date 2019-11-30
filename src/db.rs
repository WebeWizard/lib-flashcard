// This module contains database CRUD operations for each of the models.

use crate::card::Card;
use crate::deck::Deck;

use diesel::prelude::*;
use diesel::r2d2 as diesel_r2d2;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum DBApiError {
  AlreadyExists,           // deck with given email address already exists
  OtherError(DieselError), // errors from interacting with database
  BadVerifyCode,
  PoolError(r2d2::Error),
  NotFound,
}

impl From<DBApiError> for crate::FlashError {
  fn from(err: DBApiError) -> crate::FlashError {
    crate::FlashError::DBError(err)
  }
}

impl From<r2d2::Error> for DBApiError {
  fn from(err: r2d2::Error) -> DBApiError {
    DBApiError::PoolError(err)
  }
}

impl From<DieselError> for DBApiError {
  fn from(err: DieselError) -> DBApiError {
    match err {
      DieselError::NotFound => DBApiError::NotFound,
      _ => DBApiError::OtherError(err),
    }
  }
}

pub type DBManager = diesel_r2d2::Pool<diesel_r2d2::ConnectionManager<MysqlConnection>>;

pub fn new_manager(connect_string: String) -> Result<DBManager, DBApiError> {
  let connection_manager = ConnectionManager::new(connect_string.as_str());
  // build the database connection pool
  let pool = Pool::builder().max_size(10).build(connection_manager)?;
  return Ok(pool);
}

pub trait DeckApi {
  fn insert(&self, deck: &Deck) -> Result<(), DBApiError>;

  fn find(&self, id: &u64) -> Result<Deck, DBApiError>;

  fn update(&self, deck: &Deck) -> Result<(), DBApiError>;

  // TODO: need way to update email and password

  fn delete(&self, deck: Deck) -> Result<(), DBApiError>;
}

impl DeckApi for DBManager {
  fn insert(&self, deck: &Deck) -> Result<(), DBApiError> {
    unimplemented!()
  }

  fn find(&self, id: &u64) -> Result<Deck, DBApiError> {
    unimplemented!()
  }

  fn update(&self, deck: &Deck) -> Result<(), DBApiError> {
    unimplemented!()
  }

  // TODO: need way to update email and password

  fn delete(&self, deck: Deck) -> Result<(), DBApiError> {
    unimplemented!()
  }
}

pub trait CardApi {
  fn insert(&self, card: &Card) -> Result<(), DBApiError>;

  fn find(&self, token: &str) -> Result<Card, DBApiError>;

  fn update(&self, card: &Card) -> Result<(), DBApiError>;

  fn delete(&self, token: &str) -> Result<(), DBApiError>;
}

impl CardApi for DBManager {
  fn insert(&self, card: &Card) -> Result<(), DBApiError> {
    unimplemented!()
  }

  fn find(&self, token: &str) -> Result<Card, DBApiError> {
    unimplemented!()
  }

  fn update(&self, card: &Card) -> Result<(), DBApiError> {
    unimplemented!()
  }

  fn delete(&self, token: &str) -> Result<(), DBApiError> {
    unimplemented!()
  }
}
