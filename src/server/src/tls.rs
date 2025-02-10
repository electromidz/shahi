use quinn::rustls::{self, pki_types::pem::PemObject};
use rustls::ServerConfig;

use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub fn load_tls_config() -> ServerConfig {
    let cert: Vec<_> = CertificateDer::pem_file_iter("cert.pem").unwrap().collect();
    let key = PrivateKeyDer::from_pem_file("key.pem").unwrap();

    rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, key)
        .unwrap()
}
