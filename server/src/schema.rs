// @generated automatically by Diesel CLI.

diesel::table! {
    contents (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
        text_content -> Nullable<Text>,
        binary_data -> Nullable<Bytea>,
        #[max_length = 255]
        mime_type -> Varchar,
    }
}

diesel::table! {
    message_contents (message_id, content_id) {
        message_id -> Int4,
        content_id -> Int4,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    user_facts (user_id, content_id) {
        user_id -> Int4,
        content_id -> Int4,
    }
}

diesel::table! {
    user_wake_word_samples (user_id, content_id) {
        user_id -> Int4,
        content_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(message_contents -> contents (content_id));
diesel::joinable!(message_contents -> messages (message_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(user_facts -> contents (content_id));
diesel::joinable!(user_facts -> users (user_id));
diesel::joinable!(user_wake_word_samples -> contents (content_id));
diesel::joinable!(user_wake_word_samples -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    message_contents,
    messages,
    user_facts,
    user_wake_word_samples,
    users,
);
