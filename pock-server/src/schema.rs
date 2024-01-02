// @generated automatically by Diesel CLI.

diesel::table! {
    party (transaction_id, user_id) {
        transaction_id -> Int8,
        user_id -> Int8,
    }
}

diesel::table! {
    transaction (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
        value -> Float8,
        tripId -> Nullable<Int8>,
        payerId -> Nullable<Int8>,
    }
}

diesel::table! {
    transaction_participants_user (transactionId, userId) {
        transactionId -> Int8,
        userId -> Int8,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int8,
        trip_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        value -> Int8,
        payer_id -> Int8,
    }
}

diesel::table! {
    trip (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    trips (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Int8,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
    }
}

diesel::joinable!(party -> transactions (transaction_id));
diesel::joinable!(party -> users (user_id));
diesel::joinable!(transaction -> trip (tripId));
diesel::joinable!(transaction -> user (payerId));
diesel::joinable!(transaction_participants_user -> transaction (transactionId));
diesel::joinable!(transaction_participants_user -> user (userId));
diesel::joinable!(transactions -> trips (trip_id));
diesel::joinable!(transactions -> users (payer_id));

diesel::allow_tables_to_appear_in_same_query!(
    party,
    transaction,
    transaction_participants_user,
    transactions,
    trip,
    trips,
    user,
    users,
);
