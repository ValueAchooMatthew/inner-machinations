use std::collections::HashMap;

use app::models::User;
use app::schema::saved_states::connected_state;
use app::schema::users;
use app::{establish_connection, models::State};

use::app::models::{SavedState, SavedAutomata, SavedConnection};
use::app::schema::saved_states;
use::app::schema::saved_automata;
use::app::schema::saved_connections;
use diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

#[tauri::command]
pub fn save_workspace(workspace_name: String, states: HashMap<String, State>, email: String){

    let mut connection = establish_connection();

    let user: User = users::dsl::users
        .filter(users::dsl::email.eq(email))
        .limit(1)
        .get_result::<User>(&mut connection)
        .expect("There was an error finding the user's profile");

    let user_id = user.id;

    // Ugly code but as far as i'm aware i need to reretrieve the table after inserting into
    let workspace: SavedAutomata = match saved_automata::dsl::saved_automata
        .filter(saved_automata::dsl::name.eq(&workspace_name))
        .limit(1)
        .get_result::<SavedAutomata>(&mut connection) {
            Ok(workspace) =>{println!("Retrieving workspace {}", &workspace_name); workspace},
            Err(_) => {
            
            let new_saved_automata = 
            (saved_automata::u_id.eq(&user_id), saved_automata::name.eq(&workspace_name));

            diesel::insert_into(saved_automata::table)
                .values(new_saved_automata)
                .execute(&mut connection)
                .expect("There was an error creating a new workspace");

            println!("Creating new workspace {}", &workspace_name);

            saved_automata::dsl::saved_automata
                .filter(saved_automata::dsl::name.eq(workspace_name))
                .limit(1)
                .get_result::<SavedAutomata>(&mut connection)
                .expect("Big whoopsie alert")
            }
        };


    println!("{:?}", workspace);

    for (state_key, state) in states {

        let new_state =
        (saved_states::is_final.eq(state.is_final), saved_states::is_start.eq(state.is_start), saved_states::position.eq(&state_key));
        diesel::insert_into(saved_states::table)
            .values(new_state)
            .execute(&mut connection)
            .expect("There was an error inserting a new state");

        for (connection_character, states_connected_by_char) in state.states_connected_to {
            
            for con_state in states_connected_by_char {
                
                let mut connected_state_entry: Vec<SavedState> = saved_states::dsl::saved_states
                    .filter(saved_states::dsl::position.eq(con_state))
                    .limit(1)
                    .load(&mut connection)
                    .expect("failed to load the connection");

                let connected_state_id = connected_state_entry.pop().unwrap().id;

                let new_state =
                (saved_states::is_final.eq(&state.is_final), saved_states::is_start.eq(state.is_start), 
                saved_states::position.eq(&state_key), saved_states::connection_character.eq(&connection_character),
                saved_states::connected_state.eq(connected_state_id));

                diesel::insert_into(saved_states::table)
                    .values(new_state)
                    .execute(&mut connection)
                    .expect("There was an error inserting a new state");
            }

        }


    }

}
