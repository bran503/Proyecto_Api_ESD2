use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Vehiculo {
    pub id_vehiculo: i32,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub id_propietario: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateVehiculo {
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub id_propietario: i32,
}
