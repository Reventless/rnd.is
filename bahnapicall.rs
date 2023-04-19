use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Station {
    id: String,
    title: String,
}

#[derive(Deserialize, Debug)]
struct StationBoardEntry {
    name: String,
    category: String,
    stop_location: Station,
    scheduled_time: String,
    expected_time: String,
    status: String,
}

#[derive(Deserialize, Debug)]
struct StationBoard {
    station: Station,
    entries: Vec<StationBoardEntry>,
}

fn main() -> Result<(), request::Error> {
    let client = Client::new();
    let station_id = "8000085"; // Düsseldorf Hbf station ID
    let api_key = "YOUR_API_KEY_HERE";
    let url = format!(
        "https://api.deutschebahn.com/fahrplan-plus/v1/arrivalBoard/{station_id}?date={date}",
        station_id = station_id,
        date = chrono::Utc::now().format("%Y-%m-%d").to_string(),
    );
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .send()?
        .json::<StationBoard>()?;
    for entry in res.entries {
        println!(
            "Train {} ({}) at {} is expected to arrive at {} with a delay of {} minutes",
            entry.name,
            entry.category,
            entry.stop_location.title,
            entry.expected_time,
            entry.status.parse::<i32>().unwrap_or_default(),
        );
    }
    Ok(())
}
