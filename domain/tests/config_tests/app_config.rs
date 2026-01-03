// AppConfig tests
//
// Tests for AppConfig creation, validation, and serialization.

use roro_domain::{AppConfig, DomainError, PortForwardingConfig, PortValue};

#[test]
fn test_app_config_creation() {
    let config = AppConfig {
        name: "API".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "./infrastructure/local/k8s".to_string(),
        port_forwarding: vec![],
    };

    assert_eq!(config.name, "API");
    assert_eq!(config.description, "BFF Client Portal API");
    assert_eq!(config.manifests_path, "./infrastructure/local/k8s");
    assert!(config.port_forwarding.is_empty());
}

#[test]
fn test_app_config_validation_success() {
    let config = AppConfig {
        name: "API".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "./infrastructure/local/k8s".to_string(),
        port_forwarding: vec![],
    };

    assert!(config.validate().is_ok());
}

#[test]
fn test_app_config_validation_empty_name() {
    let config = AppConfig {
        name: "".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "./infrastructure/local/k8s".to_string(),
        port_forwarding: vec![],
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::AppConfigValidation(msg)) = result {
        assert!(msg.contains("name cannot be empty"));
    } else {
        panic!("Expected AppConfigValidation error");
    }
}

#[test]
fn test_app_config_validation_empty_manifests_path() {
    let config = AppConfig {
        name: "API".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "".to_string(),
        port_forwarding: vec![],
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::AppConfigValidation(msg)) = result {
        assert!(msg.contains("manifestsPath cannot be empty"));
    } else {
        panic!("Expected AppConfigValidation error");
    }
}

#[test]
fn test_app_config_validation_with_port_forward() {
    let config = AppConfig {
        name: "API".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "./infrastructure/local/k8s".to_string(),
        port_forwarding: vec![
            PortForwardingConfig {
                local_port: "3333".to_string(),
                name: "api-service".to_string(),
                port: PortValue::Numeric(5555),
                kind: "service".to_string(),
            },
            PortForwardingConfig {
                local_port: "2222".to_string(),
                name: "metrics-service".to_string(),
                port: PortValue::Named("prometheus".to_string()),
                kind: "service".to_string(),
            },
        ],
    };

    assert!(config.validate().is_ok());
}

#[test]
fn test_app_config_validation_invalid_port_forward() {
    let config = AppConfig {
        name: "API".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "./infrastructure/local/k8s".to_string(),
        port_forwarding: vec![PortForwardingConfig {
            local_port: "".to_string(), // Invalid
            name: "api-service".to_string(),
            port: PortValue::Numeric(5555),
            kind: "service".to_string(),
        }],
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(DomainError::AppConfigValidation(msg)) = result {
        assert!(msg.contains("portForwarding[0]"));
        assert!(msg.contains("localport cannot be empty"));
    } else {
        panic!("Expected AppConfigValidation error");
    }
}

#[test]
fn test_app_config_deserialize_from_json_example() {
    let json = r#"[
        {
            "name": "API",
            "description": "BFF Client Portal API",
            "manifestsPath": "./infrastructure/local/k8s",
            "portForwarding": [
                {
                    "localport": "3333",
                    "name": "api-service",
                    "port": 5555,
                    "kind": "service"
                },
                {
                    "localport": "2222",
                    "name": "metrics-service",
                    "port": "prometheus",
                    "kind": "service"
                }
            ]
        }
    ]"#;

    let configs: Vec<AppConfig> =
        serde_json::from_str(json).expect("deserialization should succeed");
    assert_eq!(configs.len(), 1);

    let config = &configs[0];
    assert_eq!(config.name, "API");
    assert_eq!(config.description, "BFF Client Portal API");
    assert_eq!(config.manifests_path, "./infrastructure/local/k8s");
    assert_eq!(config.port_forwarding.len(), 2);

    assert_eq!(config.port_forwarding[0].local_port, "3333");
    assert_eq!(config.port_forwarding[0].name, "api-service");
    assert_eq!(config.port_forwarding[0].port, PortValue::Numeric(5555));
    assert_eq!(config.port_forwarding[0].kind, "service");

    assert_eq!(config.port_forwarding[1].local_port, "2222");
    assert_eq!(config.port_forwarding[1].name, "metrics-service");
    assert_eq!(
        config.port_forwarding[1].port,
        PortValue::Named("prometheus".to_string())
    );
    assert_eq!(config.port_forwarding[1].kind, "service");
}

#[test]
fn test_app_config_deserialize_without_port_forward() {
    let json = r#"{
        "name": "API",
        "description": "BFF Client Portal API",
        "manifestsPath": "./infrastructure/local/k8s"
    }"#;

    let config: AppConfig = serde_json::from_str(json).expect("deserialization should succeed");
    assert_eq!(config.name, "API");
    assert_eq!(config.description, "BFF Client Portal API");
    assert_eq!(config.manifests_path, "./infrastructure/local/k8s");
    assert!(config.port_forwarding.is_empty());
}

#[test]
fn test_app_config_serialize() {
    let config = AppConfig {
        name: "API".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "./infrastructure/local/k8s".to_string(),
        port_forwarding: vec![PortForwardingConfig {
            local_port: "3333".to_string(),
            name: "api-service".to_string(),
            port: PortValue::Numeric(5555),
            kind: "service".to_string(),
        }],
    };

    let serialized = serde_json::to_string(&config).expect("serialization should succeed");
    assert!(serialized.contains("\"name\":\"API\""));
    assert!(serialized.contains("\"manifestsPath\":\"./infrastructure/local/k8s\""));
    assert!(serialized.contains("\"portForwarding\""));
    assert!(serialized.contains("\"localport\":\"3333\""));
}

#[test]
fn test_app_config_round_trip() {
    let original = AppConfig {
        name: "API".to_string(),
        description: "BFF Client Portal API".to_string(),
        manifests_path: "./infrastructure/local/k8s".to_string(),
        port_forwarding: vec![
            PortForwardingConfig {
                local_port: "3333".to_string(),
                name: "api-service".to_string(),
                port: PortValue::Numeric(5555),
                kind: "service".to_string(),
            },
            PortForwardingConfig {
                local_port: "2222".to_string(),
                name: "metrics-service".to_string(),
                port: PortValue::Named("prometheus".to_string()),
                kind: "service".to_string(),
            },
        ],
    };

    let serialized = serde_json::to_string(&original).expect("serialization should succeed");
    let deserialized: AppConfig =
        serde_json::from_str(&serialized).expect("deserialization should succeed");

    assert_eq!(original.name, deserialized.name);
    assert_eq!(original.description, deserialized.description);
    assert_eq!(original.manifests_path, deserialized.manifests_path);
    assert_eq!(
        original.port_forwarding.len(),
        deserialized.port_forwarding.len()
    );

    for (orig, deser) in original
        .port_forwarding
        .iter()
        .zip(deserialized.port_forwarding.iter())
    {
        assert_eq!(orig.local_port, deser.local_port);
        assert_eq!(orig.name, deser.name);
        assert_eq!(orig.port, deser.port);
        assert_eq!(orig.kind, deser.kind);
    }
}

