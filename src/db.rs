// This module contains database CRUD operations for each of the models.

use diesel::prelude::*;
use diesel::r2d2 as diesel_r2d2;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::DatabaseErrorKind as DBErrorKind;
use diesel::result::Error as DieselError;

use crate::card::Card;
use crate::deck::Deck;
use crate::game::CardScore;
use crate::schema::card_pos_asc::dsl as CardPosAscDSL;
use crate::schema::card_pos_desc::dsl as CardPosDescDSL;
use crate::schema::cards::dsl as CardDSL;
use crate::schema::cardscores::dsl as ScoreDSL;
use crate::schema::decks::dsl as DeckDSL;

#[derive(Debug)]
pub enum DBApiError {
    OtherError(DieselError), // errors from interacting with database
    BadVerifyCode,
    PoolError(r2d2::Error),
    NotAllowed, // caller is trying to do something wonky
    NotFound,
}

impl From<DBApiError> for crate::FlashError {
    fn from(err: DBApiError) -> crate::FlashError {
        crate::FlashError::DBError(err)
    }
}

impl From<r2d2::Error> for DBApiError {
    fn from(err: r2d2::Error) -> DBApiError {
        DBApiError::PoolError(err)
    }
}

impl From<DieselError> for DBApiError {
    fn from(err: DieselError) -> DBApiError {
        match err {
            DieselError::NotFound => DBApiError::NotFound,
            _ => DBApiError::OtherError(err),
        }
    }
}

pub type DBManager = diesel_r2d2::Pool<diesel_r2d2::ConnectionManager<MysqlConnection>>;

pub fn new_manager(connect_string: String) -> Result<DBManager, DBApiError> {
    let connection_manager = ConnectionManager::new(connect_string.as_str());
    // build the database connection pool
    let pool = Pool::builder().max_size(10).build(connection_manager)?;
    return Ok(pool);
}

pub trait DeckApi {
    fn insert(&self, deck: &Deck) -> Result<(), DBApiError>;

    fn find(&self, id: &u64) -> Result<Deck, DBApiError>;

    fn find_decks_for_owner(&self, owner_id: &u64) -> Result<Vec<Deck>, DBApiError>;

    fn update(&self, deck: &Deck) -> Result<(), DBApiError>;

    fn delete(&self, id: &u64) -> Result<(), DBApiError>;
}

// TODO: since crud operations for all types are basically the same,
// we could/should move them all into a DBManager impl with generic functions.
// and just keep the type specific code in their own api impls
impl DeckApi for DBManager {
    fn insert(&self, deck_info: &Deck) -> Result<(), DBApiError> {
        let mut conn = self.get()?;
        match diesel::insert_into(DeckDSL::decks)
            .values(deck_info)
            .execute(&mut conn)
        {
            Ok(_) => return Ok(()),
            Err(err) => return Err(DBApiError::OtherError(err)),
        }
    }

    fn find(&self, deck_info_id: &u64) -> Result<Deck, DBApiError> {
        let mut conn = self.get()?;
        let deck_info = DeckDSL::decks.find(deck_info_id).first(&mut conn)?;
        return Ok(deck_info);
    }

    fn find_decks_for_owner(&self, owner: &u64) -> Result<Vec<Deck>, DBApiError> {
        let mut conn = self.get()?;
        let owner_decks = DeckDSL::decks
            .filter(DeckDSL::owner_id.eq(owner))
            .get_results(&mut conn)?;
        // TODO: should result be sorted in any convenient way?
        return Ok(owner_decks);
    }

    fn update(&self, deck_info: &Deck) -> Result<(), DBApiError> {
        let mut conn = self.get()?;
        diesel::update(deck_info)
            .set(deck_info)
            .execute(&mut conn)?;
        return Ok(());
    }

    fn delete(&self, deck_info_id: &u64) -> Result<(), DBApiError> {
        let mut conn = self.get()?;
        let result = diesel::delete(DeckDSL::decks.filter(DeckDSL::id.eq(deck_info_id)))
            .execute(&mut conn)?;
        if result == 1 {
            return Ok(());
        } else {
            return Err(DBApiError::NotFound);
        }
    }
}

pub trait CardApi {
    fn insert(&self, card: &Card) -> Result<(), DBApiError>;

    fn find(&self, card_id: &u64) -> Result<Card, DBApiError>;

    fn find_cards_for_deck(&self, deck_id: &u64) -> Result<Vec<Card>, DBApiError>;

    fn update(&self, card: &Card) -> Result<(), DBApiError>;

    fn update_position(
        &self,
        card_id: u64,
        deck_id: u64,
        orig_pos: u16,
        new_pos: u16,
    ) -> Result<(), DBApiError>;

    fn delete(&self, card_id: &u64) -> Result<(), DBApiError>;
}

impl CardApi for DBManager {
    fn insert(&self, card: &Card) -> Result<(), DBApiError> {
        let mut conn = self.get()?;
        // DO NOT ALLOW USER TO MOVE CARD TO RESERVED POSITION 0
        if card.deck_pos == 0 {
            return Err(DBApiError::NotAllowed);
        }
        match diesel::insert_into(CardDSL::cards)
            .values(card)
            .execute(&mut conn)
        {
            Ok(_) => return Ok(()),
            Err(err) => return Err(DBApiError::OtherError(err)),
        }
    }

    fn find(&self, card_id: &u64) -> Result<Card, DBApiError> {
        let mut conn = self.get()?;
        let card = CardDSL::cards.find(card_id).first(&mut conn)?;
        return Ok(card);
    }

