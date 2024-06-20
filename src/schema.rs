table! {
    users (id) {
        id -> Int4,
        user_data -> Jsonb,
        chat_data -> Jsonb,
        epoch_time -> Int4,
    }
}

table! {
    requests (id) {
        id -> Int4,
        user_id -> Int4,
        command -> Text,
        response -> Text,
        request_time -> Timestamp,
    }
}

joinable!(requests -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    requests,
);
