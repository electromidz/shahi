use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub fn load_certificate_chain(path: &str) -> Result<Vec<CertificateDer>, Box<dyn Error>> {
    //regenerate key with this
    //openssl req -x509 -newkey rsa:2048 -keyout server.key -out server.crt -days 365 -nodes -subj "/CN=localhost"
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Parse the PEM file and extract certificates
    let certs: Vec<CertificateDer> = certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(CertificateDer::from)
        .collect();

    // Check if any certificates were found
    if certs.is_empty() {
        return Err("No certificates found in the file".into());
    }

    Ok(certs)
}
pub fn load_private_key(path: &str) -> Result<PrivateKeyDer, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    // Parse the PEM file and extract PKCS#8 private keys
    let keys: Vec<PrivatePkcs8KeyDer> =
        pkcs8_private_keys(&mut reader).collect::<Result<Vec<_>, _>>()?;

    // Check if any keys were found
    if keys.is_empty() {
        return Err("No PKCS#8 private keys found in the file".into());
    }

    // Use the first key (you can modify this logic if multiple keys are expected)
    let key = keys.into_iter().next().unwrap(); // Safe to unwrap because we checked `is_empty`
    Ok(PrivateKeyDer::Pkcs8(key))
}
