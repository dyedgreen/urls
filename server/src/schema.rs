table! {
    logins (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Text,
        token -> Text,
        valid_until -> Timestamp,
        claimed -> Bool,
    }
}

table! {
    users (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        email -> Text,
    }
}

joinable!(logins -> users (user_id));

allow_tables_to_appear_in_same_query!(logins, users,);
