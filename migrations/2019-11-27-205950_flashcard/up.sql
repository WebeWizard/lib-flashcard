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
  FOREIGN KEY (deck_id)
    REFERENCES decks(id)
    ON DELETE CASCADE
);