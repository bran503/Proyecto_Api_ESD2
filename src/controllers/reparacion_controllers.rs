use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use crate::models::reparacion::*;
use crate::service::reparacion as service;

pub fn reparacion_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/reparaciones", get(listar).post(crear))
        .route("/api/reparaciones/{id}", get(obtener).put(actualizar).delete(eliminar))
        .with_state(pool)
}

async fn listar(State(pool): State<PgPool>) -> Result<Json<Vec<Reparacion>>, (StatusCode, String)> {
    service::listar_reparaciones(&pool).await.map(Json).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn obtener(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Reparacion>, (StatusCode, String)> {
    service::obtener_reparacion_por_id(&pool, id).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrada") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}

async fn crear(State(pool): State<PgPool>, Json(dto): Json<CreateReparacion>) -> Result<(StatusCode, Json<Reparacion>), (StatusCode, String)> {
    service::crear_reparacion(&pool, dto).await
        .map(|r| (StatusCode::CREATED, Json(r)))
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(dto): Json<CreateReparacion>) -> Result<Json<Reparacion>, (StatusCode, String)> {
    service::actualizar_reparacion(&pool, id, dto).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrada") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::BAD_REQUEST, e) })
}

async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    service::eliminar_reparacion(&pool, id).await
        .map(|msg| (StatusCode::OK, Json(serde_json::json!({"message": msg}))))
        .map_err(|e| if e.contains("no encontrada") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}
