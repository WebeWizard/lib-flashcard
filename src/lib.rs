#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate webe_auth;
extern crate webe_id;
extern crate webe_web;

pub mod card;
pub mod db;
pub mod deck;
pub mod game;
pub mod http;
pub mod schema;

use webe_auth::session::Session;

use card::Card;
use db::DBApiError;
use deck::{Deck, DeckDetails};
use game::CardScore;

use std::sync::Mutex;
use std::time::SystemTimeError;

#[derive(Debug)]
pub enum FlashError {
  PermissionError,
  DBError(DBApiError),
  OtherError,
  SystemTimeError,
  SessionTimeout,
}

pub struct FlashManager<'f> {
  pub db_manager: db::DBManager,
  pub id_factory: &'f Mutex<webe_id::WebeIDFactory>,
}

impl From<SystemTimeError> for FlashError {
  fn from(_err: SystemTimeError) -> FlashError {
    FlashError::SystemTimeError
  }
}

impl<'f> FlashManager<'f> {
  pub fn new_id(&self) -> Result<u64, FlashError> {
    match self.id_factory.lock() {
      Ok(mut factory) => match factory.next() {
        Ok(id) => return Ok(id),
        _ => return Err(FlashError::OtherError),
      },
      // TODO: find a way to make the lock not poisonable
      _ => return Err(FlashError::OtherError), // mutex is poisoned
    }
  }

