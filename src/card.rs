use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub struct Card {
  id: u64,
  deck_id: u64,
  deck_pos: u16,
  question: String,
  answer: String,
  last_updated: u32,
}

impl Card {
  pub fn new(
    id: u64,
    deck_id: u64,
    deck_pos: u16,
    question: String,
    answer: String,
  ) -> Result<Card, SystemTimeError> {
    let now: u32 = match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(n) => n.as_secs() as u32,
      Err(err) => return Err(err),
    };
    return Ok(Card {
      id: id,
      deck_id: deck_id,
      deck_pos: deck_pos,
      question: question,
      answer: answer,
      last_updated: now,
    });
  }
}
