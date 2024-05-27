use diesel::prelude::*;
use diesel::query_builder::QueryId;
use diesel::sql_types::{Integer, Bool, VarChar, Nullable};

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

#[derive(Queryable, Selectable, QueryableByName)]
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
    #[diesel(sql_type = Nullable<Integer>)]
    pub connected_state: Option<i32>,
    #[diesel(sql_type = VarChar)]
    pub connection_character: String,
    #[diesel(sql_type = Bool)]
    pub is_start: bool,
    #[diesel(sql_type = Bool)]
    pub is_final: bool
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::saved_connections)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug)]
pub struct SavedConnection {
    pub id: i32,
    pub automata_id: i32,
    pub start_coords: String,
    pub control_point_one: String,
    pub control_point_two: String,
    pub end_coords: String
}


use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize)]
pub struct State { 
    pub position: Coordinate,
    pub states_connected_to: HashMap<String, Vec<String>>,
    pub is_start: bool,
    pub is_final: bool,
    pub element: String
}
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32
}