use sqlx::PgPool;
use crate::models::mecanico::{Mecanico, CreateMecanico};
use crate::repository::MecanicoRepository;

pub async fn crear_mecanico(pool: &PgPool, dto: CreateMecanico) -> Result<Mecanico, String> {
    if dto.nombre.trim().is_empty() {
        return Err("El nombre es obligatorio".into());
    }
    MecanicoRepository::create(pool, &dto)
        .await
        .map_err(|e| format!("Error al crear mecánico: {}", e))
}

pub async fn listar_mecanicos(pool: &PgPool) -> Result<Vec<Mecanico>, String> {
    MecanicoRepository::get_all(pool)
        .await
        .map_err(|e| format!("Error al listar mecánicos: {}", e))
}

pub async fn obtener_mecanico_por_id(pool: &PgPool, id: i32) -> Result<Mecanico, String> {
    MecanicoRepository::get_by_id(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Mecánico no encontrado".to_string())
}

pub async fn actualizar_mecanico(pool: &PgPool, id: i32, dto: CreateMecanico) -> Result<Mecanico, String> {
    if dto.nombre.trim().is_empty() {
        return Err("El nombre es obligatorio".into());
    }
    MecanicoRepository::update(pool, id, &dto)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Mecánico no encontrado para actualizar".to_string())
}

pub async fn eliminar_mecanico(pool: &PgPool, id: i32) -> Result<String, String> {
    let rows = MecanicoRepository::delete(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?;
    if rows == 0 {
        return Err("Mecánico no encontrado".to_string());
    }
    Ok("Mecánico eliminado correctamente".to_string())
}
