extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub mod card;
pub mod deck;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn write_to_file(name: &str, data: &[u8]) -> std::io::Result<()> {
    let mut f = File::create(name)?;
    f.write_all(data)?;
    f.sync_all()?;
    Ok(())
}

pub fn read_from_file(path: &str, content: &mut String) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::card::*;
    use super::deck::*;
    
    // cards should be created with any values that can be parsed to String
    #[test]
    fn card_create() {
        let card = Card::new(123, "flashcard");
        assert_eq!(card.question, "123".to_string());
        assert_eq!(card.answer, "flashcard".to_string());
    }

    // decks should be created with a string parsable name
    #[test]
    fn deck_create() {
        let deck = Deck::new(123);
        assert_eq!(deck.name, "123".to_string());
    }

    #[test]
    fn save_deck_to_yaml() {
        let mut deck = Deck::new(123);
        let card = Card::new(456, "flashcard");
        deck.cards.push_back(card);
        deck.write_to_yaml();
    }

    #[test]
    fn read_deck_from_yaml() {
        let deck = Deck::read_from_yaml("123").unwrap();
        assert_eq!(deck.name, "123".to_string());
    }
}