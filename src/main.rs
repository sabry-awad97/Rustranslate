use rand::Rng;
use reqwest::Client;
use std::io::{self, Write};
use std::{fmt, process};

enum TranslationError {
    RequestFailed,
    ResponseParsingFailed,
    NoTranslationFound(String),
    MaxRetriesExceeded(String),
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationError::RequestFailed => write!(f, "Failed to send translation request"),
            TranslationError::ResponseParsingFailed => {
                write!(f, "Failed to parse response body as JSON")
            }
            TranslationError::NoTranslationFound(word) => {
                write!(f, "No translation found for: {}", word)
            }
            TranslationError::MaxRetriesExceeded(message) => {
                write!(f, "Max retries exceeded: {}", message)
            }
        }
    }
}

struct Translator {
    source_lang: String,
    target_lang: String,
    client: Client,
}

impl Translator {
    fn new(source_lang: impl Into<String>, target_lang: impl Into<String>) -> Self {
        Self {
            source_lang: source_lang.into(),
            target_lang: target_lang.into(),
            client: Client::new(),
        }
    }

    async fn translate(&self, word: &str) -> Result<String, TranslationError> {
        let api_url = "https://translate.googleapis.com/translate_a/single";
        let mut rng = rand::thread_rng();
        let mut retries = 0;

        loop {
            let response = match self
                .client
                .get(api_url)
                .query(&[
                    ("client", "gtx"),
                    ("dt", "t"),
                    ("sl", &self.source_lang),
                    ("tl", &self.target_lang),
                    ("q", word),
                ])
                .send()
                .await
            {
                Ok(response) => response,
                Err(_) => return Err(TranslationError::RequestFailed),
            };

            let text = match response.text().await {
                Ok(text) => text,
                Err(_) => return Err(TranslationError::ResponseParsingFailed),
            };

            let json = match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => json,
                Err(_) => return Err(TranslationError::ResponseParsingFailed),
            };

            if let Some(translation) = json[0][0][0].as_str() {
                return Ok(translation.to_owned());
            } else {
                let error_message = format!("No translation found for: {}", word);
                if retries < 3 {
                    let delay = rng.gen_range(0..=5) * 1000;
                    std::thread::sleep(std::time::Duration::from_millis(delay));
                    retries += 1;
                } else {
                    return Err(TranslationError::NoTranslationFound(error_message));
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let translator = Translator::new("en", "es");
}
