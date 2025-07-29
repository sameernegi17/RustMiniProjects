
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User{
    pub id: i64,
    pub name: String,
    pub email:String
}