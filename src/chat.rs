use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{types::Usage, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chat {
    pub model: String,
    #[serde(skip)]
    client: reqwest::Client,
    #[serde(skip)]
    api_endpoint: String,
    #[serde(skip)]
    api_version: u8,
    #[serde(skip)]
    api_auth_header: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<ChatStop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    // logit_bias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatStop {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Message {
            role: role.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<ChatResponseChoice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponseChoice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

impl Deref for Chat {
    type Target = Vec<Message>;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl DerefMut for Chat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.messages
    }
}

impl Chat {
    pub(crate) fn new(
        model: &str,
        client: &reqwest::Client,
        api_version: u8,
        api_endpoint: &str,
        api_auth_header: &str,
    ) -> Self {
        Self {
            model: model.into(),
            client: client.clone(),
            api_version,
            api_endpoint: api_endpoint.into(),
            api_auth_header: api_auth_header.into(),
            ..Default::default()
        }
    }

    pub fn instructions<T>(&mut self, message: &[T])
    where
        T: AsRef<str>,
    {
        let initial_count = self.messages.len();
        let mut system_flag = false;
        let mut turn = false;
        if initial_count == 0 {
            system_flag = true;
        } else {
            let adjusted_count = initial_count - 1;
            turn = adjusted_count % 2 == 0;
        }

        let mut iter = message.iter();
        while let Some(m) = iter.next() {
            if system_flag {
                self.messages.push(Message::new("system", m.as_ref()));
                system_flag = false;
            } else {
                if !turn {
                    self.messages.push(Message::new("user", m.as_ref()));
                } else {
                    self.messages.push(Message::new("assistant", m.as_ref()));
                }
                turn = !turn;
            }
        }
    }

    pub async fn send(&mut self, message: &str) -> Result<String> {
        self.messages.push(Message {
            role: "user".into(),
            content: message.into(),
        });
        let url = format!(
            "{}/v{}/chat/completions",
            &self.api_endpoint, &self.api_version
        );
        let resp = self
            .client
            .post(url)
            .header("Authorization", &self.api_auth_header)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&self).unwrap())
            .send()
            .await?;
        let text: ChatResponse = serde_json::from_str(&resp.text().await?).unwrap();
        let choice = text.choices.first().unwrap();
        self.messages.push(choice.message.clone());
        Ok(choice.message.content.trim().into())
    }
}
