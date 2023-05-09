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
}

#[tokio::main]
async fn main() {
    let translator = Translator::new("en", "es");
}
