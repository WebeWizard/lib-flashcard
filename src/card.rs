use crate::schema::cards;

use diesel::prelude::*;
use serde::Serialize;

use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

#[derive(Serialize, AsChangeset, Identifiable, Insertable, Queryable, Debug)]
#[table_name = "cards"]
pub struct Card {
    #[serde(serialize_with = "webe_auth::utility::serialize_as_string")]
    pub id: u64,
    #[serde(serialize_with = "webe_auth::utility::serialize_as_string")]
    pub deck_id: u64,
    pub deck_pos: u16,
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

    pub fn update_position(&mut self, new_pos: u16) {
        self.deck_pos = new_pos;
    }
    pub fn update_question(&mut self, new_question: String) {
        self.question = new_question.to_owned();
    }
    pub fn update_answer(&mut self, new_answer: String) {
        self.answer = new_answer.to_owned();
    }
}
