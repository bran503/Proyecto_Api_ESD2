mod config;
mod models;
mod repository;
mod service;
mod controllers;
mod utils;

use controllers::{
    propietario_router,
    vehiculo_router,
    mecanico_router,
    servicio_router,
    reparacion_router,
};
use config::init_db_pool; 

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor iniciado en http://{direccion}");

    let pool = init_db_pool().await;

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}

fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    propietario_router(pool.clone())
        .merge(vehiculo_router(pool.clone()))
        .merge(mecanico_router(pool.clone()))
        .merge(servicio_router(pool.clone()))
        .merge(reparacion_router(pool.clone()))
}