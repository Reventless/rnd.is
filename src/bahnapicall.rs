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

async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let station_id = "8000085"; // Düsseldorf Hbf station ID
    let api_key = "";
    let url = format!(
        "https://api.deutschebahn.com/fahrplan-plus/v1/arrivalBoard/{station_id}?date={date}",
        station_id = station_id,
        date = chrono::Utc::now().format("%Y-%m-%d").to_string(),
    );
    let mut map = HeaderMap::new();

    assert_eq!(0, map.len());

    map.insert(ACCEPT, "text/plain".parse().unwrap());
    map.insert(HOST, "localhost".parse().unwrap());

    assert_eq!(2, map.len());

    map.append(ACCEPT, "text/html".parse().unwrap());

    assert_eq!(3, map.len());
    let res = reqwest::get(url)
        .await?
        .headers("Authorization", format!("Bearer {}", api_key))
        .headers("Accept", "application/json")
        .json::<StationBoard>()?
        .await?;
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
