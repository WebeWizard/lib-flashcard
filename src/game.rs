use crate::schema::cardscores;

// Flashcard game based on Brainscape
#[derive(AsChangeset, Identifiable, Insertable, Queryable, Debug)]
#[table_name = "cardscores"]
#[primary_key(account_id, card_id)]
pub struct CardScore {
  account_id: u64,
  card_id: u64,
  score: u8, // TODO: include timestamp to see if user improves over time?
}

impl CardScore {
  pub fn new(account_id: u64, card_id: u64, score: u8) -> CardScore {
    CardScore {
      account_id: account_id,
      card_id: card_id,
      score: score,
    }
  }
}
