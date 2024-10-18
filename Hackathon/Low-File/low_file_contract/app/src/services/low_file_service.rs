// Importamos las dependencias necesarias de sails_rs
use sails_rs::prelude::*;
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use sha2::{Sha256, Digest};
use rsa::pkcs1::ToRsaPublicKey;
use base64::{encode, decode};
use rand::rngs::OsRng;

// Variable estática mutable para el estado del contrato
pub static mut LOWFILE_STATE: Option<LowFileService> = None;

// Estructura para el servicio LowFile con soporte DID y hash
#[derive(Clone, Default)]
pub struct LowFileService {
    pub nombre: String,
    pub edad: u32,
    pub profesion: String,
    pub titulacion: String,
    pub ubicacion: String,
    pub certificaciones: Vec<String>,
    pub did: String,
    pub password_hash: String,
}

// Implementación del servicio LowFile con soporte DID y RSA hash
#[service]
impl LowFileService {
    /// Constructor del servicio LowFile con DID y hash.
    pub fn new(
        nombre: String, edad: u32, profesion: String, titulacion: String, 
        ubicacion: String, certificaciones: Vec<String>, password: String
    ) -> Self {
        // Generamos el DID único y el hash de la contraseña
        let (did, password_hash) = Self::generate_did_and_hash(&password);
        Self {
            nombre,
            edad,
            profesion,
            titulacion,
            ubicacion,
            certificaciones,
            did,
            password_hash,
        }
    }

    /// Método para generar el DID y el hash de la contraseña usando RSA.
    fn generate_did_and_hash(password: &str) -> (String, String) {
        // Generar par de claves RSA
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        // Crear el DID a partir de la clave pública
        let did = format!("did:example:{}", encode(public_key.to_pkcs1_der().unwrap()));

        // Generar el hash de la contraseña
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        let password_hash = encode(result);

        (did, password_hash)
    }

    /// Método para verificar el hash de la contraseña.
    pub fn verify_password(&self, password: String) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        let password_hash = encode(result);
        self.password_hash == password_hash
    }

    /// Llamada remota para establecer los datos del usuario, incluyendo DID y hash.
    pub fn set_user_data(
        &mut self,
        nombre: String, edad: u32, profesion: String, titulacion: String, 
        ubicacion: String, certificaciones: Vec<String>, password: String
    ) {
        unsafe {
            if let Some(ref mut state) = LOWFILE_STATE {
                state.nombre = nombre;
                state.edad = edad;
                state.profesion = profesion;
                state.titulacion = titulacion;
                state.ubicacion = ubicacion;
                state.certificaciones = certificaciones;
                
                // Generar un nuevo DID y actualizar el hash de la contraseña
                let (new_did, new_password_hash) = Self::generate_did_and_hash(&password);
                state.did = new_did;
                state.password_hash = new_password_hash;
            }
        }
    }

    /// Llamada remota para recuperar los datos del usuario.
    pub fn get_user_data(&self) -> IoLowFileState {
        unsafe {
            LOWFILE_STATE.as_ref().map_or(IoLowFileState::default(), |state| state.clone().into())
        }
    }
}

// Estructura para enviar datos a los usuarios, incluyendo DID.
#[derive(Encode, Decode, TypeInfo, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoLowFileState {
    pub nombre: String,
    pub edad: u32,
    pub profesion: String,
    pub titulacion: String,
    pub ubicacion: String,
    pub certificaciones: Vec<String>,
    pub did: String, // Nuevo campo para DID
}

// Implementación del rasgo From para convertir LowFileService a IoLowFileState
impl From<LowFileService> for IoLowFileState {
    fn from(value: LowFileService) -> Self {
        Self {
            nombre: value.nombre,
            edad: value.edad,
            profesion: value.profesion,
            titulacion: value.titulacion,
            ubicacion: value.ubicacion,
            certificaciones: value.certificaciones,
            did: value.did,
        }
    }
}