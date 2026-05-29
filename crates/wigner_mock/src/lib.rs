pub struct ClebschGordan {
    pub tj1: i32,
    pub tm1: i32,
    pub tj2: i32,
    pub tm2: i32,
    pub tj12: i32,
    pub tm12: i32,
}
impl ClebschGordan {
    pub fn value(&self) -> f64 {
        0.0 // Mock value for WASM
    }
}
