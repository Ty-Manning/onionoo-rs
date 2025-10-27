use onionoo::{Client, QueryParameters, endpoints};
use onionoo::parameters::{TypeValue, BoolValue, days_range, date};

#[tokio::test]
async fn test_summary_endpoint() {
    let client = Client::new();
    let params = QueryParameters::new();
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_details_endpoint() {
    let client = Client::new();
    let params = QueryParameters::new();
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_bandwidth_endpoint() {
    let client = Client::new();
    let params = QueryParameters::new();
    let result = endpoints::bandwidth(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_weights_endpoint() {
    let client = Client::new();
    let params = QueryParameters::new();
    let result = endpoints::weights(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_clients_endpoint() {
    let client = Client::new();
    let params = QueryParameters::new();
    let result = endpoints::clients(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_uptime_endpoint() {
    let client = Client::new();
    let params = QueryParameters::new();
    let result = endpoints::uptime(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== PARAMETER VARIATION TESTS ====================

#[tokio::test]
async fn test_summary_with_relay_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_summary_with_bridge_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Bridge)
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_details_with_running_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .running(BoolValue::True)
        .limit(10);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_bandwidth_with_country_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .country("US")
        .limit(5);
    let result = endpoints::bandwidth(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_weights_with_as_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .as_param("AS13335")
        .limit(5);
    let result = endpoints::weights(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_uptime_with_flag_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .flag("Guard")
        .limit(5);
    let result = endpoints::uptime(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_search_functionality() {
    let client = Client::new();
    let params = QueryParameters::new()
        .search("tor")
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_version_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .version(onionoo::parameters::helpers::version_list("0.4.7"))
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_os_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .os("Linux")
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== PAGINATION TESTS ====================

#[tokio::test]
async fn test_pagination_with_offset_and_limit() {
    let client = Client::new();
    let params = QueryParameters::new()
        .offset(10)
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_pagination_only_offset() {
    let client = Client::new();
    let params = QueryParameters::new()
        .offset(20);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_pagination_only_limit() {
    let client = Client::new();
    let params = QueryParameters::new()
        .limit(3);
    let result = endpoints::bandwidth(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== ORDERING TESTS ====================

#[tokio::test]
async fn test_ordering_by_consensus_weight() {
    let client = Client::new();
    let params = QueryParameters::new()
        .order(onionoo::parameters::helpers::order_list("consensus_weight"))
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_ordering_by_first_seen_descending() {
    let client = Client::new();
    let params = QueryParameters::new()
        .order(onionoo::parameters::helpers::order_list("-first_seen"))
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_ordering_multiple_fields() {
    let client = Client::new();
    let params = QueryParameters::new()
        .order(onionoo::parameters::helpers::order_list("consensus_weight,-first_seen"))
        .limit(5);
    let result = endpoints::weights(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== FIELD FILTERING TESTS ====================

#[tokio::test]
async fn test_field_filtering_summary() {
    let client = Client::new();
    let params = QueryParameters::new()
        .fields(onionoo::parameters::helpers::fields_list("nickname,fingerprint,or_addresses"))
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_field_filtering_details() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== RELAY vs BRIDGE ENDPOINT TESTS ====================

#[tokio::test]
async fn test_weights_endpoint_relays_only() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .limit(5);
    let result = endpoints::weights(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
    // Weights endpoint should only return relays, no bridges
    assert!(response.bridges.is_empty());
}

#[tokio::test]
async fn test_clients_endpoint_bridges_only() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Bridge)
        .limit(5);
    let result = endpoints::clients(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
    // Clients endpoint should only return bridges, no relays
    assert!(response.relays.is_empty());
}

#[tokio::test]
async fn test_bridge_specific_parameters() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Bridge)
        .limit(5);
    let result = endpoints::clients(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== TIME-BASED FILTER TESTS ====================

#[tokio::test]
async fn test_first_seen_days_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .first_seen_days(days_range("7-30"))
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_last_seen_days_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .last_seen_days(days_range("1-7"))
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_first_seen_since_date() {
    let client = Client::new();
    let params = QueryParameters::new()
        .first_seen_since(date("2023-01-01"))
        .limit(5);
    let result = endpoints::bandwidth(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_last_seen_since_date() {
    let client = Client::new();
    let params = QueryParameters::new()
        .last_seen_since(date("2023-10-01"))
        .limit(5);
    let result = endpoints::uptime(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== COMBINED PARAMETER TESTS ====================

#[tokio::test]
async fn test_multiple_parameters_combined() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .running(BoolValue::True)
        .country("DE")
        .flag("Exit")
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_complex_query_with_ordering_and_fields() {
    let client = Client::new();
    let params = QueryParameters::new()
        .type_param(TypeValue::Relay)
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

// ==================== CLIENT CONFIGURATION TESTS ====================

#[tokio::test]
async fn test_custom_base_url() {
    let client = Client::new_with_base_url("https://custom.example.com");
    assert_eq!(client.base_url(), "https://custom.example.com");
    assert!(!client.is_default());
}

#[tokio::test]
async fn test_default_client_configuration() {
    let client = Client::new();
    assert_eq!(client.base_url(), "https://onionoo.torproject.org");
    assert!(client.is_default());
}

#[tokio::test]
async fn test_client_default_trait() {
    let client: Client = Default::default();
    assert_eq!(client.base_url(), "https://onionoo.torproject.org");
    assert!(client.is_default());
}

// ==================== ENDPOINT HELPER FUNCTIONS TESTS ====================

#[tokio::test]
async fn test_all_endpoints_function() {
    let endpoints = onionoo::endpoints::helpers::all_endpoints();
    assert_eq!(endpoints.len(), 6);
}

#[tokio::test]
async fn test_relay_bridge_endpoints_function() {
    let endpoints = onionoo::endpoints::helpers::relay_bridge_endpoints();
    assert_eq!(endpoints.len(), 4);
}

#[tokio::test]
async fn test_relay_only_endpoints_function() {
    let endpoints = onionoo::endpoints::helpers::relay_only_endpoints();
    assert_eq!(endpoints.len(), 1);
}

#[tokio::test]
async fn test_bridge_only_endpoints_function() {
    let endpoints = onionoo::endpoints::helpers::bridge_only_endpoints();
    assert_eq!(endpoints.len(), 1);
}

#[tokio::test]
async fn test_find_endpoint_by_name() {
    let summary = onionoo::endpoints::helpers::find_by_name("summary");
    assert!(summary.is_some());
    
    let details = onionoo::endpoints::helpers::find_by_name("details");
    assert!(details.is_some());
    
    let invalid = onionoo::endpoints::helpers::find_by_name("invalid");
    assert!(invalid.is_none());
}

// ==================== RESPONSE CONTENT VERIFICATION TESTS ====================

#[tokio::test]
async fn test_summary_response_structure() {
    let client = Client::new();
    let params = QueryParameters::new().limit(1);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify basic response structure
    assert!(!response.version.is_empty());
    assert!(!response.relays_published.is_empty());
    assert!(!response.bridges_published.is_empty());
    
    // Check that we have either relays or bridges
    assert!(!response.relays.is_empty() || !response.bridges.is_empty());
}

#[tokio::test]
async fn test_details_response_structure() {
    let client = Client::new();
    let params = QueryParameters::new().limit(1);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify basic response structure
    assert!(!response.version.is_empty());
    
    // Check that we have relay details if relays exist
    if !response.relays.is_empty() {
        let relay = &response.relays[0];
        assert!(!relay.nickname.is_empty());
        assert!(!relay.fingerprint.is_empty());
    }
}

#[tokio::test]
async fn test_bandwidth_response_structure() {
    let client = Client::new();
    let params = QueryParameters::new().limit(1);
    let result = endpoints::bandwidth(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify basic response structure
    assert!(!response.version.is_empty());
    
    // Check that we have bandwidth data if relays exist
    if !response.relays.is_empty() {
        let relay = &response.relays[0];
        assert!(!relay.fingerprint.is_empty());
        // write_history and read_history are optional
    }
}

#[tokio::test]
async fn test_weights_response_structure() {
    let client = Client::new();
    let params = QueryParameters::new().limit(1);
    let result = endpoints::weights(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify basic response structure
    assert!(!response.version.is_empty());
    
    // Check that we have weights data if relays exist
    if !response.relays.is_empty() {
        let relay = &response.relays[0];
        assert!(!relay.fingerprint.is_empty());
    }
    
    // Weights should not have bridges
    assert!(response.bridges.is_empty());
}

#[tokio::test]
async fn test_clients_response_structure() {
    let client = Client::new();
    let params = QueryParameters::new().limit(1);
    let result = endpoints::clients(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify basic response structure
    assert!(!response.version.is_empty());
    
    // Check that we have clients data if bridges exist
    if !response.bridges.is_empty() {
        let bridge = &response.bridges[0];
        assert!(!bridge.fingerprint.is_empty());
    }
    
    // Clients should not have relays
    assert!(response.relays.is_empty());
}

#[tokio::test]
async fn test_uptime_response_structure() {
    let client = Client::new();
    let params = QueryParameters::new().limit(1);
    let result = endpoints::uptime(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify basic response structure
    assert!(!response.version.is_empty());
    
    // Check that we have uptime data if relays exist
    if !response.relays.is_empty() {
        let relay = &response.relays[0];
        assert!(!relay.fingerprint.is_empty());
    }
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_limit_zero() {
    let client = Client::new();
    let params = QueryParameters::new()
        .limit(0);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_offset_zero() {
    let client = Client::new();
    let params = QueryParameters::new()
        .offset(0)
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_large_offset() {
    let client = Client::new();
    let params = QueryParameters::new()
        .offset(1000)
        .limit(5);
    let result = endpoints::bandwidth(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_search_with_special_characters() {
    let client = Client::new();
    let params = QueryParameters::new()
        .search("test")
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_multiple_version_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .version(onionoo::parameters::helpers::version_list("0.4.7,0.4.8"))
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_version_range_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .version(onionoo::parameters::helpers::version_list("0.4.7..0.4.8"))
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_contact_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .contact("tor")
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_family_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .family("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_host_name_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .host_name(".torproject.org")
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_recommended_version_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .recommended_version(BoolValue::True)
        .limit(5);
    let result = endpoints::details(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_as_name_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .as_name("Hurricane Electric")
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}

#[tokio::test]
async fn test_lookup_filter() {
    let client = Client::new();
    let params = QueryParameters::new()
        .lookup("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
        .limit(5);
    let result = endpoints::summary(&client, params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.version.is_empty());
}