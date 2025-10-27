# Onionoo Rust Client

A Rust wrapper for the Onionoo Tor network status protocol, providing a convenient and type-safe interface to query information about Tor relays and bridges.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
onionoo = "0.1.0"
```

Then, in your Rust file:

```rust
use onionoo::Client;
```

## Usage

### Initializing the Client

To start interacting with the Onionoo API, you first need to create a `Client` instance.

```rust
use onionoo::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client with the default Onionoo URL
    let client = Client::new();

    // Or, if you want to use another url
    let custom_client = Client::new_with_base_url("https://your-onionoo-instance.com");

    Ok(())
}
```

### Building a Query

Queries are built using the [`QueryParameters`](src/parameters/mod.rs:10) builder. This allows you to filter and customize the results returned by the API.

```rust
use onionoo::{Client, QueryParameters, parameters::selection::TypeValue, parameters::selection::BoolValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Build a query to get running relays located in the US, limited to 5 results
    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::True)
        .country("US")
        .limit(5);

    println!("Query string: {}", params.to_query_string()); // Example output: ?country=US&limit=5&running=true&type=relay

    Ok(())
}
```

### Calling an Endpoint and Handling Responses

Once you have a client and query parameters, you can call one of the available endpoint functions. All endpoint functions return a `Result` containing either the successful response data or a [`ClientError`](src/client/mod.rs:14).

```rust
use onionoo::{Client, QueryParameters, endpoints, parameters::selection::TypeValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .limit(3);

    // Call the summary endpoint
    match endpoints::summary(&client, params).await {
        Ok(summary_response) => {
            println!("Successfully fetched summary data.");
            println!("Version: {}", summary_response.version);
            println!("Number of relays returned: {}", summary_response.relays.len());
            println!("Number of bridges returned: {}", summary_response.bridges.len());

            // Accessing data from relay summaries
            if let Some(first_relay) = summary_response.relays.first() {
                println!(
                    "First relay: Nickname={}, Fingerprint={}, Addresses={:?}, Running={}",
                    first_relay.n, first_relay.f, first_relay.a, first_relay.r
                );
            }

            // Accessing data from bridge summaries
            if let Some(first_bridge) = summary_response.bridges.first() {
                println!(
                    "First bridge: Nickname={}, Hashed Fingerprint={}, Running={}",
                    first_bridge.n, first_bridge.h, first_bridge.r
                );
            }
        }
        Err(e) => {
            eprintln!("Error fetching summary data: {}", e);
        }
    }

    Ok(())
}
```

### Accessing Data from Response Models

The response models (e.g., [`SummaryResponse`](src/models/mod.rs:35), [`DetailsResponse`](src/models/mod.rs:65)) are structured to mirror the JSON responses from the Onionoo API. They contain common fields like `version`, `relays_published`, `bridges_published`, and vectors of relay-specific and bridge-specific data structures.

For example, a [`SummaryResponse`](src/models/mod.rs:35) contains:

- `relays: Vec<RelaySummary>`
- `bridges: Vec<BridgeSummary>`

A [`RelaySummary`](src/models/mod.rs:40) object has fields like `n` (nickname), `f` (fingerprint), `a` (addresses), and `r` (running status).
A [`BridgeSummary`](src/models/mod.rs:54) object has fields like `n` (nickname), `h` (hashed fingerprint), and `r` (running status).

You can access these fields directly as shown in the previous example. For more detailed information, refer to the structs in the [`models`](src/models/mod.rs) module.

## API Overview

The library provides functions for all major Onionoo endpoints. Each endpoint function takes a `&Client` and `QueryParameters` and returns a `Result` with the corresponding response type.

- **`endpoints::summary(client, params)`**:
  - Fetches a summary document.
  - Returns `Result<models::SummaryResponse, ClientError>`.
  - Provides short summaries of relays and bridges.

- **`endpoints::details(client, params)`**:
  - Fetches a details document.
  - Returns `Result<models::DetailsResponse, ClientError>`.
  - Provides comprehensive information about relays and bridges.

- **`endpoints::bandwidth(client, params)`**:
  - Fetches a bandwidth document.
  - Returns `Result<models::BandwidthResponse, ClientError>`.
  - Provides aggregate bandwidth statistics for relays and bridges.

- **`endpoints::weights(client, params)`**:
  - Fetches a weights document.
  - Returns `Result<models::WeightsResponse, ClientError>`.
  - Provides aggregate statistics on a relay's probability to be selected for paths (relays only).

- **`endpoints::clients(client, params)`**:
  - Fetches a clients document.
  - Returns `Result<models::ClientsResponse, ClientError>`.
  - Provides estimates of the average number of clients connecting to a bridge (bridges only).

- **`endpoints::uptime(client, params)`**:
  - Fetches an uptime document.
  - Returns `Result<models::UptimeResponse, ClientError>`.
  - Provides fractional uptime information for relays and bridges.

## Error Handling

The library uses a custom [`ClientError`](src/client/mod.rs:14) enum to represent various errors that can occur during API interactions. You should handle these errors when calling endpoint functions.

The [`ClientError`](src/client/mod.rs:14) enum includes the following variants:

- **`ClientError::Request(reqwest::Error)`**:
  - Indicates an HTTP request failure. This could be due to network issues, DNS resolution failures, or other problems with the underlying HTTP request.
  - The underlying `reqwest::Error` provides more specific details.

- **`ClientError::StatusCode(u16, String)`**:
  - Indicates that the Onionoo API returned an HTTP error status code (e.g., 400, 404, 500).
  - The variant contains the status code and an error message from the API response body.

- **`ClientError::Deserialization(String)`**:
  - Indicates that the library failed to deserialize the JSON response from the API into the expected Rust struct.
  - This might happen if the API response format changes unexpectedly or if there's a mismatch between the expected model and the actual data.
  - The variant contains a descriptive error message, often from `serde_json`.

- **`ClientError::UrlConstruction(String)`**:
  - Indicates an error while constructing the request URL.
  - For example, if an endpoint path does not start with a `/`.

- **`ClientError::Other(String)`**:
  - A catch-all for other types of client-side errors not covered by the more specific variants.
  - The variant contains a descriptive error message.

### Example Error Handling

```rust
use onionoo::{Client, QueryParameters, endpoints, client::ClientError};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let params = QueryParameters::new().limit(10); // Valid parameters

    match endpoints::summary(&client, params).await {
        Ok(response) => {
            println!("Fetched {} relays.", response.relays.len());
        }
        Err(ClientError::Request(e)) => {
            eprintln!("Network or HTTP request error: {}", e);
        }
        Err(ClientError::StatusCode(code, message)) => {
            eprintln!("API returned error status {}: {}", code, message);
        }
        Err(ClientError::Deserialization(e)) => {
            eprintln!("Failed to parse API response: {}", e);
        }
        Err(ClientError::UrlConstruction(e)) => {
            eprintln!("URL construction error: {}", e);
        }
        Err(ClientError::Other(e)) => {
            eprintln!("An unexpected error occurred: {}", e);
        }
    }
}
```

## Disclaimer

This project is not endorsed by or affiliated with the Tor Project or the Rust Foundation.
