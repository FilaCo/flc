pub struct N64(u64);

impl N64 {
    pub fn new() -> Self {
        Self(1)
    }
}

impl Default for N64 {
    fn default() -> Self {
        Self::new()
    }
}
