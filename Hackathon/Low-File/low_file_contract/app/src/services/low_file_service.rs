// Importa las dependencias necesarias de sails_rs
use sails_rs::{
    prelude::*,
    gstd::msg
};

// Variable estática mutable para el estado del contrato
pub static mut LOWFILE_STATE: Option<LowFileService> = None;

// Estructura para el estado de LowFile
#[derive(Clone, Default)]
pub struct LowFileService {
    pub nombre: String, // Nombre del usuario
    pub edad: u32,      // Edad del usuario
    pub profesion: String, // Profesión del usuario
    pub titulacion: String, // Titulación del usuario
    pub ubicacion: String, // Ubicación del usuario
    pub certificaciones: Vec<String>, // Lista de certificaciones del usuario
    pub identi: String, // Añade un identificador
    pub public_key: Vec<u128>, // Añade un vector
    pub nickname: String,
    pub email: String,
    pub private_key: Vec<u128>
}

// Implementación del servicio LowFile
#[service]
impl LowFileService {
    // Constructor del servicio LowFile.
    pub fn new(nombre: String, edad: u32, profesion: String, titulacion: String, ubicacion: String, certificaciones: Vec<String>, identi: String, public_key: Vec<u128>, private_key: Vec<u128>) -> Self {
        Self {
            nombre,
            edad,
            profesion,
            titulacion,
            ubicacion,
            certificaciones,
            identi,
            public_key,
            nickname: String::new(),
            email: String::new(),
            private_key,
        }
    }

    // Método para inicializar el estado global de LowFile.
    pub fn init_state() {
        unsafe {
            LOWFILE_STATE = Some(LowFileService::default());
        };
    }

    // Llamada remota para establecer los datos del usuario.
    pub fn set_user_data(
        &mut self,
        nombre: String,
        edad: u32,
        profesion: String,
        titulacion: String,
        ubicacion: String,
        certificaciones: Vec<String>,
        identi: String,
        public_key: Vec<u128>,
        email: String,
        nickname: String,
        private_key: Vec<u128>
    ) ->String{
        let state = unsafe{LOWFILE_STATE.as_mut()};
        if state.is_none() {
            unsafe{LOWFILE_STATE = Some(LowFileService{
                nombre: nombre.clone(), // Nombre del usuario
                edad: edad.clone(),      // Edad del usuario
                profesion: profesion.clone(), // Profesión del usuario
                titulacion: titulacion.clone(), // Titulación del usuario
                ubicacion: ubicacion.clone(), // Ubicación del usuario
                certificaciones: certificaciones.clone(), // Lista de certificaciones del usuario
                identi: identi.clone(), // Añade un identificador
                public_key: public_key.clone(), // Añade un vector
                nickname: nickname.clone(),
                email: email.clone(),
                private_key: private_key.clone()

            })}
        }
        
        let state = unsafe{LOWFILE_STATE.as_mut().unwrap()};
                state.nombre = nombre;
                state.edad = edad;
                state.profesion = profesion;
                state.titulacion = titulacion;
                state.ubicacion = ubicacion;
                state.certificaciones = certificaciones;
                state.identi = identi;
                state.public_key = public_key;
                state.email = email;
                state.nickname = nickname;
                state.private_key = private_key;
        /*unsafe {
            if let Some(ref mut state) = LOWFILE_STATE {
                // Actualiza el estado con los datos del usuario proporcionados
                state.nombre = nombre;
                state.edad = edad;
                state.profesion = profesion;
                state.titulacion = titulacion;
                state.ubicacion = ubicacion;
                state.certificaciones = certificaciones;
                state.identi = identi;
                state.public_key = public_key;
                state.email = email;
                state.nickname = nickname;
            }
        }*/
        String::from("STATE CHANGED")
    }

    // Llamada remota para recuperar los datos del usuario.
    pub fn get_user_data(&self) -> IoLowFileState {
        unsafe {
            LOWFILE_STATE.as_ref().map_or(IoLowFileState::default(), |state| state.clone().into())
        }
    }
}

// Estructura para enviar datos a los usuarios
#[derive(Encode, Decode, TypeInfo, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoLowFileState {
    pub nombre: String, // Nombre del usuario
    pub edad: u32,      // Edad del usuario
    pub profesion: String, // Profesión del usuario
    pub titulacion: String, // Titulación del usuario
    pub ubicacion: String, // Ubicación del usuario
    pub certificaciones: Vec<String>, // Lista de certificaciones del usuario
    pub identi: String,
    pub public_key: Vec<u128>,
    pub private_key: Vec<u128>,
    pub email: String,
    pub nickname: String,
}

// Implementación del rasgo From para convertir LowFileState a IoLowFileState
impl From<LowFileService> for IoLowFileState {
    fn from(value: LowFileService) -> Self {
        Self {
            nombre: value.nombre,
            edad: value.edad,
            profesion: value.profesion,
            titulacion: value.titulacion,
            ubicacion: value.ubicacion,
            certificaciones: value.certificaciones,
            identi: value.identi,
            public_key: value.public_key,
            email: value.email,
            nickname: value.nickname,
            private_key: value.private_key
        }
    }
}