table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        joined_at -> Timestamp,
        role -> Varchar,
    }
}

table! {
    valid_roles (roles) {
        roles -> Varchar,
    }
}

joinable!(users -> valid_roles (role));

allow_tables_to_appear_in_same_query!(
    users,
    valid_roles,
);
