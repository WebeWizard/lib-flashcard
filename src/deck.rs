use crate::card::Card;
use crate::schema::flash_decks;

use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

// used as the database record since we can't exclude fields from derives
// see https://github.com/diesel-rs/diesel/issues/860
#[derive(Insertable)]
#[table_name = "flash_decks"]
pub struct DeckInfo {
  id: u64,
  name: String,
  owner_id: u64,
  last_updated: u32,
}

pub struct Deck {
  id: u64,
  name: String,
  owner_id: u64,
  last_updated: u32,
  cards: Vec<Card>,
}

impl Deck {
  pub fn new(id: u64, owner_id: u64, name: String) -> Result<Deck, SystemTimeError> {
    let now: u32 = match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(n) => n.as_secs() as u32,
      Err(err) => return Err(err),
    };
    return Ok(Deck {
      id: id,
      name: name,
      owner_id: owner_id,
      last_updated: now,
      cards: Vec::new(),
    });
  }

  // creates an empty deck using provided info
  pub fn from_info(info: DeckInfo) -> Deck {
    Deck {
      id: info.id,
      name: info.name,
      owner_id: info.owner_id,
      last_updated: info.last_updated,
      cards: Vec::new(),
    }
  }
}
