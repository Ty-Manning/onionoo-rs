//! Client Customization Example
//!
//! This example demonstrates how to customize the Onionoo client
//! with different base URLs, client configurations, and endpoint helpers.

use onionoo::{
    Client, QueryParameters, endpoints,
    selection::{BoolValue, TypeValue},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Client Customization Example");
    println!("===============================");

    // Test 1: Default client
    println!("\n📡 Test 1: Default client configuration...");
    let default_client = Client::new();
    println!("✅ Default client created");
    println!("   Base URL: {}", default_client.base_url());
    println!("   Is default: {}", default_client.is_default());

    // Test 2: Custom base URL
    println!("\n🔧 Test 2: Custom base URL configuration...");
    let custom_url = "https://onionoo.torproject.org";
    let custom_client = Client::new_with_base_url(custom_url);
    println!("✅ Custom client created");
    println!("   Base URL: {}", custom_client.base_url());
    println!("   Is default: {}", custom_client.is_default());

    // Test 3: Test multiple clients with different configurations
    println!("\n🏗️  Test 3: Multiple client instances...");

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
        println!("   {} client:", name);
        println!("      URL: {}", client.base_url());
        println!("      Default: {}", client.is_default());

        // Test connectivity with each client
        let params = QueryParameters::new().limit(1);
        match endpoints::summary(&client, params.clone()).await {
            Ok(response) => {
                println!("      ✅ Connectivity test passed");
                println!("      📊 API Version: {}", response.version);
            }
            Err(e) => {
                println!("      ❌ Connectivity test failed: {}", e);
            }
        }
    }

    // Test 4: Endpoint enumeration and helpers
    println!("\n🔍 Test 4: Endpoint enumeration...");

    // Test if endpoint helpers are available
    let relay_only_endpoints = onionoo::endpoints::helpers::relay_only_endpoints();
    println!(
        "📋 Relay-only endpoints: {} available",
        relay_only_endpoints.len()
    );

    let relay_bridge_endpoints = onionoo::endpoints::helpers::relay_bridge_endpoints();
    println!(
        "🌉 Relay+Bridge endpoints: {} available",
        relay_bridge_endpoints.len()
    );

    // Test 5: Client reuse and efficiency
    println!("\n♻️  Test 5: Client reuse patterns...");
    let shared_client = Client::new();

    // Reuse the same client for multiple requests
    println!("   Testing multiple endpoints with shared client:");

    // Test summary endpoint
    if let Ok(response) = endpoints::summary(&shared_client, QueryParameters::new().limit(1)).await
    {
        println!("   ✅ Summary endpoint: {} relays", response.relays.len());
    }

    // Test details endpoint
    if let Ok(response) = endpoints::details(&shared_client, QueryParameters::new().limit(1)).await
    {
        println!("   ✅ Details endpoint: {} relays", response.relays.len());
    }

    // Test bandwidth endpoint
    if let Ok(response) =
        endpoints::bandwidth(&shared_client, QueryParameters::new().limit(1)).await
    {
        println!("   ✅ Bandwidth endpoint: {} relays", response.relays.len());
    }

    // Test 6: Parameter object reuse
    println!("\n🔄 Test 6: Parameter reuse patterns...");

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
                println!("   ✅ {}: Success", name);
                println!("      Found: {} relays", response.relays.len());
            }
            Err(e) => {
                println!("   ❌ {}: Error - {}", name, e);
            }
        }
    }

    // Test other endpoints individually
    if let Ok(response) = endpoints::details(&shared_client, base_params.clone()).await {
        println!("   ✅ Details with base params: Success");
        println!("      Found: {} relays", response.relays.len());
    }

    if let Ok(response) = endpoints::uptime(&shared_client, base_params.clone()).await {
        println!("   ✅ Uptime with base params: Success");
        println!("      Found: {} relays", response.relays.len());
    }

    // Test 7: Error handling with different clients
    println!("\n🛡️  Test 7: Error handling across clients...");

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
                println!("   ✅ {}: API v{}", name, response.version);
            }
            Err(e) => {
                println!("   ❌ {}: {}", name, e);
            }
        }
    }

    // Test 8: Performance comparison
    println!("\n⚡ Test 8: Performance considerations...");

    let perf_client = Client::new();

    // Single client, multiple requests (efficient)
    let start_single = std::time::Instant::now();
    for i in 0..3 {
        let params = QueryParameters::new().limit(1);
        if let Ok(response) = endpoints::summary(&perf_client, params).await {
            println!("   Request {}: {} relays", i + 1, response.relays.len());
        }
    }
    let single_duration = start_single.elapsed();

    println!("   Single client time: {:?}", single_duration);

    println!("\n💡 Client Customization Tips:");
    println!("   • Reuse client instances for better performance");
    println!("   • Use custom base URLs for testing or mirrors");
    println!("   • Check client.is_default() to verify configuration");
    println!("   • Handle errors gracefully across different client configurations");
    println!("   • Reuse QueryParameters objects to avoid recreation overhead");
    println!("   • Consider connection pooling for high-frequency usage");

    println!("\n✨ Client customization example completed successfully!");

    Ok(())
}
