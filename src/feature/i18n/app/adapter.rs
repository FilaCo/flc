pub struct I18n;

impl I18n {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_dictionaries(&self) {}

    pub async fn translate(&self) {}
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}
