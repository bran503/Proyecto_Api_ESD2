use sqlx::PgPool;
use crate::models::vehiculo::{Vehiculo, CreateVehiculo};

pub struct VehiculoRepository;

impl VehiculoRepository {
    pub async fn create(pool: &PgPool, dto: &CreateVehiculo) -> Result<Vehiculo, sqlx::Error> {
        sqlx::query_as::<_, Vehiculo>(
            "INSERT INTO Vehiculos (placa, marca, modelo, id_propietario) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(&dto.placa)
        .bind(&dto.marca)
        .bind(&dto.modelo)
        .bind(dto.id_propietario)
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Vehiculo>, sqlx::Error> {
        sqlx::query_as::<_, Vehiculo>("SELECT * FROM Vehiculos ORDER BY id_vehiculo DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Vehiculo>, sqlx::Error> {
        sqlx::query_as::<_, Vehiculo>("SELECT * FROM Vehiculos WHERE id_vehiculo = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &PgPool, id: i32, dto: &CreateVehiculo) -> Result<Option<Vehiculo>, sqlx::Error> {
        sqlx::query_as::<_, Vehiculo>(
            "UPDATE Vehiculos SET placa = $1, marca = $2, modelo = $3, id_propietario = $4 WHERE id_vehiculo = $5 RETURNING *"
        )
        .bind(&dto.placa)
        .bind(&dto.marca)
        .bind(&dto.modelo)
        .bind(dto.id_propietario)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM Vehiculos WHERE id_vehiculo = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|res| res.rows_affected())
    }
}
