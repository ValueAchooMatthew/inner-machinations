use std::collections::HashMap;

use app::encrypt_user_data;
use app::models::{Connection, User};

use app::schema::saved_connections::{self};
use app::schema::users;
use app::{establish_connection, models::State};

use::app::models::SavedAutomata;
use::app::schema::saved_states;
use::app::schema::saved_automata;

use diesel::{ExpressionMethods, MysqlConnection};
use magic_crypt::new_magic_crypt;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

fn get_workspace(workspace_name: &String, user_id: &i32, conn: &mut MysqlConnection) -> SavedAutomata {
    let workspace: SavedAutomata = match saved_automata::dsl::saved_automata
        .filter(saved_automata::u_id.eq(user_id))
        .filter(saved_automata::dsl::name.eq(&workspace_name))
        .limit(1)
        .get_result::<SavedAutomata>(conn) {
            Ok(workspace) =>{println!("Retrieving workspace {}", &workspace_name); workspace},
            Err(_) => {
            
            let new_saved_automata = (
                saved_automata::u_id.eq(&user_id), 
                saved_automata::name.eq(&workspace_name)
            );

            diesel::insert_into(saved_automata::table)
                .values(new_saved_automata)
                .execute(conn)
                .expect("There was an error creating a new workspace");

            println!("Creating new workspace {}", &workspace_name);

            saved_automata::dsl::saved_automata
                .filter(saved_automata::dsl::name.eq(workspace_name))
                .limit(1)
                .get_result::<SavedAutomata>(conn)
                .expect("Big whoopsie alert")
            }
        };
    return workspace;
}



fn save_states_to_db(workspace_id: &i32, states: &HashMap<String, State>, conn: &mut MysqlConnection) {
    // First step is to remove all existing states corresponding to the workspace
    diesel::delete(saved_states::table)
        .filter(saved_states::automata_id.eq(&workspace_id))
        .execute(conn)
        .expect("There was an error deleting existing states from the states table");
    
    // Second step is to add the states from the hashmap and associate each one to the existing state id
    // A state can have 0 inclusive connected states, so we will make one entry for the state itself with no connections,
    // and one for each state it is conencted to

    let mut states_to_be_inserted =  vec![];

    for (state_pos_key, state) in states {
        // Inserting state in the case a state has no connections
        states_to_be_inserted.push((
            saved_states::automata_id.eq(workspace_id),
            saved_states::position.eq(state_pos_key),
            saved_states::is_start.eq(state.is_start),
            saved_states::is_final.eq(state.is_final),
        ));

    }

    diesel::insert_into(saved_states::table)
        .values(&states_to_be_inserted)
        .execute(conn)
        .expect("There was an error inserting the states");


}


fn save_connections_to_db(workspace_id: &i32, connections: &Vec<Connection>, conn: &mut MysqlConnection) {

    // First delete all existing connections relating to the current automata
    diesel::delete(saved_connections::table)
        .filter(saved_connections::automata_id.eq(workspace_id))
        .execute(conn)
        .expect("There was an error deleting the previous connections from the saved connections table");

    let mut connections_to_be_inserted = vec![];

    for connection in connections{
        let connection_to_be_inserted = (
            saved_connections::automata_id.eq(workspace_id),
            saved_connections::connection_character.eq(&connection.connection_character),
            saved_connections::start_point.eq(connection.curve.start_point.convert_coords_to_string()),
            saved_connections::control_point_one.eq(connection.curve.control_point_one.convert_coords_to_string()),
            saved_connections::control_point_two.eq(connection.curve.control_point_two.convert_coords_to_string()),
            saved_connections::end_point.eq(connection.curve.control_point_two.convert_coords_to_string())
        );

        connections_to_be_inserted.push(connection_to_be_inserted);
    }

    // Second step is inserting connections into connections table

    diesel::insert_into(saved_connections::table)
        .values(connections_to_be_inserted)
        .execute(conn)
        .expect("There was an error inserting the new connections into the connections table");

}


#[tauri::command]
pub fn save_workspace(workspace_name: String, states: HashMap<String, State>, email: String, connections: Vec<Connection>){

    let mut conn = establish_connection();
    let key = std::env::var("ENCRYPTION_KEY")
        .expect("Encryption Key must be set as a .env variable");
    let cipher = new_magic_crypt!(&key, 256);

    let [encrypted_user_email, _ ] = encrypt_user_data(&cipher, &email, "");


    let user: User = users::dsl::users
        .filter(users::dsl::email.eq(&encrypted_user_email))
        .limit(1)
        .get_result::<User>(&mut conn)
        .expect("There was an error finding the user's profile");

    let user_id = user.id;

    let workspace: SavedAutomata = get_workspace(&workspace_name, &user_id, &mut conn);

    save_states_to_db(&workspace.id, &states, &mut conn);
    save_connections_to_db(&workspace.id, &connections, &mut conn);
    println!("Saved!");


}
