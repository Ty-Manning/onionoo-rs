//! Geographic analysis example
use onionoo::{
    Client, QueryParameters, endpoints,
    selection::{BoolValue, TypeValue},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::True)
        .limit(10);

    let response = endpoints::details(&client, params).await?;

    println!(
        "Analyzing {} relays for geographic distribution",
        response.relays.len()
    );

    if response.relays.is_empty() {
        println!("No relays found for geographic analysis.");
        return Ok(());
    }

    let mut country_stats = std::collections::HashMap::new();
    let mut as_stats = std::collections::HashMap::new();
    let mut location_sample = Vec::new();

    for relay in &response.relays {
        if let Some(country_code) = &relay.country {
            *country_stats.entry(country_code.clone()).or_insert(0) += 1;
        }

        if let Some(as_number) = &relay.as_number {
            *as_stats.entry(as_number.clone()).or_insert(0) += 1;
        }

        if let (Some(lat), Some(lon)) = (relay.latitude, relay.longitude) {
            let country = relay
                .country_name
                .as_deref()
                .unwrap_or("Unknown")
                .to_string();
            let city_name = relay.city_name.as_deref().unwrap_or("Unknown").to_string();
            location_sample.push((country, lat, lon, city_name));
        }
    }

    println!("\nTop Countries by Relay Count:");
    let mut country_vec: Vec<(&String, &u32)> = country_stats.iter().collect();
    country_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (country, count) in country_vec.iter().take(10) {
        let percentage = (**count as f64 / response.relays.len() as f64) * 100.0;
        println!("  {}: {} relays ({:.1}%)", country, count, percentage);
    }

    println!("\nTop Autonomous Systems:");
    let mut as_vec: Vec<(&String, &u32)> = as_stats.iter().collect();
    as_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (as_number, count) in as_vec.iter().take(5) {
        let percentage = (**count as f64 / response.relays.len() as f64) * 100.0;
        println!("  AS{}: {} relays ({:.1}%)", as_number, count, percentage);
    }

    println!("\nSample Coordinates:");
    for (i, (country, lat, lon, city)) in location_sample.iter().take(5).enumerate() {
        println!("  {}: {}, {} ({}, {})", i + 1, country, city, lat, lon);
    }

    Ok(())
}
