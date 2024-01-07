#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub name: String,
}