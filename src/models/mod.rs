//! Onionoo API Data Models
//!
//! This module contains all the Rust struct definitions for deserializing
//! data from the Onionoo API endpoints based on the protocol specification.

use serde::{Deserialize, Serialize};

/// Base response structure shared by all Onionoo endpoints
#[derive(Debug, Deserialize, Serialize)]
pub struct OnionooResponse<T, U> {
    pub version: String,
    #[serde(rename = "next_major_version_scheduled")]
    pub next_major_version_scheduled: Option<String>,
    #[serde(rename = "build_revision")]
    pub build_revision: Option<String>,
    #[serde(rename = "relays_published")]
    pub relays_published: String,
    #[serde(rename = "relays_skipped")]
    pub relays_skipped: Option<u64>,
    pub relays: Vec<T>,
    #[serde(rename = "relays_truncated")]
    pub relays_truncated: Option<u64>,
    #[serde(rename = "bridges_published")]
    pub bridges_published: String,
    #[serde(rename = "bridges_skipped")]
    pub bridges_skipped: Option<u64>,
    pub bridges: Vec<U>,
    #[serde(rename = "bridges_truncated")]
    pub bridges_truncated: Option<u64>,
}

// ==================== SUMMARY DOCUMENTS ====================
/// Summary document response type
pub type SummaryResponse = OnionooResponse<RelaySummary, BridgeSummary>;

/// Relay summary object (Summary endpoint)
/// Contains short summaries with nicknames, fingerprints, IP addresses, and running information
#[derive(Debug, Deserialize, Serialize)]
pub struct RelaySummary {
    /// Relay nickname consisting of 1–19 alphanumerical characters
    pub n: String,
    /// Relay fingerprint consisting of 40 upper-case hexadecimal characters
    pub f: String,
    /// Array of IPv4 or IPv6 addresses where the relay accepts onion-routing connections
    pub a: Vec<String>,
    /// Boolean field saying whether this relay was listed as running in the last relay network status consensus
    pub r: bool,
}

/// Bridge summary object (Summary endpoint)
/// Contains bridges with hashed fingerprints and running information
#[derive(Debug, Deserialize, Serialize)]
pub struct BridgeSummary {
    /// Bridge nickname consisting of 1–19 alphanumerical characters
    pub n: String,
    /// SHA-1 hash of the bridge fingerprint consisting of 40 upper-case hexadecimal characters
    pub h: String,
    /// Boolean field saying whether this bridge was successfully tested by bridgestrap
    pub r: bool,
}

// ==================== DETAILS DOCUMENTS ====================
/// Details document response type
pub type DetailsResponse = OnionooResponse<RelayDetails, BridgeDetails>;

