table! {
    cards (id) {
        id -> Unsigned<Bigint>,
        deck_id -> Unsigned<Bigint>,
        deck_pos -> Unsigned<Smallint>,
        question -> Varchar,
        answer -> Varchar,
        last_updated -> Unsigned<Integer>,
    }
}

// VIEWS TO HELP KEEP POSITIONS IN ORDER, since you can't 'order by' during a sql update
table! {
    card_pos_asc (id) {
        id -> Unsigned<Bigint>,
        deck_id -> Unsigned<Bigint>,
        deck_pos -> Unsigned<Smallint>,
    }
}
table! {
    card_pos_desc (id) {
        id -> Unsigned<Bigint>,
        deck_id -> Unsigned<Bigint>,
        deck_pos -> Unsigned<Smallint>,
    }
}

table! {
    cardscores (account_id, card_id) {
        account_id -> Unsigned<Bigint>,
        card_id -> Unsigned<Bigint>,
        score -> Unsigned<Tinyint>,
    }
}

table! {
    decks (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        owner_id -> Unsigned<Bigint>,
        last_updated -> Unsigned<Integer>,
    }
}

joinable!(cards -> decks (deck_id));
joinable!(cardscores -> cards (card_id));

allow_tables_to_appear_in_same_query!(cards, cardscores, decks,);
