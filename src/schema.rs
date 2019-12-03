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

table! {
    decks (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        owner_id -> Unsigned<Bigint>,
        last_updated -> Unsigned<Integer>,
    }
}

joinable!(cards -> decks (deck_id));

allow_tables_to_appear_in_same_query!(
    cards,
    decks,
);
