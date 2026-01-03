// Port availability tests
//
// Tests for port conflict detection and finding available ports

use super::create_test_manager;
use roro_core::errors::CoreError;

#[tokio::test]
async fn test_port_conflict_detection() {
    let manager = create_test_manager().await;

    let listener =
        std::net::TcpListener::bind("127.0.0.1:9100").expect("Port should be available for test");

    let result = manager.check_port_available(9100);
    assert!(result.is_err());
    match result {
        Err(CoreError::PortConflict(9100)) => {}
        _ => panic!("Expected PortConflict error"),
    }

    drop(listener);

    let result = manager.check_port_available(9100);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_find_available_port() {
    let manager = create_test_manager().await;

    let listener =
        std::net::TcpListener::bind("127.0.0.1:9200").expect("Port should be available for test");

    let result = manager.find_available_port(9200);
    assert!(result.is_ok());
    let port = result.expect("Port should be found");
    assert!(port > 9200);

    drop(listener);
}

