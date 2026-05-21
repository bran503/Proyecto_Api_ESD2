#[path = "propietario_repository.rs"]
pub mod propietario;
#[path = "vehiculo_repository.rs"]
pub mod vehiculo;
#[path = "mecanico_repository.rs"]
pub mod mecanico;
#[path = "servicio_repository.rs"]
pub mod servicio;
#[path = "reparacion_repository.rs"]
pub mod reparacion;

pub use propietario::PropietarioRepository;
pub use vehiculo::VehiculoRepository;
pub use mecanico::MecanicoRepository;
pub use servicio::ServicioRepository;
pub use reparacion::ReparacionRepository;
