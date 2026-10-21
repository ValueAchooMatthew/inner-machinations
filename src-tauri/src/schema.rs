// @generated automatically by Diesel CLI.

diesel::table! {
    saved_connections (id) {
        id -> Integer,
        workspace_id -> Integer,
        connection_id -> Integer,
        from_state -> Integer,
        to_state -> Integer,
        connection_character -> Text,
        control_point_one_x -> Integer,
        control_point_one_y -> Integer,
        control_point_two_x -> Integer,
        control_point_two_y -> Integer,
    }
}

diesel::table! {
    saved_states (id) {
        id -> Integer,
        workspace_id -> Integer,
        state_id -> Integer,
        x -> Integer,
        y -> Integer,
        is_start -> Bool,
        is_final -> Bool,
    }
}

diesel::table! {
    saved_workspaces (id) {
        id -> Integer,
        workspace_name -> Text,
        type_of_automata -> crate::models::TypeOfAutomataMapping,
        date_of_last_update -> Timestamp,
        alphabet -> Text,
        should_show_string_traversal -> Bool,
        should_strict_check -> Bool,
        default_connection_character -> Text,
    }
}

diesel::joinable!(saved_connections -> saved_workspaces (workspace_id));
diesel::joinable!(saved_states -> saved_workspaces (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    saved_connections,
    saved_states,
    saved_workspaces,
);
