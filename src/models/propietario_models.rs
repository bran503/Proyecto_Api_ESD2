use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Propietario {
    pub id_propietario: i32,
    pub nombre: String,
    pub dui: Option<String>,
    pub telefono: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePropietario {
    pub nombre: String,
    pub dui: Option<String>,
    pub telefono: Option<String>,
}
