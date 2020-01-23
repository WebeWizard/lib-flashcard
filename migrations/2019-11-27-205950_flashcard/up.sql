CREATE TABLE decks (
  id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  owner_id BIGINT UNSIGNED NOT NULL, /* Account that created deck */
  last_updated INT UNSIGNED NOT NULL  /* Seconds since UNIX EPOCH */
);


CREATE TABLE cards (
  id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
  deck_id BIGINT UNSIGNED NOT NULL,
  deck_pos SMALLINT UNSIGNED NOT NULL, /* TODO: add UNIQUE constraint on deck_id and deck_pos */ 
  question VARCHAR(50) NOT NULL,
  answer VARCHAR(50) NOT NULL,
  last_updated INT UNSIGNED NOT NULL, /* Seconds since UNIX EPOCH */
  CONSTRAINT unique_pos UNIQUE (deck_id, deck_pos), /* only one card can occupy a position in a deck */
  FOREIGN KEY (deck_id)
    REFERENCES decks(id)
    ON DELETE CASCADE
);

CREATE TABLE cardscores (
  account_id BIGINT UNSIGNED NOT NULL,
  card_id BIGINT UNSIGNED NOT NULL,
  score TINYINT UNSIGNED NOT NULL,
  CONSTRAINT score_primary 
   PRIMARY KEY (account_id, card_id),
  FOREIGN KEY (card_id)
    REFERENCES cards(id)
    ON DELETE CASCADE
);