/// Relay details object (Details endpoint)
/// Based on network statuses, server descriptors, and TorDNSEL data
#[derive(Debug, Deserialize, Serialize)]
pub struct RelayDetails {
    /// Relay nickname consisting of 1–19 alphanumerical characters
    pub nickname: String,
    /// Relay fingerprint consisting of 40 upper-case hexadecimal characters
    pub fingerprint: String,
    /// Array of IPv4 or IPv6 addresses and TCP ports where the relay accepts onion-routing connections
    #[serde(rename = "or_addresses")]
    pub or_addresses: Vec<String>,
    /// Array of IPv4 addresses that the relay used to exit to the Internet in the past 24 hours
    #[serde(rename = "exit_addresses")]
    pub exit_addresses: Option<Vec<String>>,
    /// IPv4 address and TCP port where the relay accepts directory connections
    #[serde(rename = "dir_address")]
    pub dir_address: Option<String>,
    /// UTC timestamp when this relay was last seen in a network status consensus
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    /// UTC timestamp when this relay last stopped announcing an IPv4 or IPv6 address or TCP port
    #[serde(rename = "last_changed_address_or_port")]
    pub last_changed_address_or_port: String,
    /// UTC timestamp when this relay was first seen in a network status consensus
    #[serde(rename = "first_seen")]
    pub first_seen: String,
    /// Boolean field saying whether this relay was listed as running in the last relay network status consensus
    pub running: bool,
    /// Boolean field saying whether this relay indicated that it is hibernating
    pub hibernating: Option<bool>,
    /// Array of relay flags that the directory authorities assigned to this relay
    pub flags: Option<Vec<String>>,
    /// Two-letter lower-case country code as found in a GeoIP database
    pub country: Option<String>,
    /// Country name as found in a GeoIP database
    #[serde(rename = "country_name")]
    pub country_name: Option<String>,
    /// Region name as found in a GeoIP database
    #[serde(rename = "region_name")]
    pub region_name: Option<String>,
    /// City name as found in a GeoIP database
    #[serde(rename = "city_name")]
    pub city_name: Option<String>,
    /// Latitude as found in a GeoIP database
    pub latitude: Option<f64>,
    /// Longitude as found in a GeoIP database
    pub longitude: Option<f64>,
    /// AS number as found in an AS database
    #[serde(rename = "as")]
    pub as_number: Option<String>,
    /// AS name as found in an AS database
    #[serde(rename = "as_name")]
    pub as_name: Option<String>,
    /// Weight assigned to this relay by the directory authorities
    #[serde(rename = "consensus_weight")]
    pub consensus_weight: u64,
    /// Host names with verified A records
    #[serde(rename = "verified_host_names")]
    pub verified_host_names: Option<Vec<String>>,
    /// Host names without verified A records
    #[serde(rename = "unverified_host_names")]
    pub unverified_host_names: Option<Vec<String>>,
    /// UTC timestamp when the relay was last (re-)started
    #[serde(rename = "last_restarted")]
    pub last_restarted: Option<String>,
    /// Average bandwidth in bytes per second that this relay is willing to sustain over long periods
    #[serde(rename = "bandwidth_rate")]
    pub bandwidth_rate: Option<u64>,
    /// Bandwidth in bytes per second that this relay is willing to sustain in very short intervals
    #[serde(rename = "bandwidth_burst")]
    pub bandwidth_burst: Option<u64>,
    /// Bandwidth estimate in bytes per second of the capacity this relay can handle
    #[serde(rename = "observed_bandwidth")]
    pub observed_bandwidth: Option<u64>,
    /// Bandwidth in bytes per second that this relay is willing and capable to provide
    #[serde(rename = "advertised_bandwidth")]
    pub advertised_bandwidth: Option<u64>,
    /// Timestamp indicating relay has reached an overloaded state
    #[serde(rename = "overload_general_timestamp")]
    pub overload_general_timestamp: Option<u64>,
    /// Array of exit-policy lines
    #[serde(rename = "exit_policy")]
    pub exit_policy: Option<Vec<String>>,
    /// Summary version of the relay's exit policy
    #[serde(rename = "exit_policy_summary")]
    pub exit_policy_summary: Option<ExitPolicySummary>,
    /// Summary version of the relay's IPv6 exit policy
    #[serde(rename = "exit_policy_v6_summary")]
    pub exit_policy_v6_summary: Option<ExitPolicySummary>,
    /// Contact address of the relay operator
    pub contact: Option<String>,
    /// Platform string containing operating system and Tor version details
    pub platform: Option<String>,
    /// Tor software version without leading "Tor"
    pub version: Option<String>,
    /// Boolean field saying whether the Tor software version of this relay is recommended
    #[serde(rename = "recommended_version")]
    pub recommended_version: Option<bool>,
    /// Status of the Tor software version of this relay
    #[serde(rename = "version_status")]
    pub version_status: Option<String>,
    /// Array of fingerprints of relays that are in an effective, mutual family relationship
    #[serde(rename = "effective_family")]
    pub effective_family: Option<Vec<String>>,
    /// Array of fingerprints of relays that are not in an effective, mutual family relationship
    #[serde(rename = "alleged_family")]
    pub alleged_family: Option<Vec<String>>,
    /// Array of fingerprints of relays that can be reached by following effective family relationships
    #[serde(rename = "indirect_family")]
    pub indirect_family: Option<Vec<String>>,
    /// Fraction of this relay's consensus weight compared to the sum of all consensus weights
    #[serde(rename = "consensus_weight_fraction")]
    pub consensus_weight_fraction: Option<f64>,
    /// Probability of this relay to be selected for the guard position
    #[serde(rename = "guard_probability")]
    pub guard_probability: Option<f64>,
    /// Probability of this relay to be selected for the middle position
    #[serde(rename = "middle_probability")]
    pub middle_probability: Option<f64>,
    /// Probability of this relay to be selected for the exit position
    #[serde(rename = "exit_probability")]
    pub exit_probability: Option<f64>,
    /// Boolean field saying whether the consensus weight of this relay is based on measurements
    pub measured: Option<bool>,
    /// Array of addresses that the relay claims to accept but that are unreachable
    #[serde(rename = "unreachable_or_addresses")]
    pub unreachable_or_addresses: Option<Vec<String>>,
}

