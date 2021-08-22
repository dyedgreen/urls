table! {
    comments (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        comment -> Text,
        url_id -> Text,
        created_by -> Text,
        replies_to -> Nullable<Text>,
    }
}

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
        email_token -> Text,
        claim_until -> Timestamp,
        claimed -> Bool,
        session_token -> Nullable<Text>,
        last_used -> Timestamp,
        last_user_agent -> Nullable<Text>,
        revoked -> Bool,
        last_remote_ip -> Nullable<Text>,
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
    url_upvotes (url_id, user_id) {
        url_id -> Text,
        user_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    urls (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        url -> Text,
        status_code -> Integer,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
        created_by -> Text,
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

joinable!(comments -> urls (url_id));
joinable!(comments -> users (created_by));
joinable!(logins -> users (user_id));
joinable!(roles -> users (user_id));
joinable!(url_upvotes -> urls (url_id));
joinable!(url_upvotes -> users (user_id));
joinable!(urls -> users (created_by));

allow_tables_to_appear_in_same_query!(
    comments,
    invites,
    logins,
    roles,
    url_upvotes,
    urls,
    users,
);
