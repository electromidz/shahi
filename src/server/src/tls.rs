// use quinn::rustls::{self, pki_types::pem::PemObject};
// use rustls::ServerConfig;
//
// use tokio_rustls::rustls::pki_types::{
//     CertificateDer, CertificateSigningRequestDer, PrivateKeyDer,
// };
//
// pub fn load_tls_config() -> ServerConfig {
//     //let cert: Vec<_> = CertificateDer::pem_file_iter;
//     let cert = CertificateSigningRequestDer::from_pem_file("cert.pem");
//     let key = PrivateKeyDer::from_pem_file("key.pem").unwrap();
//
//     rustls::ServerConfig::builder()
//         .with_single_cert(vec![cert], key)
//         .with_no_client_auth()
//         .unwrap()
// }

use std::{
    fs::File,
    io::{BufReader, Error as IoError},
};

use tokio_rustls::rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::ServerConfig,
};

/// Load TLS configuration (certificate + private key)
pub fn load_tls_config() -> Result<ServerConfig, IoError> {
    let certs = load_certs("cert.pem")?;
    let key = load_private_key("key.pem")?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("Failed to create TLS config");

    Ok(config)
}

/// Load certificates from a PEM file.
fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, IoError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs: Vec<CertificateDer<'static>> =
        rustls_pemfile::certs(&mut reader).collect::<Result<_, _>>()?; // Collect and handle errors properly
    Ok(certs)
}

/// Load a private key from a PEM file.
fn load_private_key(path: &str) -> Result<PrivateKeyDer<'static>, IoError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader).collect::<Result<Vec<_>, _>>()?; // Collect all keys and handle errors

    let key = keys.into_iter().next().expect("No valid private key found"); // Ensure we have at least one key

    Ok(PrivateKeyDer::from(key))
}
