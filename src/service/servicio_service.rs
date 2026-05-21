use sqlx::PgPool;
use crate::models::servicio::{Servicio, CreateServicio};
use crate::repository::ServicioRepository;

pub async fn crear_servicio(pool: &PgPool, dto: CreateServicio) -> Result<Servicio, String> {
    if dto.descripcion_falla.trim().is_empty() {
        return Err("La descripción de la falla es obligatoria".into());
    }
    ServicioRepository::create(pool, &dto)
        .await
        .map_err(|e| format!("Error al crear servicio: {}", e))
}

pub async fn listar_servicios(pool: &PgPool) -> Result<Vec<Servicio>, String> {
    ServicioRepository::get_all(pool)
        .await
        .map_err(|e| format!("Error al listar servicios: {}", e))
}

pub async fn obtener_servicio_por_id(pool: &PgPool, id: i32) -> Result<Servicio, String> {
    ServicioRepository::get_by_id(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Servicio no encontrado".to_string())
}

pub async fn actualizar_servicio(pool: &PgPool, id: i32, dto: CreateServicio) -> Result<Servicio, String> {
    if dto.descripcion_falla.trim().is_empty() {
        return Err("La descripción de la falla es obligatoria".into());
    }
    ServicioRepository::update(pool, id, &dto)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Servicio no encontrado para actualizar".to_string())
}

pub async fn eliminar_servicio(pool: &PgPool, id: i32) -> Result<String, String> {
    let rows = ServicioRepository::delete(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?;
    if rows == 0 {
        return Err("Servicio no encontrado".to_string());
    }
    Ok("Servicio eliminado correctamente".to_string())
}
