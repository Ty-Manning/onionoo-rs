//! Client Customization

use onionoo::{
    Client, QueryParameters, endpoints,
    selection::{BoolValue, TypeValue},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Client Customization Example");

    // Test 1: Default client
    let default_client = Client::new();
    println!(
        "Default client created - Base URL: {}",
        default_client.base_url()
    );

    // Test 2: Custom base URL
    let custom_url = "https://onionoo.torproject.org";
    let custom_client = Client::new_with_base_url(custom_url);
    println!(
        "Custom client created - Base URL: {}",
        custom_client.base_url()
    );

    // Test 3: Multiple clients with different configurations
    let clients = vec![
        ("Default", Client::new()),
        (
            "Custom Tor Project",
            Client::new_with_base_url("https://onionoo.torproject.org"),
        ),
        (
            "Custom Test",
            Client::new_with_base_url("https://test.onionoo.torproject.org"),
        ),
    ];

    for (name, client) in clients {
        let params = QueryParameters::new().limit(1);
        match endpoints::summary(&client, params).await {
            Ok(response) => {
                println!(
                    "{} client connectivity test passed - API v{}",
                    name, response.version
                );
            }
            Err(e) => {
                println!("{} client connectivity test failed: {}", name, e);
            }
        }
    }

    // Test 4: Endpoint enumeration
    let relay_only_endpoints = onionoo::endpoints::helpers::relay_only_endpoints();
    println!(
        "Relay-only endpoints: {} available",
        relay_only_endpoints.len()
    );

    let relay_bridge_endpoints = onionoo::endpoints::helpers::relay_bridge_endpoints();
    println!(
        "Relay+Bridge endpoints: {} available",
        relay_bridge_endpoints.len()
    );

    // Test 5: Client reuse and efficiency
    let shared_client = Client::new();

    // Test multiple endpoints with shared client
    if let Ok(response) = endpoints::summary(&shared_client, QueryParameters::new().limit(1)).await
    {
        println!("Summary endpoint: {} relays", response.relays.len());
    }

    if let Ok(response) = endpoints::details(&shared_client, QueryParameters::new().limit(1)).await
    {
        println!("Details endpoint: {} relays", response.relays.len());
    }

    if let Ok(response) =
        endpoints::bandwidth(&shared_client, QueryParameters::new().limit(1)).await
    {
        println!("Bandwidth endpoint: {} relays", response.relays.len());
    }

    // Test 6: Parameter object reuse
    let base_params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::True)
        .limit(2);

    let test_cases = vec![(
        "Summary with base params",
        endpoints::summary(&shared_client, base_params.clone()),
    )];

    for (name, request) in test_cases {
        match request.await {
            Ok(response) => {
                println!("{}: Success - Found {} relays", name, response.relays.len());
            }
            Err(e) => {
                println!("{}: Error - {}", name, e);
            }
        }
    }

    if let Ok(response) = endpoints::details(&shared_client, base_params.clone()).await {
        println!(
            "Details with base params: Success - Found {} relays",
            response.relays.len()
        );
    }

    if let Ok(response) = endpoints::uptime(&shared_client, base_params.clone()).await {
        println!(
            "Uptime with base params: Success - Found {} relays",
            response.relays.len()
        );
    }

    // Test 7: Error handling with different clients
    let test_clients = vec![
        ("Valid default", Client::new()),
        (
            "Valid custom",
            Client::new_with_base_url("https://onionoo.torproject.org"),
        ),
        (
            "Invalid URL",
            Client::new_with_base_url("https://invalid.onionoo.example"),
        ),
    ];

    for (name, client) in test_clients {
        let params = QueryParameters::new().limit(1);
        match endpoints::summary(&client, params).await {
            Ok(response) => {
                println!("{}: API v{}", name, response.version);
            }
            Err(e) => {
                println!("{}: {}", name, e);
            }
        }
    }

    // Test 8: Performance comparison
    let perf_client = Client::new();

    let start_single = std::time::Instant::now();
    for i in 0..3 {
        let params = QueryParameters::new().limit(1);
        if let Ok(response) = endpoints::summary(&perf_client, params).await {
            println!("Request {}: {} relays", i + 1, response.relays.len());
        }
    }
    let single_duration = start_single.elapsed();

    println!("Single client time: {:?}", single_duration);

    Ok(())
}
