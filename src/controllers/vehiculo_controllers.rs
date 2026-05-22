use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use crate::models::vehiculo::*;
use crate::service::vehiculo as service;

pub fn vehiculo_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/vehiculos", get(listar).post(crear))
        .route("/api/vehiculos/{id}", get(obtener).put(actualizar).delete(eliminar))
        .with_state(pool)
}

async fn listar(State(pool): State<PgPool>) -> Result<Json<Vec<Vehiculo>>, (StatusCode, String)> {
    service::listar_vehiculos(&pool).await.map(Json).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn obtener(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Vehiculo>, (StatusCode, String)> {
    service::obtener_vehiculo_por_id(&pool, id).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}

async fn crear(State(pool): State<PgPool>, Json(dto): Json<CreateVehiculo>) -> Result<(StatusCode, Json<Vehiculo>), (StatusCode, String)> {
    service::crear_vehiculo(&pool, dto).await
        .map(|v| (StatusCode::CREATED, Json(v)))
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(dto): Json<CreateVehiculo>) -> Result<Json<Vehiculo>, (StatusCode, String)> {
    service::actualizar_vehiculo(&pool, id, dto).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::BAD_REQUEST, e) })
}

async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    service::eliminar_vehiculo(&pool, id).await
        .map(|msg| (StatusCode::OK, Json(serde_json::json!({"message": msg}))))
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}
