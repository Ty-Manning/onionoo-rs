//! Onionoo API Query Parameters
//!
//! This module provides structures and utilities for building query parameters
//! for the Onionoo API.

use std::collections::HashMap;

/// Main structure for building query parameters
#[derive(Debug, Clone, Default)]
pub struct QueryParameters {
    type_param: Option<TypeValue>,
    running: Option<BoolValue>,
    search: Option<String>,
    lookup: Option<String>,
    country: Option<String>,
    as_param: Option<String>,
    as_name: Option<String>,
    flag: Option<String>,
    first_seen_days: Option<DaysRange>,
    last_seen_days: Option<DaysRange>,
    first_seen_since: Option<Date>,
    last_seen_since: Option<Date>,
    contact: Option<String>,
    family: Option<String>,
    version: Option<VersionList>,
    os: Option<String>,
    host_name: Option<String>,
    recommended_version: Option<BoolValue>,
    fields: Option<FieldsList>,
    order: Option<OrderList>,
    offset: Option<u32>,
    limit: Option<u32>,
}

impl QueryParameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn type_param(mut self, type_value: TypeValue) -> Self {
        self.type_param = Some(type_value);
        self
    }

    pub fn running(mut self, running: BoolValue) -> Self {
        self.running = Some(running);
        self
    }

    pub fn search<T: Into<String>>(mut self, search: T) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn lookup<T: Into<String>>(mut self, lookup: T) -> Self {
        self.lookup = Some(lookup.into());
        self
    }

    pub fn country<T: Into<String>>(mut self, country: T) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn as_param<T: Into<String>>(mut self, as_param: T) -> Self {
        self.as_param = Some(as_param.into());
        self
    }

    pub fn as_name<T: Into<String>>(mut self, as_name: T) -> Self {
        self.as_name = Some(as_name.into());
        self
    }

    pub fn flag<T: Into<String>>(mut self, flag: T) -> Self {
        self.flag = Some(flag.into());
        self
    }

    pub fn first_seen_days(mut self, range: DaysRange) -> Self {
        self.first_seen_days = Some(range);
        self
    }

    pub fn last_seen_days(mut self, range: DaysRange) -> Self {
        self.last_seen_days = Some(range);
        self
    }

    pub fn first_seen_since(mut self, date: Date) -> Self {
        self.first_seen_since = Some(date);
        self
    }

    pub fn last_seen_since(mut self, date: Date) -> Self {
        self.last_seen_since = Some(date);
        self
    }

    pub fn contact<T: Into<String>>(mut self, contact: T) -> Self {
        self.contact = Some(contact.into());
        self
    }

    pub fn family<T: Into<String>>(mut self, family: T) -> Self {
        self.family = Some(family.into());
        self
    }

    pub fn version(mut self, version: VersionList) -> Self {
        self.version = Some(version);
        self
    }

    pub fn os<T: Into<String>>(mut self, os: T) -> Self {
        self.os = Some(os.into());
        self
    }

    pub fn host_name<T: Into<String>>(mut self, host_name: T) -> Self {
        self.host_name = Some(host_name.into());
        self
    }

    pub fn recommended_version(mut self, recommended: BoolValue) -> Self {
        self.recommended_version = Some(recommended);
        self
    }

    pub fn fields(mut self, fields: FieldsList) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn order(mut self, order: OrderList) -> Self {
        self.order = Some(order);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = HashMap::<String, String>::new();

        if let Some(ref val) = self.type_param {
            params.insert("type".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.running {
            params.insert("running".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.search {
            params.insert("search".to_string(), val.clone());
        }

        if let Some(ref val) = self.lookup {
            params.insert("lookup".to_string(), val.clone());
        }

        if let Some(ref val) = self.country {
            params.insert("country".to_string(), val.clone());
        }

        if let Some(ref val) = self.as_param {
            params.insert("as".to_string(), val.clone());
        }

        if let Some(ref val) = self.as_name {
            params.insert("as_name".to_string(), val.clone());
        }

        if let Some(ref val) = self.flag {
            params.insert("flag".to_string(), val.clone());
        }

        if let Some(ref val) = self.first_seen_days {
            params.insert("first_seen_days".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.last_seen_days {
            params.insert("last_seen_days".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.first_seen_since {
            params.insert("first_seen_since".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.last_seen_since {
            params.insert("last_seen_since".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.contact {
            params.insert("contact".to_string(), val.clone());
        }

        if let Some(ref val) = self.family {
            params.insert("family".to_string(), val.clone());
        }

        if let Some(ref val) = self.version {
            params.insert("version".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.os {
            params.insert("os".to_string(), val.clone());
        }

        if let Some(ref val) = self.host_name {
            params.insert("host_name".to_string(), val.clone());
        }

        if let Some(ref val) = self.recommended_version {
            params.insert("recommended_version".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.fields {
            params.insert("fields".to_string(), val.as_str().to_string());
        }

        if let Some(ref val) = self.order {
            params.insert("order".to_string(), val.as_str().to_string());
        }

        if let Some(val) = self.offset {
            params.insert("offset".to_string(), val.to_string());
        }

        if let Some(val) = self.limit {
            params.insert("limit".to_string(), val.to_string());
        }

        if params.is_empty() {
            String::new()
        } else {
            let mut query_parts = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>();
            query_parts.sort();
            format!("?{}", query_parts.join("&"))
        }
    }
}

/// Selection parameters for filtering results
pub mod selection {

    /// Type parameter values
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TypeValue {
        Relay,
        Bridge,
    }

    impl TypeValue {
        pub fn as_str(&self) -> &'static str {
            match self {
                TypeValue::Relay => "relay",
                TypeValue::Bridge => "bridge",
            }
        }
    }

    /// Boolean parameter values (for running, recommended_version, etc.)
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum BoolValue {
        True,
        False,
    }

    impl BoolValue {
        pub fn as_str(&self) -> &'static str {
            match self {
                BoolValue::True => "true",
                BoolValue::False => "false",
            }
        }
    }

    /// Time range format for days parameters (e.g., "7-14", "30-", "-90")
    #[derive(Debug, Clone)]
    pub struct DaysRange(pub String);

    impl DaysRange {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    /// Date format for since parameters (yyyy-MM-dd)
    #[derive(Debug, Clone)]
    pub struct Date(pub String);

    impl Date {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    /// Version list format (comma-separated, ranges with "..", or mixed)
    #[derive(Debug, Clone)]
    pub struct VersionList(pub String);

    impl VersionList {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    /// AS numbers (can be comma-separated)
    #[derive(Debug, Clone)]
    pub struct AsList(pub String);

    impl AsList {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

pub use selection::*;

/// Field filtering parameters
pub mod fields {

    /// Fields list (comma-separated)
    #[derive(Debug, Clone)]
    pub struct FieldsList(pub String);

    impl FieldsList {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

pub use fields::*;

/// Pagination and ordering parameters
pub mod pagination {

    /// Order specifications (field names with optional "-" prefix for descending)
    #[derive(Debug, Clone)]
    pub struct OrderList(pub String);

    impl OrderList {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

pub use pagination::*;

/// Helper functions for creating parameter values
pub mod helpers {
    use super::*;

    /// Create a DaysRange from various formats
    /// Examples:
    /// - "7" -> "7-7"
    /// - "7-14" -> "7-14"
    /// - "30-" -> "30-"
    /// - "-90" -> "-90"
    pub fn days_range<T: Into<String>>(range: T) -> DaysRange {
        DaysRange(range.into())
    }

    /// Create a Date from yyyy-MM-dd format
    pub fn date<T: Into<String>>(date: T) -> Date {
        Date(date.into())
    }

    /// Create a VersionList from comma-separated versions or ranges
    pub fn version_list<T: Into<String>>(versions: T) -> VersionList {
        VersionList(versions.into())
    }

    /// Create a FieldsList from comma-separated field names
    pub fn fields_list<T: Into<String>>(fields: T) -> FieldsList {
        FieldsList(fields.into())
    }

    /// Create an OrderList from field names with optional "-" prefix
    /// Example: "consensus_weight,-first_seen"
    pub fn order_list<T: Into<String>>(order: T) -> OrderList {
        OrderList(order.into())
    }

    /// Create an AsList from comma-separated AS numbers
    pub fn as_list<T: Into<String>>(as_numbers: T) -> AsList {
        AsList(as_numbers.into())
    }
}

pub use helpers::*;

/// All available parameters grouped by category
pub mod all {

    /// All selection parameter names
    pub const SELECTION_PARAMS: &[&str] = &[
        "type",
        "running",
        "search",
        "lookup",
        "country",
        "as",
        "as_name",
        "flag",
        "first_seen_days",
        "last_seen_days",
        "first_seen_since",
        "last_seen_since",
        "contact",
        "family",
        "version",
        "os",
        "host_name",
        "recommended_version",
    ];

    /// All field filtering parameter names
    pub const FIELD_PARAMS: &[&str] = &["fields"];

    /// All pagination parameter names
    pub const PAGINATION_PARAMS: &[&str] = &["order", "offset", "limit"];

    /// All parameters combined
    pub const ALL_PARAMS: &[&str] = &[
        "type",
        "running",
        "search",
        "lookup",
        "country",
        "as",
        "as_name",
        "flag",
        "first_seen_days",
        "last_seen_days",
        "first_seen_since",
        "last_seen_since",
        "contact",
        "family",
        "version",
        "os",
        "host_name",
        "recommended_version",
        "fields",
        "order",
        "offset",
        "limit",
    ];
}

/// Parameter descriptions for documentation
pub const PARAMETER_DESCRIPTIONS: &[(&str, &str)] = &[
    (
        "type",
        concat!(
            "Return only relay (parameter value relay) or only bridge documents (parameter value bridge). ",
            "Parameter values are case-insensitive."
        ),
    ),
    (
        "running",
        concat!(
            "Return only running (parameter value true) or only non-running relays and/or bridges (parameter value false). ",
            "Parameter values are case-insensitive."
        ),
    ),
    (
        "search",
        concat!(
            "Return only (1) relays with the parameter value matching (part of a) nickname, (possibly $-prefixed) beginning of a hex-encoded fingerprint, ",
            "any 4 hex character block of a space-separated fingerprint, beginning of a base64-encoded fingerprint without trailing equal signs, ",
            "or beginning of an IP address (possibly enclosed in square brackets in case of IPv6), (2) bridges with (part of a) nickname ",
            "or (possibly $-prefixed) beginning of a hashed hex-encoded fingerprint, and (3) relays and/or bridges matching a given qualified search term. ",
            "Searches are case-insensitive, except for base64-encoded fingerprints."
        ),
    ),
    (
        "lookup",
        concat!(
            "Return only the relay with the parameter value matching the fingerprint or the bridge with the parameter value matching the hashed fingerprint. ",
            "Fingerprints should always be hashed using SHA-1. Lookups only work for full fingerprints or hashed fingerprints consisting of 40 hex characters. ",
            "Lookups are case-insensitive."
        ),
    ),
    (
        "country",
        concat!(
            "Return only relays which are located in the given country as identified by a two-letter country code. ",
            "Filtering by country code is case-insensitive. The special country code xz can be used for relays that were not found in the GeoIP database."
        ),
    ),
    (
        "as",
        concat!(
            "Return only relays which are located in either one of the given autonomous systems (AS) as identified by AS number (with or without preceding \"AS\" part). ",
            "Multiple AS numbers can be provided separated by commas. Filtering by AS number is case-insensitive. ",
            "The special AS number 0 can be used for relays that were not found in the GeoIP database."
        ),
    ),
    (
        "as_name",
        concat!(
            "Return only relays with the parameter value matching (part of) the autonomous system (AS) name they are located in. ",
            "If the parameter value contains spaces, only relays are returned which contain all space-separated parts in their AS name. ",
            "Only printable ASCII characters are permitted in the parameter value, some of which need to be percent-encoded."
        ),
    ),
    (
        "flag",
        concat!(
            "Return only relays which have the given relay flag assigned by the directory authorities. ",
            "Note that if the flag parameter is specified more than once, only the first parameter value will be considered. ",
            "Filtering by flag is case-insensitive."
        ),
    ),
    (
        "first_seen_days",
        concat!(
            "Return only relays or bridges which have first been seen during the given range of days ago. ",
            "A parameter value \"x-y\" with x <= y returns relays or bridges that have first been seen at least x and at most y days ago. ",
            "Accepted short forms are \"x\", \"x-\", and \"-y\" which are interpreted as \"x-x\", \"x-infinity\", and \"0-y\"."
        ),
    ),
    (
        "last_seen_days",
        concat!(
            "Return only relays or bridges which have last been seen during the given range of days ago. ",
            "A parameter value \"x-y\" with x <= y returns relays or bridges that have last been seen at least x and at most y days ago. ",
            "Note that relays and bridges that haven't been running in the past week are not included in results, so that setting x to 8 or higher will lead to an empty result set."
        ),
    ),
    (
        "first_seen_since",
        "Return only relays or bridges which have first been seen after the given date. The date has to be passed in the format \"yyyy-MM-dd\".",
    ),
    (
        "last_seen_since",
        concat!(
            "Return only relays or bridges which have last been seen after the given date. The date has to be passed in the format \"yyyy-MM-dd\". ",
            "Note that relays and bridges that haven't been running in the past week are not included in results."
        ),
    ),
    (
        "contact",
        concat!(
            "Return only relays with the parameter value matching (part of) the contact line. ",
            "If the parameter value contains spaces, only relays are returned which contain all space-separated parts in their contact line. ",
            "Only printable ASCII characters are permitted in the parameter value, some of which need to be percent-encoded. Comparisons are case-insensitive."
        ),
    ),
    (
        "family",
        concat!(
            "Return only the relay whose fingerprint matches the parameter value and all relays that this relay has listed in its family by fingerprint ",
            "and that in turn have listed this relay in their family by fingerprint. The provided relay fingerprint must consist of 40 hex characters ",
            "where case does not matter, and it must not be hashed using SHA-1. Bridges are not contained in the result."
        ),
    ),
    (
        "version",
        concat!(
            "Return only relays or bridges running either Tor version from a list or range given in the parameter value. ",
            "Tor versions must be provided without the leading \"Tor\" part. Multiple versions can either be provided as a comma-separated list (\",\"), ",
            "as a range separated by two dots (\"..\"), or as a list of ranges. Provided versions are parsed and matched by parsed dotted numbers, rather than by string prefix."
        ),
    ),
    (
        "os",
        "Return only relays or bridges running on an operating system that starts with the parameter value. Searches are case-insensitive.",
    ),
    (
        "host_name",
        concat!(
            "Return only relays with a domain name ending in the given (partial) host name. ",
            "Searches for subdomains of a specific domain should ideally be prefixed with a period, for example: \".csail.mit.edu\". ",
            "Non-ASCII host name characters must be encoded as punycode. Filtering by host name is case-insensitive."
        ),
    ),
    (
        "recommended_version",
        concat!(
            "Return only relays and bridges running a Tor software version that is recommended (parameter value true) or not recommended by the directory authorities (parameter value false). ",
            "Uses the version in the consensus or bridge network status. Relays and bridges are not contained in either result, if the version they are running is not known. ",
            "Parameter values are case-insensitive."
        ),
    ),
    (
        "fields",
        concat!(
            "Comma-separated list of fields that will be included in the result. So far, only top-level fields in relay or bridge objects of details documents can be specified, ",
            "e.g., nickname,hashed_fingerprint. If the fields parameter is provided, all other fields which are not contained in the provided list will be removed from the result. ",
            "Field names are case-insensitive."
        ),
    ),
    (
        "order",
        concat!(
            "Re-order results by a comma-separated list of fields in ascending or descending order. Results are first ordered by the first list element, then by the second, and so on. ",
            "Possible fields for ordering are: consensus_weight and first_seen. Field names are case-insensitive. ",
            "Ascending order is the default; descending order is selected by prepending fields with a minus sign (-). ",
            "Field names can be listed at most once in either ascending or descending order."
        ),
    ),
    (
        "offset",
        concat!(
            "Skip the given number of relays and/or bridges. Relays are skipped first, then bridges. ",
            "Non-positive offset values are treated as zero and don't change the result."
        ),
    ),
    (
        "limit",
        concat!(
            "Limit result to the given number of relays and/or bridges. Relays are kept first, then bridges. ",
            "Non-positive limit values are treated as zero and lead to an empty result. When used together with offset, the offsetting step precedes the limiting step."
        ),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parameters_empty() {
        let params = QueryParameters::new();
        assert_eq!(params.to_query_string(), "");
    }

    #[test]
    fn test_query_parameters_single() {
        let params = QueryParameters::new().type_param(TypeValue::Relay);

        assert_eq!(params.to_query_string(), "?type=relay");
    }

    #[test]
    fn test_query_parameters_multiple() {
        let params = QueryParameters::new()
            .type_param(TypeValue::Relay)
            .running(BoolValue::True)
            .country("US");

        let query = params.to_query_string();
        assert!(query.contains("type=relay"));
        assert!(query.contains("running=true"));
        assert!(query.contains("country=US"));
    }

    #[test]
    fn test_days_range() {
        let range = days_range("7-14");
        assert_eq!(range.as_str(), "7-14");
    }

    #[test]
    fn test_date() {
        let date = date("2023-10-27");
        assert_eq!(date.as_str(), "2023-10-27");
    }

    #[test]
    fn test_type_value() {
        assert_eq!(TypeValue::Relay.as_str(), "relay");
        assert_eq!(TypeValue::Bridge.as_str(), "bridge");
    }

    #[test]
    fn test_bool_value() {
        assert_eq!(BoolValue::True.as_str(), "true");
        assert_eq!(BoolValue::False.as_str(), "false");
    }
}
