table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        completed -> Bool,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}
