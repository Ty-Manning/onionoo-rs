//! Onionoo Rust Client
//!
//! A Rust wrapper for the Onionoo Tor network status protocol, providing a convenient and type-safe interface to query information about Tor relays and bridges.

pub mod client;
pub mod endpoints;
pub mod models;
pub mod parameters;
pub mod utils;

// Re-export commonly used types for convenience
pub use client::Client;
pub use endpoints::{bandwidth, clients, details, summary, uptime, weights};
pub use parameters::{QueryParameters, selection};
