pub struct DyadicRational {
    pub m: i64,
    pub r: u32,
}

impl DyadicRational {
    #[verified_engine::verified]
    pub fn new(m: i64, r: u32) -> Self {
        Self { m, r }
    }
}
