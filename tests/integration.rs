use serde::Deserialize;
use serde_tuplex::{Lenient, Tuple, TupleLenient};

#[derive(Debug, Lenient, PartialEq)]
struct ApiUser {
    id: u64,
    created_at: u64,
    balance: f64,
    points: u32,
    age: Option<u8>,
    username: String,
    email: String,
    verified: bool,
}

#[test]
fn test_api_response_all_numbers() {
    let json = r#"{
        "id": 12345,
        "created_at": 1609459200,
        "balance": 1234.56,
        "points": 9999,
        "age": 28,
        "username": "alice",
        "email": "alice@example.com",
        "verified": true
    }"#;

    let user: ApiUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, 12345);
    assert_eq!(user.created_at, 1609459200);
    assert_eq!(user.balance, 1234.56);
    assert_eq!(user.points, 9999);
    assert_eq!(user.age, Some(28));
    assert_eq!(user.username, "alice");
    assert_eq!(user.email, "alice@example.com");
    assert!(user.verified);
}

#[test]
fn test_api_response_all_strings() {
    let json = r#"{
        "id": "12345",
        "created_at": "1609459200",
        "balance": "1234.56",
        "points": "9999",
        "age": "28",
        "username": "bob",
        "email": "bob@example.com",
        "verified": true
    }"#;

    let user: ApiUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, 12345);
    assert_eq!(user.created_at, 1609459200);
    assert_eq!(user.balance, 1234.56);
    assert_eq!(user.points, 9999);
    assert_eq!(user.age, Some(28));
    assert_eq!(user.username, "bob");
}

#[test]
fn test_api_response_mixed_types() {
    let json = r#"{
        "id": "12345",
        "created_at": 1609459200,
        "balance": "1234.56",
        "points": 9999,
        "age": null,
        "username": "charlie",
        "email": "charlie@example.com",
        "verified": false
    }"#;

    let user: ApiUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, 12345);
    assert_eq!(user.age, None);
}

// Real-world use case: Time-series data

#[derive(Debug, TupleLenient, PartialEq)]
struct Metric {
    timestamp: u64,
    cpu_usage: f32,
    memory_mb: u32,
    disk_io: f32,
}

#[test]
fn test_metrics_array_numbers() {
    let json = r#"[
        [1609459200, 45.5, 2048, 123.4],
        [1609459260, 52.1, 2100, 145.2],
        [1609459320, 38.9, 1980, 98.7]
    ]"#;

    let metrics: Vec<Metric> = serde_json::from_str(json).unwrap();
    assert_eq!(metrics.len(), 3);
    assert_eq!(metrics[0].timestamp, 1609459200);
    assert_eq!(metrics[0].cpu_usage, 45.5);
    assert_eq!(metrics[1].memory_mb, 2100);
}

#[test]
fn test_metrics_array_strings() {
    let json = r#"[
        ["1609459200", "45.5", "2048", "123.4"],
        ["1609459260", "52.1", "2100", "145.2"]
    ]"#;

    let metrics: Vec<Metric> = serde_json::from_str(json).unwrap();
    assert_eq!(metrics.len(), 2);
    assert_eq!(metrics[0].timestamp, 1609459200);
    assert_eq!(metrics[0].cpu_usage, 45.5);
}

#[test]
fn test_metrics_array_mixed() {
    let json = r#"[
        ["1609459200", 45.5, "2048", 123.4],
        [1609459260, "52.1", 2100, "145.2"]
    ]"#;

    let metrics: Vec<Metric> = serde_json::from_str(json).unwrap();
    assert_eq!(metrics.len(), 2);
    assert_eq!(metrics[0].cpu_usage, 45.5);
    assert_eq!(metrics[1].memory_mb, 2100);
}

// Real-world use case: CSV-like data from external system

#[derive(Debug, TupleLenient, PartialEq)]
struct Transaction {
    id: u64,
    amount: f64,
    fee: f64,
    timestamp: u64,
}

#[test]
fn test_csv_style_batch() {
    let json = r#"[
        ["1001", "99.99", "2.50", "1609459200"],
        ["1002", "150.00", "3.75", "1609459260"],
        ["1003", "75.25", "1.88", "1609459320"],
        ["1004", "200.50", "5.01", "1609459380"]
    ]"#;

    let transactions: Vec<Transaction> = serde_json::from_str(json).unwrap();
    assert_eq!(transactions.len(), 4);
    assert_eq!(transactions[0].id, 1001);
    assert_eq!(transactions[0].amount, 99.99);
    assert_eq!(transactions[0].fee, 2.50);
    assert_eq!(transactions[3].id, 1004);
}

// Real-world use case: Nested structures

#[derive(Debug, Lenient, PartialEq)]
struct Order {
    order_id: u64,
    total: f64,
    customer_id: u64,
    status: String,
}

#[derive(Debug, PartialEq, Deserialize)]
struct OrderBatch {
    batch_id: u32,
    orders: Vec<Order>,
}

#[test]
fn test_nested_with_lenient() {
    let json = r#"{
        "batch_id": 500,
        "orders": [
            {"order_id": "1001", "total": "299.99", "customer_id": "5001", "status": "pending"},
            {"order_id": 1002, "total": 149.50, "customer_id": 5002, "status": "shipped"},
            {"order_id": "1003", "total": 89.99, "customer_id": "5003", "status": "delivered"}
        ]
    }"#;

    let batch: OrderBatch = serde_json::from_str(json).unwrap();
    assert_eq!(batch.batch_id, 500);
    assert_eq!(batch.orders.len(), 3);
    assert_eq!(batch.orders[0].order_id, 1001);
    assert_eq!(batch.orders[0].total, 299.99);
    assert_eq!(batch.orders[1].order_id, 1002);
    assert_eq!(batch.orders[2].status, "delivered");
}

