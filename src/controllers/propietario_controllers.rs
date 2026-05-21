use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use crate::models::propietario::*;
use crate::service::propietario as service;

pub fn propietario_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/propietarios", get(listar).post(crear))
        .route("/api/propietarios/{id}", get(obtener).put(actualizar).delete(eliminar))
        .with_state(pool)
}

async fn listar(State(pool): State<PgPool>) -> Result<Json<Vec<Propietario>>, (StatusCode, String)> {
    service::listar_propietarios(&pool).await.map(Json).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn obtener(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Propietario>, (StatusCode, String)> {
    service::obtener_propietario_por_id(&pool, id).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}

async fn crear(State(pool): State<PgPool>, Json(dto): Json<CreatePropietario>) -> Result<(StatusCode, Json<Propietario>), (StatusCode, String)> {
    service::crear_propietario(&pool, dto).await
        .map(|p| (StatusCode::CREATED, Json(p)))
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(dto): Json<CreatePropietario>) -> Result<Json<Propietario>, (StatusCode, String)> {
    service::actualizar_propietario(&pool, id, dto).await
        .map(Json)
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::BAD_REQUEST, e) })
}

async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    service::eliminar_propietario(&pool, id).await
        .map(|msg| (StatusCode::OK, Json(serde_json::json!({"message": msg}))))
        .map_err(|e| if e.contains("no encontrado") { (StatusCode::NOT_FOUND, e) } else { (StatusCode::INTERNAL_SERVER_ERROR, e) })
}
