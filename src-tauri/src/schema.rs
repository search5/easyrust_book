// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int8,
        uuid -> Uuid,
        title -> Varchar,
        author -> Varchar,
        translator -> Nullable<Varchar>,
        publisher -> Nullable<Varchar>,
        coverimage -> Nullable<Varchar>,
        purchasedate -> Nullable<Date>,
        purchaselocation -> Nullable<Varchar>,
        #[max_length = 13]
        isbn -> Nullable<Varchar>,
        createdat -> Nullable<Timestamptz>,
    }
}
