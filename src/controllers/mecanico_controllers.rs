use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use crate::models::mecanico::*;
use crate::service::mecanico as service;

pub fn mecanico_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/mecanicos", get(listar).post(crear))
        .route("/api/mecanicos/{id}", get(obtener).put(actualizar).delete(eliminar))
        .with_state(pool)
}

async fn listar(State(pool): State<PgPool>) -> Result<Json<Vec<Mecanico>>, (StatusCode, String)> {
    service::listar_mecanicos(&pool).await.map(Json).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn obtener(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Mecanico>, (StatusCode, String)> {
    service::obtener_mecanico_por_id(&pool, id).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}

async fn crear(State(pool): State<PgPool>, Json(dto): Json<CreateMecanico>) -> Result<(StatusCode, Json<Mecanico>), (StatusCode, String)> {
    service::crear_mecanico(&pool, dto).await
        .map(|m| (StatusCode::CREATED, Json(m)))
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(dto): Json<CreateMecanico>) -> Result<Json<Mecanico>, (StatusCode, String)> {
    service::actualizar_mecanico(&pool, id, dto).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::BAD_REQUEST, e) })
}

async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    service::eliminar_mecanico(&pool, id).await
        .map(|msg| (StatusCode::OK, Json(serde_json::json!({"message": msg}))))
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}
