use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};


#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::trip)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Trip {
    pub id: i64,
    pub name: String,
    pub description: String
}

#[derive(Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::trip)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct NewTrip {
    pub name: String,
    pub description: String
}
