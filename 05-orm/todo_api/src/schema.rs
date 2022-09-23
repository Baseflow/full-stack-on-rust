table! {
    todos (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        completed -> Bool,
        completed_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}
