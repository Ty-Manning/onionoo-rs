//! Uptime endpoint example
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

    let response = endpoints::uptime(&client, params).await?;

    println!("Found {} relays with uptime data:", response.relays.len());

    if response.relays.is_empty() {
        println!("No relays found matching the criteria.");
        return Ok(());
    }

    for (i, relay) in response.relays.iter().enumerate() {
        println!("\nRelay #{}: {}", i + 1, relay.fingerprint);

        if let Some(uptime) = &relay.uptime {
            if let Some(one_month) = &uptime.one_month {
                println!("  Uptime history (1 month):");
                println!("    First: {}, Last: {}", one_month.first, one_month.last);
                println!(
                    "    Interval: {} seconds, Points: {}",
                    one_month.interval,
                    one_month.values.len()
                );

                let valid_values: Vec<f64> = one_month.values.iter().flatten().copied().collect();
                if !valid_values.is_empty() {
                    let avg_uptime: f64 =
                        valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                    println!("    Average uptime: {:.2}%", avg_uptime * 100.0);
                }
            }
        }

        if let Some(flags) = &relay.flags {
            println!(
                "  Flag uptime history: {} different flags",
                flags.flags.len()
            );
        }
    }

    println!("\nResponse Metadata:");
    println!("  API Version: {}", response.version);
    println!("  Relays Published: {}", response.relays_published);

    Ok(())
}
