//! Clients endpoint example (bridges only)
use onionoo::{
    Client, QueryParameters, endpoints,
    selection::{BoolValue, TypeValue},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let params = QueryParameters::new()
        .type_param(TypeValue::Bridge)
        .running(BoolValue::True)
        .limit(3);

    let response = endpoints::clients(&client, params).await?;

    println!("Found {} bridges with client data:", response.bridges.len());

    if response.bridges.is_empty() {
        println!("No bridges found matching the criteria.");
        return Ok(());
    }

    for (i, bridge) in response.bridges.iter().enumerate() {
        println!("\nBridge #{}: {}", i + 1, bridge.fingerprint);

        if let Some(clients) = &bridge.average_clients {
            if let Some(one_month) = &clients.one_month {
                println!("  Client history (1 month):");
                println!("    First: {}, Last: {}", one_month.first, one_month.last);
                println!("    Points: {}", one_month.values.len());

                let valid_values: Vec<f64> = one_month.values.iter().flatten().copied().collect();
                if !valid_values.is_empty() {
                    let avg_clients: f64 =
                        valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                    println!("    Average clients: {:.2}", avg_clients);
                }
            }
        }
    }

    println!("\nResponse Metadata:");
    println!("  API Version: {}", response.version);
    println!("  Bridges Published: {}", response.bridges_published);

    Ok(())
}
