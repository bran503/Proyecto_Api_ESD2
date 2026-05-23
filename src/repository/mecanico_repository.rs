use sqlx::PgPool;
use crate::models::mecanico::{Mecanico, CreateMecanico};

pub struct MecanicoRepository;

impl MecanicoRepository {
    pub async fn create(pool: &PgPool, dto: &CreateMecanico) -> Result<Mecanico, sqlx::Error> {
        sqlx::query_as::<_, Mecanico>(
            "INSERT INTO Mecanicos (nombre, cargo, salario) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&dto.nombre)
        .bind(&dto.cargo)
        .bind(&dto.salario)
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Mecanico>, sqlx::Error> {
        sqlx::query_as::<_, Mecanico>("SELECT * FROM Mecanicos ORDER BY id_mecanico DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Mecanico>, sqlx::Error> {
        sqlx::query_as::<_, Mecanico>("SELECT * FROM Mecanicos WHERE id_mecanico = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &PgPool, id: i32, dto: &CreateMecanico) -> Result<Option<Mecanico>, sqlx::Error> {
        sqlx::query_as::<_, Mecanico>(
            "UPDATE Mecanicos SET nombre = $1, cargo = $2, salario = $3 WHERE id_mecanico = $4 RETURNING *"
        )
        .bind(&dto.nombre)
        .bind(&dto.cargo)
        .bind(&dto.salario)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM Mecanicos WHERE id_mecanico = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|res| res.rows_affected())
    }
}