    fn find_cards_for_deck(&self, card_deck_id: &u64) -> Result<Vec<Card>, DBApiError> {
        let mut conn = self.get()?;
        let deck_cards = CardDSL::cards
            .filter(CardDSL::deck_id.eq(card_deck_id))
            .order(CardDSL::deck_pos.asc())
            .get_results(&mut conn)?;
        // TODO: should result be sorted in any convenient way? position?
        return Ok(deck_cards);
    }

    fn update(&self, card: &Card) -> Result<(), DBApiError> {
        let mut conn = self.get()?;
        diesel::update(card).set(card).execute(&mut conn)?;
        return Ok(());
    }

    fn update_position(
        &self,
        card_id: u64,
        deck_id: u64,
        orig_pos: u16,
        new_pos: u16,
    ) -> Result<(), DBApiError> {
        // DO NOT ALLOW USER TO MOVE CARD TO RESERVED POSITION 0
        if new_pos == 0 {
            return Err(DBApiError::NotAllowed);
        }
        // ignore without change
        if new_pos == orig_pos {
            return Ok(());
        }

        let mut conn = self.get()?;

        conn.transaction::<(), DBApiError, _>(|conn| {
            // try to set the target card to position 0
            // if not found, then the card doesn't exist at the original position the user expects
            // - and therefore should be an error
            let result = diesel::update(
                CardDSL::cards.filter(
                    CardDSL::deck_id
                        .eq(deck_id)
                        .and(CardDSL::id.eq(card_id))
                        .and(CardDSL::deck_pos.eq(orig_pos)),
                ),
            )
            .set(CardDSL::deck_pos.eq(0))
            .execute(conn)?;
            if result == 0 {
                // no matching card
                return Err(DBApiError::OtherError(DieselError::NotFound));
            } // NOTE:  result > 1 should be impossible based on card_id being primary key
            // shift all cards between new and orig
            if new_pos < orig_pos {
                diesel::update(
                    CardPosDescDSL::card_pos_desc.filter(
                        CardPosDescDSL::deck_id
                            .eq(deck_id)
                            .and(CardPosDescDSL::deck_pos.le(orig_pos))
                            .and(CardPosDescDSL::deck_pos.ge(new_pos)),
                    ),
                )
                .set(CardPosDescDSL::deck_pos.eq(CardPosDescDSL::deck_pos + 1))
                .execute(conn)?;
            } else {
                diesel::update(
                    CardPosAscDSL::card_pos_asc.filter(
                        CardPosAscDSL::deck_id
                            .eq(deck_id)
                            .and(CardPosAscDSL::deck_pos.le(new_pos))
                            .and(CardPosAscDSL::deck_pos.ge(orig_pos)),
                    ),
                )
                .set(CardPosAscDSL::deck_pos.eq(CardPosAscDSL::deck_pos - 1))
                .execute(conn)?;
            }
            // move the card into final position
            diesel::update(CardDSL::cards.filter(CardDSL::id.eq(card_id)))
                .set(CardDSL::deck_pos.eq(new_pos))
                .execute(conn)?;
            return Ok(());
        })?;

        return Ok(());
    }

    fn delete(&self, card_id: &u64) -> Result<(), DBApiError> {
        let mut conn = self.get()?;
        conn.transaction::<(), DBApiError, _>(|conn| {
            // get the card if exists
            let card: Card = CardDSL::cards.find(card_id).first(conn)?;
            // delete the card
            diesel::delete(CardDSL::cards.filter(CardDSL::id.eq(card_id))).execute(conn)?;
            // shift all of the following cards down 1 position
            diesel::update(
                CardDSL::cards.filter(
                    CardDSL::deck_id
                        .eq(card.deck_id)
                        .and(CardDSL::deck_pos.ge(card.deck_pos)),
                ),
            )
            .set(CardDSL::deck_pos.eq(CardDSL::deck_pos - 1))
            .execute(conn)?;
            return Ok(());
        })?;
        return Ok(());
    }
}

pub trait GameApi {
    fn update_score(&self, score: CardScore) -> Result<(), DBApiError>;
    fn get_deck_scores(&self, deck_id: u64, account_id: u64) -> Result<Vec<CardScore>, DBApiError>;
}

impl GameApi for DBManager {
    fn update_score(&self, score: CardScore) -> Result<(), DBApiError> {
        let mut conn = self.get()?;
        // NOTE: Mysql does not support Upsert
        // so we must try insert first, then update on duplicate
        match diesel::insert_into(ScoreDSL::cardscores)
            .values(&score)
            .execute(&mut conn)
        {
            Ok(_) => {}
            Err(error) => match error {
                DieselError::DatabaseError(db_error, _) => {
                    match db_error {
                        DBErrorKind::UniqueViolation => {
                            // already exists, time to try update
                            diesel::update(ScoreDSL::cardscores)
                                .filter(ScoreDSL::card_id.eq(score.card_id))
                                .set(&score)
                                .execute(&mut conn)?;
                            return Ok(());
                        }
                        _ => return Err(DBApiError::from(error)),
                    }
                }
                _ => return Err(DBApiError::from(error)),
            },
        }
        return Ok(());
    }

    fn get_deck_scores(&self, deck_id: u64, account_id: u64) -> Result<Vec<CardScore>, DBApiError> {
        let mut conn = self.get()?;
        let deck_scores = ScoreDSL::cardscores
            .inner_join(CardDSL::cards)
            .select((ScoreDSL::account_id, ScoreDSL::card_id, ScoreDSL::score))
            .filter(
                CardDSL::deck_id
                    .eq(deck_id)
                    .and(ScoreDSL::account_id.eq(account_id)),
            )
            .load::<CardScore>(&mut conn)?;
        // TODO: should result be sorted in any convenient way? position?
        return Ok(deck_scores);
    }
}
