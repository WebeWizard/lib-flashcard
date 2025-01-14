use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    all_the_blobs (id) {
        id -> Integer,
        tiny -> Tinyblob,
        normal -> Blob,
        medium -> Mediumblob,
        big -> Longblob,
    }
}

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
    comments (id) {
        id -> Integer,
        post_id -> Integer,
        text -> Text,
    }
}

table! {
    composite_fk (id) {
        id -> Integer,
        post_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    cyclic_fk_1 (id) {
        id -> Integer,
        cyclic_fk_2_id -> Nullable<Integer>,
    }
}

table! {
    cyclic_fk_2 (id) {
        id -> Integer,
        cyclic_fk_1_id -> Nullable<Integer>,
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

table! {
    fk_doesnt_reference_pk (id) {
        id -> Integer,
        random -> Nullable<Text>,
    }
}

table! {
    fk_inits (id) {
        id -> Integer,
    }
}

table! {
    fk_tests (id) {
        id -> Integer,
        fk_id -> Integer,
    }
}

table! {
    followings (user_id, post_id) {
        user_id -> Integer,
        post_id -> Integer,
        email_notifications -> Bool,
    }
}

table! {
    likes (comment_id, user_id) {
        comment_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    multiple_fks_to_same_table (id) {
        id -> Integer,
        post_id_1 -> Nullable<Integer>,
        post_id_2 -> Nullable<Integer>,
    }
}

table! {
    nullable_doubles (id) {
        id -> Integer,
        n -> Nullable<Double>,
    }
}

table! {
    nullable_table (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}

table! {
    numbers (n) {
        n -> Integer,
    }
}

table! {
    points (x, y) {
        x -> Integer,
        y -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
    }
}

table! {
    precision_numbers (n) {
        n -> Double,
    }
}

table! {
    self_referential_fk (id) {
        id -> Integer,
        parent_id -> Integer,
    }
}

table! {
    special_comments (id) {
        id -> Integer,
        special_post_id -> Integer,
    }
}

table! {
    special_posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
    }
}

table! {
    trees (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
    }
}

table! {
    unsigned_table (id) {
        id -> Integer,
        value -> Unsigned<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

table! {
    users_select_for_update (id) {
        id -> Integer,
        name -> Varchar,
        hair_color -> Nullable<Varchar>,
    }
}

table! {
    users_with_name_pk (name) {
        name -> Varchar,
    }
}

table! {
    with_keywords (fn_) {
        #[sql_name = "fn"]
        fn_ -> Integer,
        #[sql_name = "let"]
        let_ -> Integer,
        #[sql_name = "extern"]
        extern_ -> Integer,
    }
}

joinable!(cards -> decks (deck_id));
joinable!(cardscores -> cards (card_id));
joinable!(comments -> posts (post_id));
joinable!(cyclic_fk_1 -> cyclic_fk_2 (cyclic_fk_2_id));
joinable!(fk_tests -> fk_inits (fk_id));
joinable!(followings -> posts (post_id));
joinable!(followings -> users (user_id));
joinable!(likes -> comments (comment_id));
joinable!(likes -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(cards, cardscores, decks,);
