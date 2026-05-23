use sqlx::PgPool;
use crate::models::servicio::{Servicio, CreateServicio};

pub struct ServicioRepository;

impl ServicioRepository {
    pub async fn create(pool: &PgPool, dto: &CreateServicio) -> Result<Servicio, sqlx::Error> {
        sqlx::query_as::<_, Servicio>(
            "INSERT INTO Servicios (descripcion_falla, precio_estimado) VALUES ($1, $2) RETURNING *"
        )
        .bind(&dto.descripcion_falla)
        .bind(&dto.precio_estimado)
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Servicio>, sqlx::Error> {
        sqlx::query_as::<_, Servicio>("SELECT * FROM Servicios ORDER BY id_servicio DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Servicio>, sqlx::Error> {
        sqlx::query_as::<_, Servicio>("SELECT * FROM Servicios WHERE id_servicio = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &PgPool, id: i32, dto: &CreateServicio) -> Result<Option<Servicio>, sqlx::Error> {
        sqlx::query_as::<_, Servicio>(
            "UPDATE Servicios SET descripcion_falla = $1, precio_estimado = $2 WHERE id_servicio = $3 RETURNING *"
        )
        .bind(&dto.descripcion_falla)
        .bind(&dto.precio_estimado)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM Servicios WHERE id_servicio = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|res| res.rows_affected())
    }
}
