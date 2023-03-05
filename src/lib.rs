mod config;
mod error;
pub mod types;

pub use config::Config;
pub use error::Error;
use types::{Completion, CompletionParams, Model, Models};

pub type Result<T> = std::result::Result<T, Error>;

pub struct OpenAi {
    client: reqwest::Client,
    api_version: u8,
    api_endpoint: String,
    api_auth_header: String,
}

impl OpenAi {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(OpenAi {
            client: config.client.clone(),
            api_version: config.api_version,
            api_endpoint: config.api_endpoint.to_owned(),
            api_auth_header: config
                .api_key
                .map(|s| format!("Bearer {}", s))
                .ok_or_else(|| Error::ApiKeyRequired)?,
        })
    }
}

impl OpenAi {
    pub async fn list_models(&self) -> Result<Models> {
        let url = format!("{}/v{}/models", &self.api_endpoint, &self.api_version);
        let res = self
            .client
            .get(url)
            .header("Authorization", &self.api_auth_header)
            .send()
            .await?;
        Ok(serde_json::from_str(&res.text().await?).unwrap())
    }

    pub async fn get_model(&self, model: &str) -> Result<Model> {
        let url = format!(
            "{}/v{}/models/{}",
            &self.api_endpoint, &self.api_version, model
        );
        let res = self
            .client
            .get(url)
            .header("Authorization", &self.api_auth_header)
            .send()
            .await?;
        Ok(serde_json::from_str(&res.text().await?).unwrap())
    }

    pub async fn create_completion(&self, params: CompletionParams) -> Result<Completion> {
        let url = format!("{}/v{}/completions", &self.api_endpoint, &self.api_version);
        let res = self
            .client
            .post(url)
            .header("Authorization", &self.api_auth_header)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).unwrap())
            .send()
            .await?;
        Ok(serde_json::from_str(&res.text().await?).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn generic_test() -> Result<()> {
        let api_key = std::env::var("OPENAIKEY").unwrap();
        let config = Config {
            api_key: Some(&api_key),
            ..Default::default()
        };
        let client = OpenAi::new(&config)?;
        // let models = client.list_models().await?;
        // models.data.iter().for_each(|s| println!("{}", s.id));

        let model = client.get_model("text-davinci-003").await?;
        dbg!(&model);

        let req = "Say this is a test";
        let resp = client
            .create_completion(CompletionParams::with_prompt(&model.id, req))
            .await?;
        println!("U: {}\nR: {}", req, &resp.choices.first().unwrap().text);

        Ok(())
    }
}
