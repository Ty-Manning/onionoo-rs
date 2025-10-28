//! Query Parameters

use onionoo::{
    Client, QueryParameters, endpoints,
    selection::{BoolValue, TypeValue},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Onionoo Client Query Parameters");

    let client = Client::new();

    // Test 1: Basic parameter usage
    let basic_params = QueryParameters::new().type_param(TypeValue::Relay).limit(3);

    let response = endpoints::summary(&client, basic_params).await?;
    println!(
        "Found {} relays with basic parameters",
        response.relays.len()
    );

    // Test 2: Running status filtering
    // Only running relays
    let running_params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::True)
        .limit(2);

    let running_response = endpoints::summary(&client, running_params).await?;
    println!("Found {} running relays", running_response.relays.len());

    // Only non-running relays
    let non_running_params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::False)
        .limit(2);

    let non_running_response = endpoints::summary(&client, non_running_params).await?;
    println!(
        "Found {} non-running relays",
        non_running_response.relays.len()
    );

    // Test 3: Bridge-specific parameters
    let bridge_params = QueryParameters::new()
        .type_param(TypeValue::Bridge)
        .running(BoolValue::True)
        .limit(2);

    let bridge_response = endpoints::summary(&client, bridge_params).await?;
    println!("Found {} running bridges", bridge_response.bridges.len());

    // Test 4: Parameter combinations
    let combined_params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::True)
        .limit(1);

    let combined_response = endpoints::details(&client, combined_params).await?;
    println!(
        "Found {} relays with combined parameters",
        combined_response.relays.len()
    );

    if !combined_response.relays.is_empty() {
        let relay = &combined_response.relays[0];
        println!("Sample relay: {}", relay.nickname);
        println!("Fingerprint: {}", relay.fingerprint);
        println!("Running: {}", relay.running);
        println!(
            "Country: {:?}",
            relay.country_name.as_deref().unwrap_or("Unknown")
        );

        if let Some(flags) = &relay.flags {
            println!("Flags: {}", flags.join(", "));
        }
    }

    // Test 5: Different endpoints with same parameters
    let test_params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::True)
        .limit(1);

    // Summary endpoint
    let summary_resp = endpoints::summary(&client, test_params.clone()).await?;
    println!("Summary endpoint: {} relays", summary_resp.relays.len());

    // Details endpoint
    let details_resp = endpoints::details(&client, test_params.clone()).await?;
    println!("Details endpoint: {} relays", details_resp.relays.len());

    // Bandwidth endpoint
    let bandwidth_resp = endpoints::bandwidth(&client, test_params.clone()).await?;
    println!("Bandwidth endpoint: {} relays", bandwidth_resp.relays.len());

    // Uptime endpoint
    let uptime_resp = endpoints::uptime(&client, test_params.clone()).await?;
    println!("Uptime endpoint: {} relays", uptime_resp.relays.len());

    // Weights endpoint (relays only)
    let weights_resp = endpoints::weights(&client, test_params.clone()).await?;
    println!("Weights endpoint: {} relays", weights_resp.relays.len());

    // Test 6: Edge cases and limits
    // Very small limit
    let small_limit_params = QueryParameters::new().type_param(TypeValue::Relay).limit(1);

    let small_response = endpoints::summary(&client, small_limit_params).await?;
    println!("Small limit (1): {} relays", small_response.relays.len());

    // Zero limit (should return empty or minimal results)
    let zero_limit_params = QueryParameters::new().type_param(TypeValue::Relay).limit(0);

    let zero_response = endpoints::summary(&client, zero_limit_params).await?;
    println!("Zero limit: {} relays", zero_response.relays.len());

    // Test 7: Metadata handling with different parameters
    for (name, params) in [
        (
            "Relay running",
            QueryParameters::new()
                .type_param(TypeValue::Relay)
                .running(BoolValue::True)
                .limit(1),
        ),
        (
            "Bridge running",
            QueryParameters::new()
                .type_param(TypeValue::Bridge)
                .running(BoolValue::True)
                .limit(1),
        ),
    ] {
        match endpoints::summary(&client, params).await {
            Ok(resp) => {
                println!(
                    "{}: API v{}, {} relays published",
                    name, resp.version, resp.relays_published
                );

                if let Some(skipped) = resp.relays_skipped {
                    println!("Skipped: {}", skipped);
                }

                if let Some(truncated) = resp.relays_truncated {
                    println!("Truncated: {}", truncated);
                }
            }
            Err(e) => {
                println!("{}: Error - {}", name, e);
            }
        }
    }

    Ok(())
}
