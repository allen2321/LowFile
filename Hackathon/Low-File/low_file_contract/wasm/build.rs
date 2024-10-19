use sails_client_gen::ClientGenerator;
use std::{env, fs, path::PathBuf};
use app::LowFileProgram;  // Aseguramos que el programa utilizado es LowFileProgram

fn main() {
    // Compilar el contrato para obtener el archivo .opt.wasm
    sails_rs::build_wasm();

    // Ruta donde se encuentra el archivo "Cargo.toml"
    let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Ruta donde se generará el cliente (en OUT_DIR)
    let outdir_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Ruta donde se creará el archivo "app.idl"
    let idl_path = cargo_toml_path.clone().join("app.idl");
    let client_path = outdir_path.clone().join("app_client.rs");

    // Generar la IDL del contrato
    sails_idl_gen::generate_idl_to_file::<LowFileProgram>(idl_path.clone())  // Cambiado a LowFileProgram
        .expect("Fallo al generar la IDL");

    // Si deseas generar el cliente del contrato, descomenta esta sección
    
    ClientGenerator::from_idl_path(&idl_path)
        .generate_to(client_path.clone())
        .expect("Fallo al generar el cliente");
    

    // Copiar el cliente al directorio actual donde se encuentra "Cargo.toml"
     // Then, copies the client that is in the OUT_DIR path in the current directory (wasm), where the 
    //"Cargo.toml" file is located 
        fs::copy(client_path, cargo_toml_path.join("app_client.rs"))
            .unwrap();
}
