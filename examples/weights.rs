//! Weights endpoint example (relays only)
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

    let response = endpoints::weights(&client, params).await?;

    println!("Found {} relays with weight data:", response.relays.len());

    if response.relays.is_empty() {
        println!("No relays found matching the criteria.");
        return Ok(());
    }

    for (i, relay) in response.relays.iter().enumerate() {
        println!("\nRelay #{}: {}", i + 1, relay.fingerprint);

        if let Some(consensus_weight) = &relay.consensus_weight {
            if let Some(one_month) = &consensus_weight.one_month {
                println!("  Consensus weight (1 month):");
                println!("    First: {}, Last: {}", one_month.first, one_month.last);
                println!("    Points: {}", one_month.values.len());

                let valid_values: Vec<f64> = one_month.values.iter().flatten().copied().collect();
                if !valid_values.is_empty() {
                    let avg_weight: f64 =
                        valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                    println!("    Average consensus weight: {}", avg_weight as u64);
                }
            }
        }

        let mut positions = Vec::new();

        if let Some(guard) = &relay.guard_probability {
            if let Some(one_month) = &guard.one_month {
                let valid_values: Vec<f64> = one_month.values.iter().flatten().copied().collect();
                if !valid_values.is_empty() {
                    let avg_prob: f64 =
                        valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                    // Apply the factor to get the actual probability value
                    let scaled_prob = avg_prob * one_month.factor;
                    positions.push(("Guard", scaled_prob));
                }
            }
        }

        if let Some(middle) = &relay.middle_probability {
            if let Some(one_month) = &middle.one_month {
                let valid_values: Vec<f64> = one_month.values.iter().flatten().copied().collect();
                if !valid_values.is_empty() {
                    let avg_prob: f64 =
                        valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                    let scaled_prob = avg_prob * one_month.factor;
                    positions.push(("Middle", scaled_prob));
                }
            }
        }

        if let Some(exit) = &relay.exit_probability {
            if let Some(one_month) = &exit.one_month {
                let valid_values: Vec<f64> = one_month.values.iter().flatten().copied().collect();
                if !valid_values.is_empty() {
                    let avg_prob: f64 =
                        valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                    let scaled_prob = avg_prob * one_month.factor;
                    positions.push(("Exit", scaled_prob));
                }
            }
        }

        if !positions.is_empty() {
            println!("  Position probabilities:");
            for (position, probability) in positions {
                println!("    {}: {:.6}", position, probability);
            }
        }
    }

    println!("\nResponse Metadata:");
    println!("  API Version: {}", response.version);
    println!("  Relays Published: {}", response.relays_published);

    Ok(())
}
