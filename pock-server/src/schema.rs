// @generated automatically by Diesel CLI.

diesel::table! {
    transaction (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
        value -> Float8,
        tripId -> Int8,
        payerId -> Int8,
    }
}

diesel::table! {
    transaction_participants_user (transactionId, userId) {
        transactionId -> Int8,
        userId -> Int8,
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
    user (id) {
        id -> Int8,
        name -> Varchar,
    }
}

diesel::joinable!(transaction -> trip (tripId));
diesel::joinable!(transaction -> user (payerId));
diesel::joinable!(transaction_participants_user -> transaction (transactionId));
diesel::joinable!(transaction_participants_user -> user (userId));

diesel::allow_tables_to_appear_in_same_query!(
    transaction,
    transaction_participants_user,
    trip,
    user,
);
