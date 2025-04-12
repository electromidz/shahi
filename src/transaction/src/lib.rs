use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,          // Sender's public key (address)
    pub receiver: String,        // Receiver's public key (address)
    pub amount: Option<u64>,     // Amount being transferred
    pub payload: Option<String>, // Use for messaging NFT, etc
    //pub signature: Vec<u8>,      // Digital signature
}

impl Transaction {
    // Create a new transaction
    pub fn new(
        sender: String,
        receiver: String,
        amount: Option<u64>,
        payload: Option<String>,
        //secret_key: &SecretKey,
    ) -> Self {
        //let message = Self::create_message(&sender, &receiver, amount);
        //let signature = Self::sign_transaction(&message, secret_key);
        Transaction {
            sender,
            receiver,
            amount,
            payload,
            //signature,
        }
    }

    // Create a message to sign (sender + receiver + amount)
    // #[warn(dead_code)]
    // fn create_message(sender: &str, receiver: &str, amount: Option<u64>) -> Message {
    //     let data = format!("{}{}{:?}", sender, receiver, amount);
    //     let hash = sha256::Hash::hash(data.as_bytes());
    //     Message::from_digest(hash.to_byte_array())
    // }
    //
    // // Sign the transaction
    //
    // #[warn(dead_code)]
    // fn sign_transaction(message: &Message, secret_key: &SecretKey) -> Vec<u8> {
    //     let secp = Secp256k1::new();
    //     let signature = secp.sign_ecdsa(message, secret_key);
    //     signature.serialize_der().to_vec()
    // }

    // Verify the transaction's signature
    // pub fn verify_signature(&self) -> bool {
    //     let secp = Secp256k1::new();
    //     let message = Self::create_message(&self.sender, &self.receiver, self.amount);
    //     let signature = match Signature::from_der(&self.signature) {
    //         Ok(sig) => sig,
    //         Err(_) => return false,
    //     };
    //     let public_key = match PublicKey::from_str(&self.sender) {
    //         Ok(pubkey) => pubkey,
    //         Err(_) => return false,
    //     };
    //     secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::rand::rngs::OsRng;

    #[test]
    fn test_transaction_creation_and_verification() {
        // Generate a random secret key and public key
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        let sender = public_key.to_string();
        let receiver = "receiver_address".to_string();
        let payload =
            Some("{\"type\": \"NFT\", \"id\": \"123\", \"name\": \"Rare Dragon\"}".to_string());

        // Create a new transaction
        let amount = Some(100);

        let transaction = Transaction::new(
            sender.clone(),
            receiver.clone(),
            amount,
            payload,
            &secret_key,
        );

        // Verify the transaction's signature
        assert!(transaction.verify_signature());

        // Tamper with the transaction and verify it fails
        let mut tampered_transaction = transaction.clone();
        tampered_transaction.amount = Some(200); // Change the amount
        assert!(!tampered_transaction.verify_signature()); // Should fail
    }
}