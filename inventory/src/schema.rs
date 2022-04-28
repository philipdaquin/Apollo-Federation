table! {
    inventory (id) {
        id -> Int4,
        weight -> Nullable<Int4>,
        price -> Nullable<Int4>,
        in_stock -> Bool,
        shipping_estimate -> Nullable<Int4>,
    }
}
