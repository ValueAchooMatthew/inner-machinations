// @generated automatically by Diesel CLI.

diesel::table! {
    saved_automata (id) {
        id -> Integer,
        u_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    saved_connections (id) {
        id -> Integer,
        automata_id -> Integer,
        #[max_length = 255]
        start_point -> Varchar,
        #[max_length = 255]
        control_point_one -> Varchar,
        #[max_length = 255]
        control_point_two -> Varchar,
        #[max_length = 255]
        end_point -> Varchar,
        #[max_length = 1]
        connection_character -> Varchar,
    }
}

diesel::table! {
    saved_states (id) {
        id -> Integer,
        automata_id -> Integer,
        #[max_length = 255]
        position -> Varchar,
        is_start -> Bool,
        is_final -> Bool,
    }
}

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

diesel::joinable!(saved_automata -> users (u_id));
diesel::joinable!(saved_connections -> saved_automata (automata_id));
diesel::joinable!(saved_states -> saved_automata (automata_id));

diesel::allow_tables_to_appear_in_same_query!(
    saved_automata,
    saved_connections,
    saved_states,
    users,
);
