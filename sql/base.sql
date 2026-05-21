CREATE TABLE Propietarios (
    id_propietario SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    dui VARCHAR(10) UNIQUE,
    telefono VARCHAR(20)
);

CREATE TABLE Vehiculos (
    id_vehiculo SERIAL PRIMARY KEY,
    placa VARCHAR(15) UNIQUE NOT NULL,
    marca VARCHAR(50),
    modelo VARCHAR(50),
    id_propietario INT REFERENCES Propietarios(id_propietario)
);

CREATE TABLE Mecanicos (
    id_mecanico SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    cargo VARCHAR(50),
    salario DECIMAL(10,2)
);

CREATE TABLE Servicios (
    id_servicio SERIAL PRIMARY KEY,
    descripcion_falla TEXT NOT NULL,
    precio_estimado DECIMAL(10,2)
);

CREATE TABLE Reparaciones (
    id_reparacion SERIAL PRIMARY KEY,
    id_vehiculo INT REFERENCES Vehiculos(id_vehiculo),
    id_mecanico INT REFERENCES Mecanicos(id_mecanico),
    id_servicio INT REFERENCES Servicios(id_servicio),
    fecha_entrada DATE DEFAULT CURRENT_DATE,
    estado VARCHAR(30) -- Ejemplo: 'En Proceso', 'Terminado'
);