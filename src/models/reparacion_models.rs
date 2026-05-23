use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Reparacion {
    pub id_reparacion: i32,
    pub id_vehiculo: i32,
    pub id_mecanico: i32,
    pub id_servicio: i32,
    pub fecha_entrada: Option<NaiveDate>,
    pub estado: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReparacion {
    pub id_vehiculo: i32,
    pub id_mecanico: i32,
    pub id_servicio: i32,
    pub fecha_entrada: Option<NaiveDate>,
    pub estado: Option<String>,
}
