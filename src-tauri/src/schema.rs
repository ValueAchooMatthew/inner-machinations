// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        verified -> Bool,
        #[max_length = 6]
        code -> Nullable<Varchar>,
    }
}
