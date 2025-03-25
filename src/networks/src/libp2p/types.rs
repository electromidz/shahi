use std::error::Error;

pub trait TP2PNetwork {
    async fn listen(&mut self, addr: &str) -> Result<(), Box<dyn Error>>;
    async fn dial(&mut self, addr: &str) -> Result<(), Box<dyn Error>>;
}
