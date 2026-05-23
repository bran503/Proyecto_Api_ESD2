use sqlx::PgPool;
use crate::models::reparacion::{Reparacion, CreateReparacion};

pub struct ReparacionRepository;

impl ReparacionRepository {
    pub async fn create(pool: &PgPool, dto: &CreateReparacion) -> Result<Reparacion, sqlx::Error> {
        sqlx::query_as::<_, Reparacion>(
            "INSERT INTO Reparaciones (id_vehiculo, id_mecanico, id_servicio, estado)
             VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(dto.id_vehiculo)
        .bind(dto.id_mecanico)
        .bind(dto.id_servicio)
        .bind(&dto.estado)
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Reparacion>, sqlx::Error> {
        sqlx::query_as::<_, Reparacion>("SELECT * FROM Reparaciones ORDER BY id_reparacion DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Reparacion>, sqlx::Error> {
        sqlx::query_as::<_, Reparacion>("SELECT * FROM Reparaciones WHERE id_reparacion = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &PgPool, id: i32, dto: &CreateReparacion) -> Result<Option<Reparacion>, sqlx::Error> {
        sqlx::query_as::<_, Reparacion>(
            "UPDATE Reparaciones SET id_vehiculo = $1, id_mecanico = $2, id_servicio = $3, estado = $4
             WHERE id_reparacion = $5 RETURNING *"
        )
        .bind(dto.id_vehiculo)
        .bind(dto.id_mecanico)
        .bind(dto.id_servicio)
        .bind(&dto.estado)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM Reparaciones WHERE id_reparacion = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|res| res.rows_affected())
    }
}
