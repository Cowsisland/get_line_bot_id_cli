use clap::Parser;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct Response {
    userId: String,
    basicId: String,
    displayName: String,
    pictureUrl: String,
    chatMode: String,
    markAsReadMode: String,
}

// Parserを継承した構造体はArgの代わりに使用することが可能。
#[derive(Parser)]
#[clap(
    name = "bi",
    author = "Ussy",
    version = "v1.0.0",
    about = "To get line bot id."
)]
struct AppArg {
    // 必須のオプション
    #[clap(short = 'i', long = "id")]
    id: String,
}

fn main() {
    let arg: AppArg = AppArg::parse();
    println!("{}", arg.id);
    match get_bot_id(arg.id) {
        Ok(resp) => {
            if resp.status().is_success() {
                let resp_json: Response = resp.json().unwrap();
                println!("{}", resp_json.userId)
            } else {
                println!("status: {:?}", resp.status());
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }
}

fn get_bot_id(token: String) -> Result<reqwest::blocking::Response, Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get("https://api.line.me/v2/bot/info")
        .header("Authorization", &format!("Bearer {}", token))
        .send()?;
    Ok(resp)
}