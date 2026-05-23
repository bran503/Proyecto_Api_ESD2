use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Mecanico {
    pub id_mecanico: i32,
    pub nombre: String,
    pub cargo: Option<String>,
    pub salario: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMecanico {
    pub nombre: String,
    pub cargo: Option<String>,
    pub salario: Option<Decimal>,
}