/// Bridge details object (Details endpoint)
#[derive(Debug, Deserialize, Serialize)]
pub struct BridgeDetails {
    /// Bridge nickname consisting of 1–19 alphanumerical characters
    pub nickname: String,
    /// SHA-1 hash of the bridge fingerprint consisting of 40 upper-case hexadecimal characters
    #[serde(rename = "hashed_fingerprint")]
    pub hashed_fingerprint: String,
    /// Array of sanitized IPv4 or IPv6 addresses and TCP ports where the bridge accepts connections
    #[serde(rename = "or_addresses")]
    pub or_addresses: Vec<String>,
    /// UTC timestamp when this bridge was last seen in a bridge network status
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    /// UTC timestamp when this bridge was first seen in a bridge network status
    #[serde(rename = "first_seen")]
    pub first_seen: String,
    /// Boolean field saying whether this bridge was successfully tested by bridgestrap
    pub running: bool,
    /// Array of relay flags that the bridge authority assigned to this bridge
    pub flags: Option<Vec<String>>,
    /// UTC timestamp when the bridge was last (re-)started
    #[serde(rename = "last_restarted")]
    pub last_restarted: Option<String>,
    /// Bandwidth in bytes per second that this bridge is willing and capable to provide
    #[serde(rename = "advertised_bandwidth")]
    pub advertised_bandwidth: Option<u64>,
    /// Timestamp indicating bridge has reached an overloaded state
    #[serde(rename = "overload_general_timestamp")]
    pub overload_general_timestamp: Option<u64>,
    /// Platform string containing operating system and Tor version details
    pub platform: Option<String>,
    /// Tor software version without leading "Tor"
    pub version: Option<String>,
    /// Boolean field saying whether the Tor software version of this bridge is recommended
    #[serde(rename = "recommended_version")]
    pub recommended_version: Option<bool>,
    /// Status of the Tor software version of this bridge
    #[serde(rename = "version_status")]
    pub version_status: Option<String>,
    /// Array of (pluggable) transport names supported by this bridge
    pub transports: Option<Vec<String>>,
    /// Array of country codes where this bridge is not served because it is believed to be blocked
    pub blocklist: Option<Vec<String>>,
    /// BridgeDB distributor that this bridge is currently assigned to
    #[serde(rename = "bridgedb_distributor")]
    pub bridgedb_distributor: Option<String>,
    /// Contact address of the bridge operator
    pub contact: Option<String>,
}

/// Summary version of exit policy
#[derive(Debug, Deserialize, Serialize)]
pub struct ExitPolicySummary {
    pub accept: Option<Vec<String>>,
    pub reject: Option<Vec<String>>,
}

// ==================== BANDWIDTH DOCUMENTS ====================
/// Bandwidth document response type
pub type BandwidthResponse = OnionooResponse<RelayBandwidth, BridgeBandwidth>;

/// Relay bandwidth object (Bandwidth endpoint)
/// Contains aggregate statistics of a relay's consumed bandwidth for different time intervals
#[derive(Debug, Deserialize, Serialize)]
pub struct RelayBandwidth {
    /// Relay fingerprint consisting of 40 upper-case hexadecimal characters
    pub fingerprint: String,
    /// Object containing graph history objects with written bytes for different time periods
    #[serde(rename = "write_history")]
    pub write_history: Option<BandwidthHistory>,
    /// Object containing graph history objects with read bytes for different time periods
    #[serde(rename = "read_history")]
    pub read_history: Option<BandwidthHistory>,
    /// JSON object containing the overload-ratelimits information for the relay
    #[serde(rename = "overload_ratelimits")]
    pub overload_ratelimits: Option<OverloadRatelimits>,
    /// JSON object containing the overload-fd-exhausted information for the relay
    #[serde(rename = "overload_fd_exhausted")]
    pub overload_fd_exhausted: Option<OverloadFdExhausted>,
}

