use clap::{command, Parser};
use dotenv;
use reqwest::blocking::Client;

#[derive(Parser)]
#[command(name = "wet")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    /// City to get weather from
    #[arg(short, long)]
    city: String,
}
fn main() {
    let args = Args::parse();

    dotenv::dotenv().unwrap();

    let mut map = std::collections::HashMap::new();
    map.insert(
        "key",
        std::env::var("API_TOKEN").expect("should have API_TOKEN env variable"),
    );
    map.insert("q", args.city.clone());
    map.insert("aqi", "no".to_string());

    let client = Client::new();
    let response = client
        .get("https://api.weatherapi.com/v1/current.json")
        .query(&map)
        .send()
        .expect("should get a response back")
        .json::<serde_json::Value>()
        .expect("json should exist");

    println!(
        "It currently feels like {} in {}.",
        serde_json::to_string_pretty(&response["current"]["feelslike_f"]).unwrap(),
        args.city
    );
}
