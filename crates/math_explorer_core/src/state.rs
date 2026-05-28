pub enum StateData {
    TimeSeries { time: f64, values: Vec<f64> },
    Discrete(Vec<i64>),
}

pub trait ModelState: Send + Sync {
    fn extract_data(&self) -> StateData;
}
