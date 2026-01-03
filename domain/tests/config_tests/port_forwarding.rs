// PortForwardingConfig tests
//
// Tests for PortForwardingConfig creation and validation.

use roro_domain::{DomainError, PortForwardingConfig, PortValue};

#[test]
fn test_port_forward_config_creation() {
    let config = PortForwardingConfig {
        local_port: "3333".to_string(),
        name: "api-service".to_string(),
        port: PortValue::Numeric(5555),
        kind: "service".to_string(),
    };

    assert_eq!(config.local_port, "3333");
    assert_eq!(config.name, "api-service");
    assert_eq!(config.port, PortValue::Numeric(5555));
    assert_eq!(config.kind, "service");
}

#[test]
fn test_port_forward_config_validation_success() {
    let config = PortForwardingConfig {
        local_port: "3333".to_string(),
        name: "api-service".to_string(),
        port: PortValue::Numeric(5555),
        kind: "service".to_string(),
    };

    assert!(config.validate().is_ok());
}

#[test]
fn test_port_forward_config_validation_empty_localport() {
    let config = PortForwardingConfig {
        local_port: "".to_string(),
        name: "api-service".to_string(),
        port: PortValue::Numeric(5555),
        kind: "service".to_string(),
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::PortForwardingValidation(msg)) = result {
        assert!(msg.contains("localport cannot be empty"));
    } else {
        panic!("Expected PortForwardingValidation error");
    }
}

#[test]
fn test_port_forward_config_validation_zero_localport() {
    let config = PortForwardingConfig {
        local_port: "0".to_string(),
        name: "api-service".to_string(),
        port: PortValue::Numeric(5555),
        kind: "service".to_string(),
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::PortForwardingValidation(msg)) = result {
        assert!(msg.contains("localport cannot be 0"));
    } else {
        panic!("Expected PortForwardingValidation error");
    }
}

#[test]
fn test_port_forward_config_validation_invalid_localport() {
    let config = PortForwardingConfig {
        local_port: "invalid".to_string(),
        name: "api-service".to_string(),
        port: PortValue::Numeric(5555),
        kind: "service".to_string(),
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::PortForwardingValidation(msg)) = result {
        assert!(msg.contains("must be a valid port number"));
    } else {
        panic!("Expected PortForwardingValidation error");
    }
}

#[test]
fn test_port_forward_config_validation_empty_name() {
    let config = PortForwardingConfig {
        local_port: "3333".to_string(),
        name: "".to_string(),
        port: PortValue::Numeric(5555),
        kind: "service".to_string(),
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::PortForwardingValidation(msg)) = result {
        assert!(msg.contains("name cannot be empty"));
    } else {
        panic!("Expected PortForwardingValidation error");
    }
}

#[test]
fn test_port_forward_config_validation_empty_kind() {
    let config = PortForwardingConfig {
        local_port: "3333".to_string(),
        name: "api-service".to_string(),
        port: PortValue::Numeric(5555),
        kind: "".to_string(),
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::PortForwardingValidation(msg)) = result {
        assert!(msg.contains("kind cannot be empty"));
    } else {
        panic!("Expected PortForwardingValidation error");
    }
}

#[test]
fn test_port_forward_config_deserialize_from_json() {
    let json = r#"{
        "localport": "3333",
        "name": "api-service",
        "port": 5555,
        "kind": "service"
    }"#;

    let config: PortForwardingConfig =
        serde_json::from_str(json).expect("deserialization should succeed");
    assert_eq!(config.local_port, "3333");
    assert_eq!(config.name, "api-service");
    assert_eq!(config.port, PortValue::Numeric(5555));
    assert_eq!(config.kind, "service");
}

#[test]
fn test_port_forward_config_deserialize_named_port() {
    let json = r#"{
        "localport": "2222",
        "name": "metrics-service",
        "port": "prometheus",
        "kind": "service"
    }"#;

    let config: PortForwardingConfig =
        serde_json::from_str(json).expect("deserialization should succeed");
    assert_eq!(config.local_port, "2222");
    assert_eq!(config.name, "metrics-service");
    assert_eq!(config.port, PortValue::Named("prometheus".to_string()));
    assert_eq!(config.kind, "service");
}

