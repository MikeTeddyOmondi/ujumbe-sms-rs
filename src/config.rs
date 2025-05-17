#[derive(Debug, Clone)]
pub struct UjumbeSmsConfig {
    pub api_key: String,
    pub email: String,
    pub base_url: String,
}

impl UjumbeSmsConfig {
    pub fn new(api_key: String, email: String) -> Self {
        UjumbeSmsConfig {
            api_key,
            email,
            base_url: "https://ujumbesms.co.ke".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}
