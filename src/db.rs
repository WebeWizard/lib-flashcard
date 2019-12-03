// This module contains database CRUD operations for each of the models.

use diesel::prelude::*;
use diesel::r2d2 as diesel_r2d2;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error as DieselError;

use crate::card::Card;
use crate::deck::Deck;
use crate::schema::cards::dsl::{id as cardId, *};
use crate::schema::decks::dsl::{id as deckId, *};

#[derive(Debug)]
pub enum DBApiError {
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

  fn delete(&self, id: &u64) -> Result<(), DBApiError>;
}

// TODO: since crud operations for all types are basically the same,
// we could/should move them all into a DBManager impl with generic functions.
// and just keep the type specific code in their own api impls
impl DeckApi for DBManager {
  fn insert(&self, deck_info: &Deck) -> Result<(), DBApiError> {
    let conn = self.get()?;
    match diesel::insert_into(decks).values(deck_info).execute(&conn) {
      Ok(_) => return Ok(()),
      Err(err) => return Err(DBApiError::OtherError(err)),
    }
  }

  fn find(&self, deck_info_id: &u64) -> Result<Deck, DBApiError> {
    let conn = self.get()?;
    let deck_info = decks.find(deck_info_id).first(&conn)?;
    return Ok(deck_info);
  }

  fn update(&self, deck_info: &Deck) -> Result<(), DBApiError> {
    let conn = self.get()?;
    diesel::update(decks).set(deck_info).execute(&conn)?;
    return Ok(());
  }

  fn delete(&self, deck_info_id: &u64) -> Result<(), DBApiError> {
    let conn = self.get()?;
    let result = diesel::delete(decks.filter(deckId.eq(deck_info_id))).execute(&conn)?;
    if result == 1 {
      return Ok(());
    } else {
      return Err(DBApiError::NotFound);
    }
  }
}

pub trait CardApi {
  fn insert(&self, card: &Card) -> Result<(), DBApiError>;

  fn find(&self, card_id: &u64) -> Result<Card, DBApiError>;

  fn update(&self, card: &Card) -> Result<(), DBApiError>;

  fn delete(&self, card_id: &u64) -> Result<(), DBApiError>;
}

impl CardApi for DBManager {
  fn insert(&self, card: &Card) -> Result<(), DBApiError> {
    let conn = self.get()?;
    match diesel::insert_into(cards).values(card).execute(&conn) {
      Ok(_) => return Ok(()),
      Err(err) => return Err(DBApiError::OtherError(err)),
    }
  }

  fn find(&self, card_id: &u64) -> Result<Card, DBApiError> {
    let conn = self.get()?;
    let card = cards.find(card_id).first(&conn)?;
    return Ok(card);
  }

  fn update(&self, card: &Card) -> Result<(), DBApiError> {
    let conn = self.get()?;
    diesel::update(cards).set(card).execute(&conn)?;
    return Ok(());
  }

  fn delete(&self, card_id: &u64) -> Result<(), DBApiError> {
    let conn = self.get()?;
    let result = diesel::delete(cards.filter(cardId.eq(card_id))).execute(&conn)?;
    if result == 1 {
      return Ok(());
    } else {
      return Err(DBApiError::NotFound);
    }
  }
}
