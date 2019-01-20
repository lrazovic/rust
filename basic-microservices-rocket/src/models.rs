#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::heroes;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable, Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "heroes"]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn create(hero: Hero, connection: &PgConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");
        heroes::table
            .order(heroes::id.desc())
            .first(connection)
            .unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<Hero> {
        heroes::table
            .order(heroes::id.asc())
            .load::<Hero>(connection)
            .unwrap()
    }

    pub fn update(id: i32, hero: Hero, connection: &PgConnection) -> bool {
        diesel::update(heroes::table.find(id))
            .set(&hero)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(heroes::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
