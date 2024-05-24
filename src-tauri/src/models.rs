use diesel::prelude::*;

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

use crate::schema::users;
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct Register<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize)]
pub struct State { 
    pub position: Coordinate,
    pub states_connected_to: HashMap<String, Vec<String>>,
    pub is_final: bool,
    pub element: String
}
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32
}


pub struct Stae {

    


}