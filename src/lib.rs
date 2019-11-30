#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate webe_id;

pub mod card;
pub mod db;
pub mod deck;
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
}
