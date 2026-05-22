use sqlx::PgPool;
use crate::models::vehiculo::{Vehiculo, CreateVehiculo};
use crate::repository::VehiculoRepository;
use crate::repository::PropietarioRepository;

pub async fn crear_vehiculo(pool: &PgPool, dto: CreateVehiculo) -> Result<Vehiculo, String> {
    if dto.placa.trim().is_empty() {
        return Err("La placa es obligatoria".into());
    }
    if PropietarioRepository::get_by_id(pool, dto.id_propietario)
        .await
        .map_err(|e| format!("Error validando propietario: {}", e))?
        .is_none()
    {
        return Err("El propietario especificado no existe".to_string());
    }
    VehiculoRepository::create(pool, &dto)
        .await
        .map_err(|e| format!("Error al crear vehículo: {}", e))
}

pub async fn listar_vehiculos(pool: &PgPool) -> Result<Vec<Vehiculo>, String> {
    VehiculoRepository::get_all(pool)
        .await
        .map_err(|e| format!("Error al listar vehículos: {}", e))
}

pub async fn obtener_vehiculo_por_id(pool: &PgPool, id: i32) -> Result<Vehiculo, String> {
    VehiculoRepository::get_by_id(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Vehículo no encontrado".to_string())
}

pub async fn actualizar_vehiculo(pool: &PgPool, id: i32, dto: CreateVehiculo) -> Result<Vehiculo, String> {
    if dto.placa.trim().is_empty() {
        return Err("La placa es obligatoria".into());
    }
    VehiculoRepository::update(pool, id, &dto)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Vehículo no encontrado para actualizar".to_string())
}

pub async fn eliminar_vehiculo(pool: &PgPool, id: i32) -> Result<String, String> {
    let rows = VehiculoRepository::delete(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?;
    if rows == 0 {
        return Err("Vehículo no encontrado".to_string());
    }
    Ok("Vehículo eliminado correctamente".to_string())
}