// Real-world use case: Configuration file

#[derive(Debug, Lenient, PartialEq)]
struct ServerConfig {
    #[serde_tuplex(skip)]
    port: u16,
    timeout_ms: u64,
    max_connections: u32,
    rate_limit: u32,
    buffer_size_kb: u32,
    debug: bool,
}

#[test]
fn test_config_file_parsing() {
    let json = r#"{
        "port": 8080,
        "timeout_ms": "30000",
        "max_connections": "1000",
        "rate_limit": "100",
        "buffer_size_kb": "64",
        "debug": false
    }"#;

    let config: ServerConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.port, 8080);
    assert_eq!(config.timeout_ms, 30000);
    assert_eq!(config.max_connections, 1000);
    assert_eq!(config.rate_limit, 100);
    assert_eq!(config.buffer_size_kb, 64);
    assert!(!config.debug);
}

// Real-world use case: Sensor data from IoT devices

#[derive(Debug, TupleLenient, PartialEq)]
struct SensorReading {
    device_id: u32,
    timestamp: u64,
    temperature: f32,
    humidity: f32,
    battery_pct: u8,
}

#[test]
fn test_iot_sensor_stream() {
    let json = r#"[
        ["101", "1609459200", "22.5", "45.0", "95"],
        [102, 1609459260, 23.1, 47.5, 94],
        ["103", 1609459320, "21.8", "44.2", "93"],
        [104, "1609459380", 22.9, "46.8", 92]
    ]"#;

    let readings: Vec<SensorReading> = serde_json::from_str(json).unwrap();
    assert_eq!(readings.len(), 4);
    assert_eq!(readings[0].device_id, 101);
    assert_eq!(readings[0].temperature, 22.5);
    assert_eq!(readings[3].battery_pct, 92);
}

// Real-world use case: Financial data

#[derive(Debug, Lenient, PartialEq)]
struct StockPrice {
    symbol: String,
    price: f64,
    volume: u64,
    market_cap: u64,
    pe_ratio: Option<f32>,
}

#[test]
fn test_stock_data_feed() {
    let json = r#"[
        {"symbol": "AAPL", "price": "150.25", "volume": "89234567", "market_cap": "2500000000000", "pe_ratio": "28.5"},
        {"symbol": "GOOGL", "price": 2800.50, "volume": 1234567, "market_cap": 1850000000000, "pe_ratio": 25.3},
        {"symbol": "TSLA", "price": "750.00", "volume": 45678901, "market_cap": "750000000000", "pe_ratio": null}
    ]"#;

    let stocks: Vec<StockPrice> = serde_json::from_str(json).unwrap();
    assert_eq!(stocks.len(), 3);
    assert_eq!(stocks[0].symbol, "AAPL");
    assert_eq!(stocks[0].price, 150.25);
    assert_eq!(stocks[0].volume, 89234567);
    assert_eq!(stocks[2].pe_ratio, None);
}

// Real-world use case: Geographic coordinates

#[derive(Debug, Tuple, PartialEq)]
struct Coordinate {
    latitude: f64,
    longitude: f64,
}

#[test]
fn test_coordinate_array() {
    let json = r#"[
        [37.7749, -122.4194],
        [40.7128, -74.0060],
        [51.5074, -0.1278]
    ]"#;

    let coords: Vec<Coordinate> = serde_json::from_str(json).unwrap();
    assert_eq!(coords.len(), 3);
    assert_eq!(coords[0].latitude, 37.7749);
    assert_eq!(coords[0].longitude, -122.4194);
}

#[test]
fn test_coordinate_roundtrip() {
    let coords = vec![
        Coordinate {
            latitude: 37.7749,
            longitude: -122.4194,
        },
        Coordinate {
            latitude: 40.7128,
            longitude: -74.0060,
        },
    ];

    let json = serde_json::to_string(&coords).unwrap();
    let deserialized: Vec<Coordinate> = serde_json::from_str(&json).unwrap();
    assert_eq!(coords, deserialized);
}

// Real-world use case: Database export with mixed types

#[derive(Debug, Lenient, PartialEq)]
struct DatabaseRecord {
    row_id: u64,
    created_timestamp: u64,
    updated_timestamp: u64,
    value: f64,
    count: u32,
    active: bool,
    category: String,
}

#[test]
fn test_database_export() {
    let json = r#"[
        {"row_id": "1", "created_timestamp": "1609459200", "updated_timestamp": "1609459200", "value": "100.0", "count": "5", "active": true, "category": "A"},
        {"row_id": 2, "created_timestamp": 1609459260, "updated_timestamp": 1609459300, "value": 200.5, "count": 10, "active": false, "category": "B"},
        {"row_id": "3", "created_timestamp": "1609459320", "updated_timestamp": 1609459400, "value": "150.75", "count": 7, "active": true, "category": "C"}
    ]"#;

    let records: Vec<DatabaseRecord> = serde_json::from_str(json).unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].row_id, 1);
    assert_eq!(records[1].value, 200.5);
    assert!(records[2].active);
}
