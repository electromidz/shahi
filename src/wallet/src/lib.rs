use secp256k1::hashes::{sha256, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

#[derive(Debug)]
pub struct Wallet {
    name: Option<String>,
    public_key: PublicKey,
    address: String,
}

impl Wallet {
    /// Create a new wallet with an optional name.
    pub fn new(name: Option<String>) -> Self {
        let secp = Secp256k1::new(); // Consider making this static
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        let address = derive_address(&public_key);

        Wallet {
            name,
            public_key,
            address,
        }
    }

    /// Get the public key.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Get the wallet address.
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get the wallet name.
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

/// Derives a simple SHA-256 address from a public key.
/// In a real blockchain, this should follow a standard format like Base58 or Bech32.
fn derive_address(public_key: &PublicKey) -> String {
    let public_key_bytes = public_key.serialize();
    let hash = sha256::Hash::hash(&public_key_bytes);
    format!("0x{}", hex::encode(hash)) // Better formatting
}
