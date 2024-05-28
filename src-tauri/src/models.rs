use diesel::prelude::*;
use diesel::query_builder::QueryId;
use diesel::sql_types::{Integer, Bool, VarChar};

// Any instances of a character being typed as a string is done due to the fact the deserialized datatype coming from the
// type scripty back-end, despite being a single character is always of type string since typescript does not have a character data type

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub code: Option<String>
}

#[derive(Queryable, QueryableByName, QueryId, Selectable, Insertable)]
#[diesel(table_name = crate::schema::saved_automata)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug)]
pub struct SavedAutomata {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub u_id: i32,
    #[diesel(sql_type = VarChar)]
    pub name: String
}

#[derive(Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::saved_states)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug)]
pub struct SavedState {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub automata_id: i32,
    #[diesel(sql_type = VarChar)]
    pub position: String,
    #[diesel(sql_type = Bool)]
    pub is_start: bool,
    #[diesel(sql_type = Bool)]
    pub is_final: bool
}

#[derive(Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::saved_connections)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug)]
pub struct SavedConnection {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub automata_id: i32,
    #[diesel(sql_type = VarChar)]
    pub start_point: String,
    #[diesel(sql_type = VarChar)]
    pub control_point_one: String,
    #[diesel(sql_type = VarChar)]
    pub control_point_two: String,
    #[diesel(sql_type = VarChar)]
    pub end_point: String,
    #[diesel(sql_type = VarChar)]
    pub connection_character: String
}


use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct State { 
    pub position: Coordinate,
    pub states_connected_to: HashMap<String, Vec<String>>,
    pub is_start: bool,
    pub is_final: bool,
    pub element: String
}
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    pub curve: BezierCurve,
    pub connection_character: String,
    pub element: String

}

#[derive(Debug, Deserialize, Serialize)]
pub struct BezierCurve {
    pub start_point: Coordinate,
    pub control_point_one: Coordinate,
    pub control_point_two: Coordinate,
    pub end_point: Coordinate

}

impl Coordinate{
    pub fn convert_coords_to_string(&self) -> String {
        let mut built_string = self.x.to_string();
        built_string.push(',');
        // Using reference here since push_str takes in &str as param
        built_string.push_str(&self.y.to_string());
        return built_string;
    }

}