table! {
    user_account (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Bpchar,
        email -> Varchar,
        is_enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user_activation_token (user_id) {
        user_id -> Uuid,
        token -> Nullable<Bpchar>,
        created_at -> Timestamp,
    }
}

joinable!(user_activation_token -> user_account (user_id));

allow_tables_to_appear_in_same_query!(
    user_account,
    user_activation_token,
);
