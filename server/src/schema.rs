table! {
    invites (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        token -> Text,
        created_by -> Text,
        claimed_by -> Nullable<Text>,
    }
}

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
    roles (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Text,
        permission -> Text,
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
joinable!(roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    invites,
    logins,
    roles,
    users,
);
