use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use crate::models::servicio::*;
use crate::service::servicio as service;

pub fn servicio_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/servicios", get(listar).post(crear))
        .route("/api/servicios/{id}", get(obtener).put(actualizar).delete(eliminar))
        .with_state(pool)
}

async fn listar(State(pool): State<PgPool>) -> Result<Json<Vec<Servicio>>, (StatusCode, String)> {
    service::listar_servicios(&pool).await.map(Json).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn obtener(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Servicio>, (StatusCode, String)> {
    service::obtener_servicio_por_id(&pool, id).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}

async fn crear(State(pool): State<PgPool>, Json(dto): Json<CreateServicio>) -> Result<(StatusCode, Json<Servicio>), (StatusCode, String)> {
    service::crear_servicio(&pool, dto).await
        .map(|s| (StatusCode::CREATED, Json(s)))
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(dto): Json<CreateServicio>) -> Result<Json<Servicio>, (StatusCode, String)> {
    service::actualizar_servicio(&pool, id, dto).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::BAD_REQUEST, e) })
}

async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    service::eliminar_servicio(&pool, id).await
        .map(|msg| (StatusCode::OK, Json(serde_json::json!({"message": msg}))))
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}
