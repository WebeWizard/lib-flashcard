use std::fmt::Display;
use std::collections::VecDeque;
use super::card::Card;

use rand::random;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
  #[serde(skip)]
  pub id: usize,
  pub name: String,
  pub cards: VecDeque<Card>
}

impl Deck {
  pub fn new<T: Display>(n: T) -> Deck {
    Deck {id: random::<usize>(), name: n.to_string(), cards: VecDeque::<Card>::new()}
  }

  pub fn write_to_yaml(&self) -> std::io::Result<()> {
    let yaml = serde_yaml::to_string(&self).unwrap();
    super::write_to_file(
      &format!("{}.yaml",&self.name), yaml.as_bytes()
    )
  }

  pub fn read_from_yaml(deck_name: &str) -> std::io::Result<Deck> {
    let mut yaml = String::new();
    super::read_from_file(
      &format!("{}.yaml",deck_name), &mut yaml)?;
    let deck: Deck = serde_yaml::from_str(&yaml).unwrap();
    Ok(deck)
  }

}
