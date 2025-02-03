pub trait TP2PNetwork {
    async fn listen(&mut self, addr: &str) -> Resul<(), Box<dyn Error>>;
    async fn dial(&mut self, addr: &str) -> Resul<(), Box<dyn Error>>;
    async fn run(self);
}
