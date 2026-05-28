pub trait ModelConfig: Send + Sync + Clone {
    fn name(&self) -> &str { "Unnamed Config" }
}
