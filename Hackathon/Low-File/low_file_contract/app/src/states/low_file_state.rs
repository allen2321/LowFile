// Importa las dependencias necesarias de sails_rs
use sails_rs::prelude::*;
   

// Variable estática mutable para el estado del contrato
pub static mut LOWFILE_STATE: Option<LowFileState> = None;

// Estructura para el estado de LowFile
#[derive(Clone, Default)]
pub struct LowFileState {
    pub nombre: String, // Nombre del usuario
    pub nickname: String,
    pub email: String,
    pub edad: u32,      // Edad del usuario
    pub profesion: String, // Profesión del usuario
    pub titulacion: String, // Titulación del usuario
    pub ubicacion: String, // Ubicación del usuario
    pub certificaciones: Vec<String>, // Lista de certificaciones del usuario
    pub identi: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

// Implementación de métodos para LowFileState

impl LowFileState {
    /// Método para inicializar el estado global de LowFile.
    pub fn init_state() {
        unsafe {
            LOWFILE_STATE = Some(Self::default());
        };
    }

    /// Método para obtener una referencia mutable al estado.
    pub fn state_mut() -> &'static mut LowFileState {
        let state = unsafe { LOWFILE_STATE.as_mut() };
        debug_assert!(state.is_some(), "El estado no está inicializado");
        unsafe { state.unwrap_unchecked() }
    }

    /// Método para obtener una referencia inmutable al estado.
    pub fn state_ref() -> &'static LowFileState {
        let state = unsafe { LOWFILE_STATE.as_ref() };
        debug_assert!(state.is_some(), "El estado no está inicializado");
        unsafe { state.unwrap_unchecked() }
    }
}