use sqlx::PgPool;
use crate::models::reparacion::{Reparacion, CreateReparacion};
use crate::repository::ReparacionRepository;
use crate::repository::VehiculoRepository;
use crate::repository::MecanicoRepository;
use crate::repository::ServicioRepository;

pub async fn crear_reparacion(pool: &PgPool, dto: CreateReparacion) -> Result<Reparacion, String> {
    let vehiculo_existe = VehiculoRepository::get_by_id(pool, dto.id_vehiculo)
        .await.map_err(|e| e.to_string())?.is_some();
    let mecanico_existe = MecanicoRepository::get_by_id(pool, dto.id_mecanico)
        .await.map_err(|e| e.to_string())?.is_some();
    let servicio_existe = ServicioRepository::get_by_id(pool, dto.id_servicio)
        .await.map_err(|e| e.to_string())?.is_some();

    if !vehiculo_existe { return Err("El vehículo especificado no existe".into()); }
    if !mecanico_existe { return Err("El mecánico especificado no existe".into()); }
    if !servicio_existe { return Err("El servicio especificado no existe".into()); }

    ReparacionRepository::create(pool, &dto)
        .await
        .map_err(|e| format!("Error al crear reparación: {}", e))
}

pub async fn listar_reparaciones(pool: &PgPool) -> Result<Vec<Reparacion>, String> {
    ReparacionRepository::get_all(pool)
        .await
        .map_err(|e| format!("Error al listar reparaciones: {}", e))
}

pub async fn obtener_reparacion_por_id(pool: &PgPool, id: i32) -> Result<Reparacion, String> {
    ReparacionRepository::get_by_id(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Reparación no encontrada".to_string())
}

pub async fn actualizar_reparacion(pool: &PgPool, id: i32, dto: CreateReparacion) -> Result<Reparacion, String> {
    ReparacionRepository::update(pool, id, &dto)
        .await
        .map_err(|e| format!("Error BD: {}", e))?
        .ok_or("Reparación no encontrada para actualizar".to_string())
}

pub async fn eliminar_reparacion(pool: &PgPool, id: i32) -> Result<String, String> {
    let rows = ReparacionRepository::delete(pool, id)
        .await
        .map_err(|e| format!("Error BD: {}", e))?;
    if rows == 0 {
        return Err("Reparación no encontrada".to_string());
    }
    Ok("Reparación eliminada correctamente".to_string())
}
