use serde::{Deserialize, Serialize};

use crate::schema::positions;
use chrono::{NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
pub struct Position {
    pub id: Uuid,
    pub date: NaiveDateTime,
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPos{
    pub latitude: f64,
    pub longitude: f64
}
