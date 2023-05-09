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

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
