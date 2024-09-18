// @generated automatically by Diesel CLI.
diesel::table! {
  saved_regex_workspaces (id) {
    id -> Integer,
    user_id -> Integer,
    regex_name -> Text,
    regex -> Text,
    date_of_last_update -> Timestamp,
  }
}

diesel::table! {
  saved_regular_automata_connections (id) {
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
  saved_regular_automata_workspaces (id) {
        id -> Integer,
        user_id -> Integer,
        workspace_name -> Text,
        type_of_automata -> crate::miscellaneous::database_models_and_utilities::TypeOfAutomataMapping,
        date_of_last_update -> Timestamp,
        alphabet -> Text,
        should_show_string_traversal -> Bool,
        should_strict_check -> Bool,
        default_connection_character -> Text,
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
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        verified -> Bool,
        number_of_untitled_regular_automata_workspaces -> Integer,
        number_of_untitled_regex_workspaces -> Integer,
        code -> Nullable<Text>,
    }
}

diesel::joinable!(saved_regex_workspaces -> users (user_id));
diesel::joinable!(saved_regular_automata_connections -> saved_regular_automata_workspaces (workspace_id));
diesel::joinable!(saved_regular_automata_workspaces -> users (user_id));
diesel::joinable!(saved_states -> saved_regular_automata_workspaces (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    saved_regex_workspaces,
    saved_regular_automata_connections,
    saved_regular_automata_workspaces,
    saved_states,
    users,
);
