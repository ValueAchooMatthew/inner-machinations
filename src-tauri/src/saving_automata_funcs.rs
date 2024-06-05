use std::collections::HashMap;

use app::encrypt_user_data;
use app::models::{BezierCurve, Connection, Coordinate, SavedConnection, SavedState, User};

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

fn get_user_id(email: &String, conn: &mut MysqlConnection) -> i32 {
    let key = std::env::var("ENCRYPTION_KEY")
        .expect("Encryption Key must be set as a .env variable");
    let cipher = new_magic_crypt!(&key, 256);

    let [encrypted_user_email, _ ] = encrypt_user_data(&cipher, &email, "");

    let user: User = users::dsl::users
        .filter(users::dsl::email.eq(&encrypted_user_email))
        .limit(1)
        .get_result::<User>(conn)
        .expect("There was an error finding the user's profile");

    user.id

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
            saved_connections::end_point.eq(connection.curve.end_point.convert_coords_to_string())
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

    let user_id = get_user_id(&email, &mut conn);

    let workspace: SavedAutomata = get_workspace(&workspace_name, &user_id, &mut conn);

    save_states_to_db(&workspace.id, &states, &mut conn);
    save_connections_to_db(&workspace.id, &connections, &mut conn);
    println!("Saved!");

}

#[tauri::command]
pub fn delete_workspace(workspace_name: String, email: String){

    let mut conn: MysqlConnection = establish_connection();

    let user_id = get_user_id(&email, &mut conn);

    let workspace: SavedAutomata = get_workspace(&workspace_name, &user_id, &mut conn);

    diesel::delete(saved_states::table)
        .filter(saved_states::automata_id.eq(workspace.id))
        .execute(&mut conn)
        .expect("There was an error deleting the old workspace's previous states");

    diesel::delete(saved_connections::table)
        .filter(saved_connections::automata_id.eq(workspace.id))
        .execute(&mut conn)
        .expect("There was an error deleting the old workspace's previous connections");

    diesel::delete(saved_automata::table)
        .filter(saved_automata::u_id.eq(&user_id))
        .filter(saved_automata::name.eq(workspace_name))
        .execute(&mut conn)
        .expect("There was an error deleting the workspace from the db");

}

#[tauri::command]
pub fn retrieve_workspace_data(workspace_name: String, email: String) -> (Option<usize>, Vec<State>, Vec<Connection>, HashMap<String, State>) {
    
    let mut conn: MysqlConnection = establish_connection();
    
    let user_id = get_user_id(&email, &mut conn);

    let workspace = get_workspace(&workspace_name, &user_id, &mut conn);
    
    // First get the states and connections from the database
    let retrieved_states: Vec<SavedState> = saved_states::dsl::saved_states
        .filter(saved_states::automata_id.eq(&workspace.id))
        .get_results::<SavedState>(&mut conn)
        .expect("There was an issue getting the workspace's states");


    let mut start_state_index: Option<usize> = None;
    let mut state_connections: HashMap<String, State> = HashMap::new();
    let mut connections: Vec<Connection> = vec![];
    let mut states: Vec<State> = vec![];

    for (index, state) in retrieved_states.iter().enumerate() {
        if state.is_start {
            start_state_index = Some(index);
        }
        let parsed_state = parse_saved_state_to_regular_state(state, &workspace, &mut conn);
        state_connections.insert(state.position.to_owned(), parsed_state.to_owned());
        states.push(parsed_state.to_owned());
    }

    let retrieved_connections: Vec<SavedConnection> = saved_connections::dsl::saved_connections
        .filter(saved_connections::dsl::automata_id.eq(&workspace.id))
        .get_results::<SavedConnection>(&mut conn)
        .expect("There was an issue getting the workspace's states");

    for connection in retrieved_connections {
        let parsed_connection = Connection {
            curve: BezierCurve {
                start_point: parse_position_key_to_coordinate(&connection.start_point),
                control_point_one: parse_position_key_to_coordinate(&connection.control_point_one),
                control_point_two: parse_position_key_to_coordinate(&connection.control_point_two),
                end_point: parse_position_key_to_coordinate(&connection.end_point),
            },
            connection_character: connection.connection_character,
            element: "Connection".to_owned()
        };
        connections.push(parsed_connection);
    };

    return (start_state_index, states, connections, state_connections);


}

fn parse_saved_state_to_regular_state(state: &SavedState, workspace: &SavedAutomata, conn: &mut MysqlConnection) -> State {
    let mut parsed_state = State {
        position: parse_position_key_to_coordinate(&state.position), 
        states_connected_to: HashMap::<String, Vec<String>>::new(),
        is_start: state.is_start,
        is_final: state.is_final,
        element: "State".to_owned()
    };

    let states_connected_to_given_state: Vec<SavedConnection> = saved_connections::dsl::saved_connections
        .filter(saved_connections::dsl::automata_id.eq(&workspace.id))
        .filter(saved_connections::dsl::start_point.eq(&state.position))
        .get_results::<SavedConnection>(conn)
        .expect("There was an issue getting the workspace's states");

    for connected_state in states_connected_to_given_state {

        let binding: Vec<String> = vec![];
        let mut current_connections = match parsed_state.states_connected_to
            .get_mut(&connected_state.connection_character){
                Some(states) => states,
                None => &binding
            }
            .to_owned();

        current_connections.push(connected_state.end_point);

        parsed_state.states_connected_to.insert(
            connected_state.connection_character, 
            current_connections);
    }
    parsed_state
}

fn parse_position_key_to_coordinate(key: &String) -> Coordinate {

    let split_key: Vec<&str> = key.split(",").collect();
    let coord_x = split_key
        .get(0)
        .expect("There was an error parsing the state's coordinates")
        .to_owned();

    let coord_y = split_key
        .get(1)
        .expect("There was an error parsing the state's coordinates")
        .to_owned();

    Coordinate { 
        x: coord_x.parse::<i32>().unwrap(), 
        y: coord_y.parse::<i32>().unwrap()  
    }

}

#[tauri::command]
pub fn get_users_saved_workspaces(email: String) -> Vec<String> {

    let mut conn = establish_connection();

    let user_id = get_user_id(&email, &mut conn);

    let retrieved_workspaces: Vec<SavedAutomata> = saved_automata::dsl::saved_automata
        .filter(saved_automata::dsl::u_id.eq(&user_id))
        .get_results(&mut conn)
        .expect("There was an error retrieving the user's saved workspaces");

    let workspace_names: Vec<String> = retrieved_workspaces
        .iter()
        .map(|workspace| workspace.name.to_owned())
        .collect();

    return workspace_names.to_owned();

}
