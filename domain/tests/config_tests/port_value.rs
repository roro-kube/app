// PortValue tests
//
// Tests for PortValue serialization and deserialization.

use roro_domain::PortValue;

#[test]
fn test_port_value_numeric() {
    let port = PortValue::Numeric(8080);
    assert_eq!(port, PortValue::Numeric(8080));
}

#[test]
fn test_port_value_named() {
    let port = PortValue::Named("prometheus".to_string());
    assert_eq!(port, PortValue::Named("prometheus".to_string()));
}

#[test]
fn test_port_value_serialize_numeric() {
    let port = PortValue::Numeric(5555);
    let serialized = serde_json::to_string(&port).expect("serialization should succeed");
    assert_eq!(serialized, "5555");
}

#[test]
fn test_port_value_serialize_named() {
    let port = PortValue::Named("prometheus".to_string());
    let serialized = serde_json::to_string(&port).expect("serialization should succeed");
    assert_eq!(serialized, "\"prometheus\"");
}

#[test]
fn test_port_value_deserialize_numeric() {
    let json = "5555";
    let port: PortValue = serde_json::from_str(json).expect("deserialization should succeed");
    assert_eq!(port, PortValue::Numeric(5555));
}

#[test]
fn test_port_value_deserialize_named() {
    let json = "\"prometheus\"";
    let port: PortValue = serde_json::from_str(json).expect("deserialization should succeed");
    assert_eq!(port, PortValue::Named("prometheus".to_string()));
}

#[test]
fn test_port_value_deserialize_invalid_type() {
    let json = "true";
    let result: Result<PortValue, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_port_value_deserialize_overflow() {
    let json = "70000"; // Exceeds u16::MAX
    let result: Result<PortValue, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

