use sqlx::PgPool;
use crate::models::propietario::{Propietario, CreatePropietario};
use crate::repository::PropietarioRepository;

pub async fn crear_propietario(pool: &PgPool, dto: CreatePropietario) -> Result<Propietario, String> {
    if dto.nombre.trim().is_empty() {
        return Err("El nombre es obligatorio".into());
    }
    PropietarioRepository::create(pool, &dto)
        .await
        .map_err(|e| format!("Error al crear propietario: {}", e))
}

pub async fn listar_propietarios(pool: &PgPool) -> Result<Vec<Propietario>, String> {
    PropietarioRepository::get_all(pool)
        .await
        .map_err(|e| format!("Error al listar propietarios: {}", e))
}

pub async fn obtener_propietario_por_id(pool: &PgPool, id: i32) -> Result<Propietario, String> {
    PropietarioRepository::get_by_id(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Propietario no encontrado".to_string())
}

pub async fn actualizar_propietario(pool: &PgPool, id: i32, dto: CreatePropietario) -> Result<Propietario, String> {
    if dto.nombre.trim().is_empty() {
        return Err("El nombre es obligatorio".into());
    }
    PropietarioRepository::update(pool, id, &dto)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Propietario no encontrado para actualizar".to_string())
}

pub async fn eliminar_propietario(pool: &PgPool, id: i32) -> Result<String, String> {
    let rows = PropietarioRepository::delete(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?;
    if rows == 0 {
        return Err("Propietario no encontrado".to_string());
    }
    Ok("Propietario eliminado correctamente".to_string())
}
