table! {
    review (id) {
        id -> Int4,
        body -> Varchar,
        author_id -> Int4,
        product_id -> Int4,
        heading -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        media -> Nullable<Varchar>,
        is_edited -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
    }
}
