//! Error handling example
use onionoo::{
    Client, QueryParameters, endpoints,
    selection::{BoolValue, TypeValue},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Test normal operation
    match fetch_relays_safely(&client, TypeValue::Relay, BoolValue::True, 3).await {
        Ok(relays) => {
            println!("Successfully fetched {} relays", relays.len());
            for relay in &relays {
                println!(
                    "  Relay: {} ({})",
                    relay.n,
                    if relay.r { "Running" } else { "Offline" }
                );
            }
        }
        Err(e) => {
            println!("Error fetching relays: {}", e);
        }
    }

    // Test empty results
    let empty_params = QueryParameters::new().type_param(TypeValue::Relay).limit(0);
    match endpoints::summary(&client, empty_params).await {
        Ok(response) => {
            if response.relays.is_empty() {
                println!("Empty result handled gracefully");
                println!(
                    "  Version: {}, Published: {}",
                    response.version, response.relays_published
                );
            }
        }
        Err(e) => {
            println!("Error with limit=0: {}", e);
        }
    }

    // Test retry logic
    let mut attempts = 0;
    let max_attempts = 3;

    while attempts < max_attempts {
        attempts += 1;
        println!("Attempt {} of {}...", attempts, max_attempts);

        match endpoints::summary(&client, QueryParameters::new().limit(1)).await {
            Ok(_) => {
                println!("Retry successful on attempt {}", attempts);
                break;
            }
            Err(e) if attempts < max_attempts => {
                println!("Attempt {} failed: {}, retrying...", attempts, e);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                println!("All retry attempts failed: {}", e);
                break;
            }
        }
    }

    Ok(())
}

async fn fetch_relays_safely(
    client: &Client,
    type_value: TypeValue,
    running: BoolValue,
    limit: u32,
) -> Result<Vec<onionoo::models::RelaySummary>, Box<dyn std::error::Error>> {
    let params = QueryParameters::new()
        .type_param(type_value)
        .running(running)
        .limit(limit);

    let response = endpoints::summary(client, params).await?;
    Ok(response.relays)
}
