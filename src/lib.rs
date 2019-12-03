#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate webe_auth;
extern crate webe_id;
extern crate webe_web;

pub mod card;
pub mod db;
pub mod deck;
pub mod http;
pub mod schema;

use card::Card;
use db::DBApiError;
use deck::Deck;

use std::sync::Mutex;

#[derive(Debug)]
pub enum FlashError {
    DBError(DBApiError),
    OtherError,
}

pub struct FlashManager<'f> {
    pub db_manager: db::DBManager,
    pub id_factory: &'f Mutex<webe_id::WebeIDFactory>,
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

    pub fn get_decks_for_owner(owner_id: &u64) -> Result<Vec<Deck>, FlashError> {
        unimplemented!()
    }

    pub fn get_cards_for_deck(deck_id: &u64) -> Result<Vec<Card>, FlashError> {
        unimplemented!()
    }

    pub fn get_card(card_id: &u64) -> Result<Card, FlashError> {
        unimplemented!()
    }

    // TODO: maybe a get_all_decks_for_owner and get_all_cards_for_deck
    // for a future offline sync.
}
