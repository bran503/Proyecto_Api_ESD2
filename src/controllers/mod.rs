#[path = "propietario_controllers.rs"]
pub mod propietario;
#[path = "vehiculo_controllers.rs"]
pub mod vehiculo;
#[path = "mecanico_controllers.rs"]
pub mod mecanico;
#[path = "servicio_controllers.rs"]
pub mod servicio;
#[path = "reparacion_controllers.rs"]
pub mod reparacion;

pub use propietario::propietario_router;
pub use vehiculo::vehiculo_router;
pub use mecanico::mecanico_router;
pub use servicio::servicio_router;
pub use reparacion::reparacion_router;
