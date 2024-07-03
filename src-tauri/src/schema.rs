// @generated automatically by Diesel CLI.

diesel::table! {
    saved_connections (id) {
        id -> Integer,
        workspace_id -> Integer,
        start_point -> Text,
        control_point_one -> Text,
        control_point_two -> Text,
        end_point -> Text,
        connection_character -> Text,
    }
}

diesel::table! {
    saved_states (id) {
        id -> Integer,
        workspace_id -> Integer,
        position -> Text,
        is_start -> Bool,
        is_final -> Bool,
    }
}

diesel::table! {
    saved_workspaces (id) {
        id -> Integer,
        user_id -> Integer,
        workspace_name -> Text,
        type_of_automata -> crate::models::TypeOfAutomataMapping,
        date_of_last_update -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        verified -> Bool,
        code -> Nullable<Text>,
    }
}

diesel::joinable!(saved_connections -> saved_workspaces (workspace_id));
diesel::joinable!(saved_states -> saved_workspaces (workspace_id));
diesel::joinable!(saved_workspaces -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    saved_connections,
    saved_states,
    saved_workspaces,
    users,
);
