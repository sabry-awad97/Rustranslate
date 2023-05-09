use anyhow::{Context, Result};
use rand::Rng;
use reqwest::Client;
use std::io::{self, Write};
use std::process;

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
