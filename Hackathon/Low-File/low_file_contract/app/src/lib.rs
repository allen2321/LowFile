#![no_std]

// Importamos los módulos necesarios de Sails-RS
use sails_rs::prelude::*;
use crate::states::low_file_state::LowFileState;
use gstd::collections::HashMap; 
// Importamos los módulos internos
pub mod states;
pub mod services;

// Importamos el servicio para utilizarlo en el programa
use services::low_file_service::LowFileService;

// Traffic light program struct para construir el programa (sólo puede haber uno por contrato)
pub struct TrafficLightProgram;

// Implementación del programa principal que expone los servicios
#[program]
impl TrafficLightProgram {
    // Constructor de la aplicación (se llama una vez por vida de la aplicación)
    pub fn new() -> Self {
        // Inicializamos el estado si es necesario
        LowFileState::init_state ();

        Self
    }

    // Método para exponer el servicio TrafficLight con soporte para DID y autenticación
    #[route("TrafficLight")]
    pub fn traffic_light_svc(&self) -> LowFileService {
        LowFileService::new(
            "".to_string(), 0, "".to_string(), 
            "".to_string(), "".to_string(), Vec::new(), 
            "".to_string(),
        )
    }
}