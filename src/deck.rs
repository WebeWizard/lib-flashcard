use crate::card::Card;
use crate::schema::decks;

use serde::Serialize;

use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

// used as the database record since we can't exclude fields from derives
// see https://github.com/diesel-rs/diesel/issues/860
#[derive(Serialize, AsChangeset, Identifiable, Insertable, Queryable, Debug)]
#[table_name = "decks"]
pub struct Deck {
  #[serde(serialize_with = "webe_auth::utility::serialize_as_string")]
  pub id: u64,
  name: String,
  pub owner_id: u64,
  last_updated: u32,
}

#[derive(Serialize)]
pub struct DeckDetails {
  pub info: Deck,
  pub cards: Vec<Card>,
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
    });
  }

  pub fn rename(&mut self, name: &str) {
    self.name = name.to_owned();
  }
}