  // get deck info
  pub fn get_deck_info(&self, session: &Session, deck_id: &u64) -> Result<Deck, FlashError> {
    if !session.is_expired() {
      let deck = db::DeckApi::find(&self.db_manager, deck_id)?;
      return Ok(deck);
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  // create deck
  pub fn create_deck(&self, session: &Session, name: String) -> Result<Deck, FlashError> {
    // TODO: validate name isn't empty
    if !session.is_expired() {
      let id = self.new_id()?;
      let deck = Deck::new(id, session.account_id, name)?;
      db::DeckApi::insert(&self.db_manager, &deck)?;
      return Ok(deck);
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  // update deck
  pub fn rename_deck(&self, session: &Session, deck_id: u64, name: &str) -> Result<(), FlashError> {
    // TODO: validate name isn't empty
    if !session.is_expired() {
      // find the existing deck in the db
      // TODO: include session.account_id as a filter in the db query
      let existing = db::DeckApi::find(&self.db_manager, &deck_id)?;
      if session.account_id != existing.owner_id {
        return Err(FlashError::PermissionError);
      }
      // provide db the modified object
      let mut updated = existing;
      updated.rename(name);
      // TODO: add a 'rename' function to the database api instead of using 'update'
      db::DeckApi::update(&self.db_manager, &updated).map_err(|e| FlashError::DBError(e))
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  // delete deck
  pub fn delete_deck(&self, session: &Session, deck_id: u64) -> Result<(), FlashError> {
    if !session.is_expired() {
      // TODO: include session.account_id as a filter in the db query
      let existing = db::DeckApi::find(&self.db_manager, &deck_id)?;
      if session.account_id != existing.owner_id {
        return Err(FlashError::PermissionError);
      }
      db::DeckApi::delete(&self.db_manager, &deck_id).map_err(|e| FlashError::DBError(e))
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  // create card
  pub fn create_card(
    &self,
    session: &Session,
    deck_id: u64,
    deck_pos: u16,
    question: String,
    answer: String,
  ) -> Result<Card, FlashError> {
    if !session.is_expired() {
      // TODO: like most things, checking valid session, checking deck owner, etc
      // - can be done entirely in database with one call instead of many api calls
      let deck = db::DeckApi::find(&self.db_manager, &deck_id)?;
      if deck.owner_id == session.account_id {
        let id = self.new_id()?;
        let card = Card::new(id, deck_id, deck_pos, question, answer)?;
        db::CardApi::insert(&self.db_manager, &card)?;
        return Ok(card);
      } else {
        return Err(FlashError::PermissionError);
      }
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  // fetch card
  pub fn get_card(&self, session: &Session, card_id: u64) -> Result<Card, FlashError> {
    if !session.is_expired() {
      // find the existing card in the db
      let existing = db::CardApi::find(&self.db_manager, &card_id)?;
      // verify owner. TODO: can these two be made into one database call?
      let deck = db::DeckApi::find(&self.db_manager, &existing.deck_id)?;
      if deck.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      let card = db::CardApi::find(&self.db_manager, &card_id)?;
      return Ok(card);
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  // update card
  pub fn update_card(
    &self,
    session: &Session,
    card_id: u64,
    question: Option<String>,
    answer: Option<String>,
  ) -> Result<(), FlashError> {
    if !session.is_expired() {
      // find the existing card in the db
      let existing = db::CardApi::find(&self.db_manager, &card_id)?;
      // verify owner. TODO: can these two be made into one database call?
      let deck = db::DeckApi::find(&self.db_manager, &existing.deck_id)?;
      if deck.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      // provide db the modified object
      let mut updated = existing;
      if let Some(question) = question {
        updated.update_question(question);
      }
      if let Some(answer) = answer {
        updated.update_answer(answer);
      }
      // TODO: should this function be split into update_pos , update_question etc?
      db::CardApi::update(&self.db_manager, &updated).map_err(|e| FlashError::DBError(e))
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  pub fn update_card_position(
    &self,
    session: &Session,
    deck_id: u64,
    card_id: u64,
    orig_pos: u16,
    new_pos: u16,
  ) -> Result<(), FlashError> {
    if !session.is_expired() {
      // verify owner. TODO: can these two be made into one database call?
      let deck = db::DeckApi::find(&self.db_manager, &deck_id)?;
      if deck.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      db::CardApi::update_position(&self.db_manager, deck_id, card_id, orig_pos, new_pos)
        .map_err(|e| FlashError::DBError(e))
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  // delete card
  pub fn delete_card(&self, session: &Session, card_id: u64) -> Result<(), FlashError> {
    if !session.is_expired() {
      // find the existing card in the db
      let existing = db::CardApi::find(&self.db_manager, &card_id)?;
      // verify owner. TODO: can these two be made into one database call?
      let deck = db::DeckApi::find(&self.db_manager, &existing.deck_id)?;
      if deck.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      db::CardApi::delete(&self.db_manager, &card_id).map_err(|e| FlashError::DBError(e))
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  pub fn get_decks_for_session(&self, session: &Session) -> Result<Vec<Deck>, FlashError> {
    if !session.is_expired() {
      return db::DeckApi::find_decks_for_owner(&self.db_manager, &session.account_id)
        .map_err(|e| FlashError::DBError(e));
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  pub fn get_deck_details(
    &self,
    session: &Session,
    deck_id: &u64,
  ) -> Result<DeckDetails, FlashError> {
    if !session.is_expired() {
      let deck_info = self.get_deck_info(session, deck_id)?;
      // verify owner. TODO: can these two be made into one database call?
      if deck_info.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      let cards = self.get_cards_for_deck(session, deck_id)?; // TODO: turn into single sql statement using join
      return Ok(DeckDetails {
        info: deck_info,
        cards: cards,
      });
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  pub fn get_cards_for_deck(
    &self,
    session: &Session,
    deck_id: &u64,
  ) -> Result<Vec<Card>, FlashError> {
    if !session.is_expired() {
      // verify owner. TODO: can these two be made into one database call?
      let deck = db::DeckApi::find(&self.db_manager, &deck_id)?;
      if deck.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      return db::CardApi::find_cards_for_deck(&self.db_manager, deck_id)
        .map_err(|e| FlashError::DBError(e));
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  pub fn update_score(&self, session: &Session, card_id: u64, score: u8) -> Result<(), FlashError> {
    if !session.is_expired() {
      let card_score = CardScore::new(session.account_id, card_id, score);
      let card = db::CardApi::find(&self.db_manager, &card_id)?;
      let deck = db::DeckApi::find(&self.db_manager, &card.deck_id)?;
      if deck.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      return db::GameApi::update_score(&self.db_manager, card_score)
        .map_err(|e| FlashError::DBError(e));
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }

  pub fn get_deck_scores(
    &self,
    session: &Session,
    deck_id: u64,
  ) -> Result<Vec<CardScore>, FlashError> {
    if !session.is_expired() {
      let deck = db::DeckApi::find(&self.db_manager, &deck_id)?;
      if deck.owner_id != session.account_id {
        return Err(FlashError::PermissionError);
      }
      return db::GameApi::get_deck_scores(&self.db_manager, deck_id)
        .map_err(|e| FlashError::DBError(e));
    } else {
      return Err(FlashError::SessionTimeout);
    }
  }
}
