#![no_std]

// Importa las dependencias necesarias
use sails_rs::prelude::*;

// Importa los módulos 
pub mod states;
pub mod services;

// Importa el servicio que se va a utilizar en el programa
use services::low_file_service::LowFileService;  // Cambiado a LowFileService


// Programa principal del contrato
pub struct LowFileProgram;

// Programa LowFile, puede alojar uno o más servicios y los expone al consumidor externo.
// Solo se permite un programa por aplicación
#[program]
impl LowFileProgram {
    // Constructor de la aplicación (función asociada)
    // Se puede llamar una vez por ciclo de vida de la aplicación
    pub fn new() -> Self {
        // Inicializa el estado;  // Inicializa el servicio LowFile
        Self
    }

    // Método que trabaja con "&self", y sin otros parámetros, se trata como un 
    // constructor de servicio expuesto, llamado cada vez que un mensaje de solicitud 
    // necesita ser despachado a un servicio seleccionado
    #[route("LowFileService")]  // Ajustado a LowFileService
    pub fn low_file_svc(&self) -> LowFileService {
        LowFileService::new(
            "".to_string(), 
            "".to_string(), 
            0, 
            "".to_string(), 
            "".to_string(), 
            "".to_string(), 
            Vec::new(),
            "".to_string(), 
            "".to_string().into(),
            "".to_string(),
            "".to_string(),
        )
    }
}
