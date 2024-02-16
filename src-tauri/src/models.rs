use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

use crate::schema::users;
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct Register<'a> {
    pub email: &'a str,
    pub password: &'a str,
}