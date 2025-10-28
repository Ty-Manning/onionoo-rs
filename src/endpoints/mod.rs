//! Onionoo API Endpoints
//!
//! This module provides easy-to-use asynchronous functions for each API endpoint.

use crate::{
    client::{Client, ClientError},
    models,
    parameters::QueryParameters,
};

/// Base URL for the Onionoo API
pub const BASE_URL: &str = "https://onionoo.torproject.org";

/// Available API endpoints
pub enum Endpoint {
    /// Summary endpoint - returns a summary document
    Summary,
    /// Details endpoint - returns a details document  
    Details,
    /// Bandwidth endpoint - returns a bandwidth document
    Bandwidth,
    /// Weights endpoint - returns a weights document
    Weights,
    /// Clients endpoint - returns a clients document
    Clients,
    /// Uptime endpoint - returns an uptime document
    Uptime,
}

impl Endpoint {
    pub fn method(&self) -> &'static str {
        match self {
            Endpoint::Summary => "GET",
            Endpoint::Details => "GET",
            Endpoint::Bandwidth => "GET",
            Endpoint::Weights => "GET",
            Endpoint::Clients => "GET",
            Endpoint::Uptime => "GET",
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            Endpoint::Summary => "/summary",
            Endpoint::Details => "/details",
            Endpoint::Bandwidth => "/bandwidth",
            Endpoint::Weights => "/weights",
            Endpoint::Clients => "/clients",
            Endpoint::Uptime => "/uptime",
        }
    }

    pub fn url(&self) -> String {
        format!("{}{}", BASE_URL, self.path())
    }

    pub fn name(&self) -> &'static str {
        match self {
            Endpoint::Summary => "summary",
            Endpoint::Details => "details",
            Endpoint::Bandwidth => "bandwidth",
            Endpoint::Weights => "weights",
            Endpoint::Clients => "clients",
            Endpoint::Uptime => "uptime",
        }
    }
}

/// Detailed endpoint descriptions
pub const ENDPOINT_DESCRIPTIONS: &[(&str, &str)] = &[
    (
        "summary",
        concat!(
            "Returns a summary document containing short summaries of relays with nicknames, ",
            "fingerprints, IP addresses, and running information as well as bridges with ",
            "hashed fingerprints and running information."
        ),
    ),
    (
        "details",
        concat!(
            "Returns a details document based on network statuses published by the Tor directories, ",
            "server descriptors published by relays and bridges, and data published by Tor network ",
            "services TorDNSEL and BridgeDB. Contains comprehensive information about relays and bridges."
        ),
    ),
    (
        "bandwidth",
        concat!(
            "Returns a bandwidth document containing aggregate statistics of a relay's or bridge's ",
            "consumed bandwidth for different time intervals. Only updated when a relay or bridge ",
            "publishes a new server descriptor, which may take up to 18 hours during normal operation."
        ),
    ),
    (
        "weights",
        concat!(
            "Returns a weights document containing aggregate statistics of a relay's probability ",
            "to be selected by clients for building paths. Contains different time intervals and ",
            "is available for relays only."
        ),
    ),
    (
        "clients",
        concat!(
            "Returns a clients document containing estimates of the average number of clients ",
            "connecting to a bridge every day. Only available for bridges, not relays."
        ),
    ),
    (
        "uptime",
        concat!(
            "Returns an uptime document containing fractional uptimes of relays and bridges. ",
            "Contains different time intervals and is available for both relays and bridges."
        ),
    ),
];

/// Response data types for each endpoint
pub enum ResponseType {
    /// SummaryResponse type
    Summary,
    /// DetailsResponse type  
    Details,
    /// BandwidthResponse type
    Bandwidth,
    /// WeightsResponse type
    Weights,
    /// ClientsResponse type
    Clients,
    /// UptimeResponse type
    Uptime,
}

impl Endpoint {
    pub fn response_type(&self) -> ResponseType {
        match self {
            Endpoint::Summary => ResponseType::Summary,
            Endpoint::Details => ResponseType::Details,
            Endpoint::Bandwidth => ResponseType::Bandwidth,
            Endpoint::Weights => ResponseType::Weights,
            Endpoint::Clients => ResponseType::Clients,
            Endpoint::Uptime => ResponseType::Uptime,
        }
    }
}

// ==================== ASYNC ENDPOINT FUNCTIONS ====================

/// Returns a summary document containing short summaries of relays with nicknames,
/// fingerprints, IP addresses, and running information as well as bridges with
/// hashed fingerprints and running information.
///
/// # Examples
///
/// ```no_run
/// use onionoo::{Client, QueryParameters, endpoints, selection::TypeValue};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let params = QueryParameters::new()
///     .type_param(TypeValue::Relay)
///     .limit(10);
/// let response = endpoints::summary(&client, params).await?;
/// println!("Found {} relays", response.relays.len());
/// # Ok(())
/// # }
/// ```
pub async fn summary(
    client: &Client,
    params: QueryParameters,
) -> Result<models::SummaryResponse, ClientError> {
    client.get("/summary", params).await
}

