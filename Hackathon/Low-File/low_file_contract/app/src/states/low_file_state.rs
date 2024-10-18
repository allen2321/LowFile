// Importa las dependencias necesarias de sails_rs
use sails_rs::prelude::*;

// Variable estática mutable para el estado del contrato
pub static mut LOWFILE_STATE: Option<LowFileState> = None;

// Estructura para el estado de LowFile
#[derive(Clone, Default)]
pub struct LowFileState {
    pub nombre: String,
    pub edad: u32,
    pub profesion: String,
    pub titulacion: String,
    pub ubicacion: String,
    pub certificaciones: Vec<String>,
    pub did: String,
    pub password_hash: String,
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