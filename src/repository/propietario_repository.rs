use sqlx::PgPool;
use crate::models::propietario::{Propietario, CreatePropietario};

pub struct PropietarioRepository;

impl PropietarioRepository {
    pub async fn create(pool: &PgPool, dto: &CreatePropietario) -> Result<Propietario, sqlx::Error> {
        sqlx::query_as::<_, Propietario>(
            "INSERT INTO Propietarios (nombre, dui, telefono) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&dto.nombre)
        .bind(&dto.dui)
        .bind(&dto.telefono)
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Propietario>, sqlx::Error> {
        sqlx::query_as::<_, Propietario>("SELECT * FROM Propietarios ORDER BY id_propietario DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Propietario>, sqlx::Error> {
        sqlx::query_as::<_, Propietario>("SELECT * FROM Propietarios WHERE id_propietario = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &PgPool, id: i32, dto: &CreatePropietario) -> Result<Option<Propietario>, sqlx::Error> {
        sqlx::query_as::<_, Propietario>(
            "UPDATE Propietarios SET nombre = $1, dui = $2, telefono = $3 WHERE id_propietario = $4 RETURNING *"
        )
        .bind(&dto.nombre)
        .bind(&dto.dui)
        .bind(&dto.telefono)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM Propietarios WHERE id_propietario = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|res| res.rows_affected())
    }
}