/// Returns a details document based on network statuses published by the Tor directories,
/// server descriptors published by relays and bridges, and data published by Tor network
/// services TorDNSEL and BridgeDB. Contains comprehensive information about relays and bridges.
///
/// # Examples
///
/// ```no_run
/// use onionoo::{Client, QueryParameters, endpoints, selection::TypeValue};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let params = QueryParameters::new()
///     .type_param(TypeValue::Relay)
///     .limit(5);
/// let response = endpoints::details(&client, params).await?;
/// println!("Found {} relays", response.relays.len());
/// # Ok(())
/// # }
/// ```
pub async fn details(
    client: &Client,
    params: QueryParameters,
) -> Result<models::DetailsResponse, ClientError> {
    client.get("/details", params).await
}

/// Returns a bandwidth document containing aggregate statistics of a relay's or bridge's
/// consumed bandwidth for different time intervals. Only updated when a relay or bridge
/// publishes a new server descriptor, which may take up to 18 hours during normal operation.
///
/// # Examples
///
/// ```no_run
/// use onionoo::{Client, QueryParameters, endpoints, selection::TypeValue};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let params = QueryParameters::new()
///     .type_param(TypeValue::Relay)
///     .limit(10);
/// let response = endpoints::bandwidth(&client, params).await?;
/// println!("Found {} relays", response.relays.len());
/// # Ok(())
/// # }
/// ```
pub async fn bandwidth(
    client: &Client,
    params: QueryParameters,
) -> Result<models::BandwidthResponse, ClientError> {
    client.get("/bandwidth", params).await
}

/// Returns a weights document containing aggregate statistics of a relay's probability
/// to be selected by clients for building paths. Contains different time intervals and
/// is available for relays only.
///
/// # Examples
///
/// ```no_run
/// use onionoo::{Client, QueryParameters, endpoints, selection::TypeValue};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let params = QueryParameters::new()
///     .type_param(TypeValue::Relay)
///     .limit(10);
/// let response = endpoints::weights(&client, params).await?;
/// println!("Found {} relays", response.relays.len());
/// # Ok(())
/// # }
/// ```
pub async fn weights(
    client: &Client,
    params: QueryParameters,
) -> Result<models::WeightsResponse, ClientError> {
    client.get("/weights", params).await
}

/// Returns a clients document containing estimates of the average number of clients
/// connecting to a bridge every day. Only available for bridges, not relays.
///
/// # Examples
///
/// ```no_run
/// use onionoo::{Client, QueryParameters, endpoints, selection::TypeValue};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let params = QueryParameters::new()
///     .type_param(TypeValue::Bridge)
///     .limit(10);
/// let response = endpoints::clients(&client, params).await?;
/// println!("Found {} bridges", response.bridges.len());
/// # Ok(())
/// # }
/// ```
pub async fn clients(
    client: &Client,
    params: QueryParameters,
) -> Result<models::ClientsResponse, ClientError> {
    client.get("/clients", params).await
}

/// Returns an uptime document containing fractional uptimes of relays and bridges.
/// Contains different time intervals and is available for both relays and bridges.
///
/// # Examples
///
/// ```no_run
/// use onionoo::{Client, QueryParameters, endpoints, selection::TypeValue};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let params = QueryParameters::new()
///     .type_param(TypeValue::Relay)
///     .limit(10);
/// let response = endpoints::uptime(&client, params).await?;
/// println!("Found {} relays", response.relays.len());
/// # Ok(())
/// # }
/// ```
pub async fn uptime(
    client: &Client,
    params: QueryParameters,
) -> Result<models::UptimeResponse, ClientError> {
    client.get("/uptime", params).await
}

/// Helper functions for working with endpoints
pub mod helpers {
    use super::*;

    /// Returns all available endpoints
    pub fn all_endpoints() -> Vec<Endpoint> {
        vec![
            Endpoint::Summary,
            Endpoint::Details,
            Endpoint::Bandwidth,
            Endpoint::Weights,
            Endpoint::Clients,
            Endpoint::Uptime,
        ]
    }

    /// Returns endpoints that support both relays and bridges
    pub fn relay_bridge_endpoints() -> Vec<Endpoint> {
        vec![
            Endpoint::Summary,
            Endpoint::Details,
            Endpoint::Bandwidth,
            Endpoint::Uptime,
        ]
    }

    /// Returns endpoints that support only relays
    pub fn relay_only_endpoints() -> Vec<Endpoint> {
        vec![Endpoint::Weights]
    }

    /// Returns endpoints that support only bridges  
    pub fn bridge_only_endpoints() -> Vec<Endpoint> {
        vec![Endpoint::Clients]
    }

    /// Finds an endpoint by name
    pub fn find_by_name(name: &str) -> Option<Endpoint> {
        match name.to_lowercase().as_str() {
            "summary" => Some(Endpoint::Summary),
            "details" => Some(Endpoint::Details),
            "bandwidth" => Some(Endpoint::Bandwidth),
            "weights" => Some(Endpoint::Weights),
            "clients" => Some(Endpoint::Clients),
            "uptime" => Some(Endpoint::Uptime),
            _ => None,
        }
    }
}
