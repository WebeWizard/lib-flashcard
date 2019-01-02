use std::fmt::Display;

use rand::random;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    #[serde(skip)]
    pub id: usize,
    pub question: String,
    pub answer: String
}

impl Card {
    pub fn new<Q: Display,A: Display>(q: Q, a: A) -> Card {
        Card {id: random::<usize>(), question: q.to_string(), answer: a.to_string()}
    }
}