use diesel::prelude::*;

use crate::models;
use uuid::Uuid;
use chrono::Utc;

pub fn get_all_pos(conn: &PgConnection) -> Result<Vec<models::Position>, diesel::result::Error> {
    use crate::schema::positions::dsl::*;

    let pos = positions.limit(30).load::<models::Position>(conn).expect("Error loading pos");

    Ok(pos)
}

pub fn insert_new_pos(pos: &models::MapPos, conn: &PgConnection) -> Result<models::Position, diesel::result::Error> {
    use crate::schema::positions::dsl::*;

    let new_pos = models::Position {
        id: Uuid::new_v4(),
        date: Utc::now().naive_utc(),
        latitude: pos.latitude,
        longitude: pos.longitude
    };

    diesel::insert_into(positions).values(&new_pos).execute(conn)?;

    Ok(new_pos)
}