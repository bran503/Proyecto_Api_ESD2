use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Servicio {
    pub id_servicio: i32,
    pub descripcion_falla: String,
    pub precio_estimado: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct CreateServicio {
    pub descripcion_falla: String,
    pub precio_estimado: Option<Decimal>,
}