/// Bridge bandwidth object (Bandwidth endpoint)
#[derive(Debug, Deserialize, Serialize)]
pub struct BridgeBandwidth {
    /// SHA-1 hash of the bridge fingerprint consisting of 40 upper-case hexadecimal characters
    pub fingerprint: String,
    /// Object containing graph history objects with written bytes for different time periods
    #[serde(rename = "write_history")]
    pub write_history: Option<BandwidthHistory>,
    /// Object containing graph history objects with read bytes for different time periods
    #[serde(rename = "read_history")]
    pub read_history: Option<BandwidthHistory>,
    /// JSON object containing the overload-ratelimits information for the bridge
    #[serde(rename = "overload_ratelimits")]
    pub overload_ratelimits: Option<OverloadRatelimits>,
    /// JSON object containing the overload-fd-exhausted information for the bridge
    #[serde(rename = "overload_fd_exhausted")]
    pub overload_fd_exhausted: Option<OverloadFdExhausted>,
}

/// Bandwidth history for different time periods
#[derive(Debug, Deserialize, Serialize)]
pub struct BandwidthHistory {
    /// History for 1 month
    #[serde(rename = "1_month")]
    pub one_month: Option<GraphHistory>,
    /// History for 6 months
    #[serde(rename = "6_months")]
    pub six_months: Option<GraphHistory>,
    /// History for 1 year
    #[serde(rename = "1_year")]
    pub one_year: Option<GraphHistory>,
    /// History for 5 years
    #[serde(rename = "5_years")]
    pub five_years: Option<GraphHistory>,
}

/// Overload ratelimits information
#[derive(Debug, Deserialize, Serialize)]
pub struct OverloadRatelimits {
    pub timestamp: Option<u64>,
    #[serde(rename = "rate-limit")]
    pub rate_limit: Option<u64>,
    #[serde(rename = "burst-limit")]
    pub burst_limit: Option<u64>,
    #[serde(rename = "read-overload-count")]
    pub read_overload_count: Option<u64>,
    #[serde(rename = "write-overload-count")]
    pub write_overload_count: Option<u64>,
}

/// Overload file descriptor exhausted information
#[derive(Debug, Deserialize, Serialize)]
pub struct OverloadFdExhausted {
    pub timestamp: Option<u64>,
}

// ==================== WEIGHTS DOCUMENTS ====================
/// Weights document response type (relays only)
pub type WeightsResponse = OnionooResponse<RelayWeights, ()>;

/// Relay weights object (Weights endpoint)
/// Contains aggregate statistics of a relay's probability to be selected by clients for building paths
#[derive(Debug, Deserialize, Serialize)]
pub struct RelayWeights {
    /// Relay fingerprint consisting of 40 upper-case hexadecimal characters
    pub fingerprint: String,
    /// History object containing the fraction of this relay's consensus weight
    #[serde(rename = "consensus_weight_fraction")]
    pub consensus_weight_fraction: Option<WeightHistory>,
    /// History object containing the probability of this relay to be selected for the guard position
    #[serde(rename = "guard_probability")]
    pub guard_probability: Option<WeightHistory>,
    /// History object containing the probability of this relay to be selected for the middle position
    #[serde(rename = "middle_probability")]
    pub middle_probability: Option<WeightHistory>,
    /// History object containing the probability of this relay to be selected for the exit position
    #[serde(rename = "exit_probability")]
    pub exit_probability: Option<WeightHistory>,
    /// History object containing the absolute consensus weight of this relay
    #[serde(rename = "consensus_weight")]
    pub consensus_weight: Option<WeightHistory>,
}

