//! Bandwidth endpoint example
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
        .limit(3);

    let response = endpoints::bandwidth(&client, params).await?;

    println!(
        "Found {} relays with bandwidth data:",
        response.relays.len()
    );

    if response.relays.is_empty() {
        println!("No relays found matching the criteria.");
        return Ok(());
    }

    for (i, relay) in response.relays.iter().enumerate() {
        println!("\nRelay #{}: {}", i + 1, relay.fingerprint);

        if let Some(write_history) = &relay.write_history {
            if let Some(one_month) = &write_history.one_month {
                println!("  Write history (1 month):");
                println!("    First: {}, Last: {}", one_month.first, one_month.last);
                println!(
                    "    Interval: {} seconds, Points: {}",
                    one_month.interval,
                    one_month.values.len()
                );

                if !one_month.values.is_empty() {
                    println!(
                        "    Sample values: {:?}",
                        &one_month.values[..std::cmp::min(3, one_month.values.len())]
                    );
                }
            }
        }

        if let Some(overload) = &relay.overload_ratelimits {
            if let Some(timestamp) = overload.timestamp {
                println!("  Overload detected at: {}", timestamp);
            }
        }
    }

    println!("\nResponse Metadata:");
    println!("  API Version: {}", response.version);
    println!("  Relays Published: {}", response.relays_published);

    Ok(())
}
