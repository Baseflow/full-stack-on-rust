table! {
    todos (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        completed -> Bool,
        completed_at -> Timestamp,
        created_at -> Timestamp,
    }
}
