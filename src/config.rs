#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub api_key: Option<&'a str>,
    pub api_endpoint: &'a str,
    pub api_version: u8,
    pub client: reqwest::Client,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            api_key: None,
            api_endpoint: "https://api.openai.com",
            api_version: 1,
            client: reqwest::Client::builder().https_only(true).build().unwrap(),
        }
    }
}