/// Weight history for different time periods
#[derive(Debug, Deserialize, Serialize)]
pub struct WeightHistory {
    /// History for 1 month
    #[serde(rename = "1_month")]
    pub one_month: Option<GraphHistory>,
    /// History for 6 months
    #[serde(rename = "6_months")]
    pub six_months: Option<GraphHistory>,
    /// History for 1 year
    #[serde(rename = "1_year")]
    pub one_year: Option<GraphHistory>,
    /// History for 5 years
    #[serde(rename = "5_years")]
    pub five_years: Option<GraphHistory>,
}

// ==================== CLIENTS DOCUMENTS ====================
/// Clients document response type (bridges only)
pub type ClientsResponse = OnionooResponse<(), BridgeClients>;

/// Bridge clients object (Clients endpoint)
/// Contains estimates of the average number of clients connecting to a bridge every day
#[derive(Debug, Deserialize, Serialize)]
pub struct BridgeClients {
    /// SHA-1 hash of the bridge fingerprint consisting of 40 upper-case hexadecimal characters
    pub fingerprint: String,
    /// Object containing graph history objects with the average number of clients connecting to this bridge
    #[serde(rename = "average_clients")]
    pub average_clients: Option<ClientsHistory>,
}

/// Client history for different time periods
#[derive(Debug, Deserialize, Serialize)]
pub struct ClientsHistory {
    /// History for 1 month
    #[serde(rename = "1_month")]
    pub one_month: Option<GraphHistory>,
    /// History for 6 months
    #[serde(rename = "6_months")]
    pub six_months: Option<GraphHistory>,
    /// History for 1 year
    #[serde(rename = "1_year")]
    pub one_year: Option<GraphHistory>,
    /// History for 5 years
    #[serde(rename = "5_years")]
    pub five_years: Option<GraphHistory>,
}

// ==================== UPTIME DOCUMENTS ====================
/// Uptime document response type
pub type UptimeResponse = OnionooResponse<RelayUptime, BridgeUptime>;

/// Relay uptime object (Uptime endpoint)
/// Contains fractional uptimes of relays
#[derive(Debug, Deserialize, Serialize)]
pub struct RelayUptime {
    /// Relay fingerprint consisting of 40 upper-case hexadecimal characters
    pub fingerprint: String,
    /// Object containing graph history objects with the fractional uptime of this relay
    pub uptime: Option<UptimeHistory>,
    /// Object containing fractional times of this relay having relay flags assigned
    pub flags: Option<FlagHistory>,
}

/// Bridge uptime object (Uptime endpoint)
/// Contains fractional uptimes of bridges
#[derive(Debug, Deserialize, Serialize)]
pub struct BridgeUptime {
    /// SHA-1 hash of the bridge fingerprint consisting of 40 upper-case hexadecimal characters
    pub fingerprint: String,
    /// Uptime history objects for different time periods
    pub uptime: Option<UptimeHistory>,
}

/// Uptime history for different time periods
#[derive(Debug, Deserialize, Serialize)]
pub struct UptimeHistory {
    /// History for 1 month
    #[serde(rename = "1_month")]
    pub one_month: Option<GraphHistory>,
    /// History for 6 months
    #[serde(rename = "6_months")]
    pub six_months: Option<GraphHistory>,
    /// History for 1 year
    #[serde(rename = "1_year")]
    pub one_year: Option<GraphHistory>,
    /// History for 5 years
    #[serde(rename = "5_years")]
    pub five_years: Option<GraphHistory>,
}

/// Flag history for different time periods
#[derive(Debug, Deserialize, Serialize)]
pub struct FlagHistory {
    #[serde(flatten)]
    pub flags: std::collections::HashMap<String, UptimeHistory>,
}

// ==================== GRAPH HISTORY OBJECTS ====================
/// Graph history objects contained in bandwidth, weights, clients, and uptime documents
#[derive(Debug, Deserialize, Serialize)]
pub struct GraphHistory {
    /// UTC timestamp of the first data point
    pub first: String,
    /// UTC timestamp of the last data point
    pub last: String,
    /// Time interval between two data points in seconds
    pub interval: u64,
    /// Factor by which subsequent data values need to be multiplied
    pub factor: f64,
    /// Number of provided data points
    pub count: Option<u64>,
    /// Array of normalized values between 0 and 999
    pub values: Vec<Option<f64>>,
}