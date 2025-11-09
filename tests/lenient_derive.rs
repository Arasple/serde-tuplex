use serde_tuplex::Lenient;

#[derive(Debug, Lenient, PartialEq)]
struct User {
    id: u64,
    balance: f64,
    age: Option<u32>,
    name: String,
}

#[derive(Debug, Lenient, PartialEq)]
struct Config {
    timeout: u64,
    #[serde_tuplex(skip)]
    port: u16,
    max_connections: u32,
}

#[derive(Debug, Lenient, PartialEq)]
struct AllNumericTypes {
    u8_val: u8,
    u16_val: u16,
    u32_val: u32,
    u64_val: u64,
    i8_val: i8,
    i16_val: i16,
    i32_val: i32,
    i64_val: i64,
    f32_val: f32,
    f64_val: f64,
}

// Basic lenient deserialization

#[test]
fn test_lenient_with_numeric_fields() {
    let json = r#"{"id": 42, "balance": 99.99, "age": 30, "name": "Alice"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(
        user,
        User {
            id: 42,
            balance: 99.99,
            age: Some(30),
            name: "Alice".to_string(),
        }
    );
}

#[test]
fn test_lenient_with_string_numbers() {
    let json = r#"{"id": "100", "balance": "250.50", "age": "25", "name": "Bob"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(
        user,
        User {
            id: 100,
            balance: 250.50,
            age: Some(25),
            name: "Bob".to_string(),
        }
    );
}

#[test]
fn test_lenient_mixed_numbers_and_strings() {
    let json = r#"{"id": "123", "balance": 99.99, "age": 40, "name": "Charlie"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(
        user,
        User {
            id: 123,
            balance: 99.99,
            age: Some(40),
            name: "Charlie".to_string(),
        }
    );
}

#[test]
fn test_lenient_option_with_null() {
    let json = r#"{"id": 1, "balance": 0.0, "age": null, "name": "Dave"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(
        user,
        User {
            id: 1,
            balance: 0.0,
            age: None,
            name: "Dave".to_string(),
        }
    );
}

#[test]
fn test_lenient_option_missing_field() {
    let json = r#"{"id": 2, "balance": 50.0, "name": "Eve"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(
        user,
        User {
            id: 2,
            balance: 50.0,
            age: None,
            name: "Eve".to_string(),
        }
    );
}

#[test]
fn test_lenient_option_with_string() {
    let json = r#"{"id": 3, "balance": 75.0, "age": "35", "name": "Frank"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(
        user,
        User {
            id: 3,
            balance: 75.0,
            age: Some(35),
            name: "Frank".to_string(),
        }
    );
}

// Field-level control with #[serde_tuplex(skip)]

#[test]
fn test_lenient_skip_attribute_strict_only() {
    let json = r#"{"timeout": "1000", "port": 8080, "max_connections": "100"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(
        config,
        Config {
            timeout: 1000,
            port: 8080,
            max_connections: 100,
        }
    );
}

#[test]
fn test_lenient_skip_attribute_rejects_string_port() {
    let json = r#"{"timeout": 1000, "port": "8080", "max_connections": 100}"#;
    let result: Result<Config, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// All numeric types

#[test]
fn test_lenient_all_numeric_types_as_numbers() {
    let json = r#"{
        "u8_val": 255,
        "u16_val": 65535,
        "u32_val": 4294967295,
        "u64_val": 18446744073709551615,
        "i8_val": -128,
        "i16_val": -32768,
        "i32_val": -2147483648,
        "i64_val": -9223372036854775808,
        "f32_val": 3.25,
        "f64_val": 2.5
    }"#;
    let data: AllNumericTypes = serde_json::from_str(json).unwrap();
    assert_eq!(data.u8_val, 255);
    assert_eq!(data.u16_val, 65535);
    assert_eq!(data.u32_val, 4294967295);
    assert_eq!(data.u64_val, 18446744073709551615);
    assert_eq!(data.i8_val, -128);
    assert_eq!(data.i16_val, -32768);
    assert_eq!(data.i32_val, -2147483648);
    assert_eq!(data.i64_val, -9223372036854775808);
    assert_eq!(data.f32_val, 3.25);
    assert_eq!(data.f64_val, 2.5);
}

#[test]
fn test_lenient_all_numeric_types_as_strings() {
    let json = r#"{
        "u8_val": "255",
        "u16_val": "65535",
        "u32_val": "4294967295",
        "u64_val": "18446744073709551615",
        "i8_val": "-128",
        "i16_val": "-32768",
        "i32_val": "-2147483648",
        "i64_val": "-9223372036854775808",
        "f32_val": "3.25",
        "f64_val": "2.5"
    }"#;
    let data: AllNumericTypes = serde_json::from_str(json).unwrap();
    assert_eq!(data.u8_val, 255);
    assert_eq!(data.u16_val, 65535);
    assert_eq!(data.u32_val, 4294967295);
    assert_eq!(data.u64_val, 18446744073709551615);
    assert_eq!(data.i8_val, -128);
    assert_eq!(data.i16_val, -32768);
    assert_eq!(data.i32_val, -2147483648);
    assert_eq!(data.i64_val, -9223372036854775808);
    assert_eq!(data.f32_val, 3.25);
    assert_eq!(data.f64_val, 2.5);
}

// Error cases

#[test]
fn test_lenient_invalid_string_for_numeric() {
    let json = r#"{"id": "not_a_number", "balance": 50.0, "age": 30, "name": "Test"}"#;
    let result: Result<User, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_lenient_missing_required_field() {
    let json = r#"{"id": 1, "balance": 50.0, "age": 30}"#;
    let result: Result<User, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_lenient_wrong_type_for_string_field() {
    let json = r#"{"id": 1, "balance": 50.0, "age": 30, "name": 12345}"#;
    let result: Result<User, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_lenient_option_invalid_string() {
    let json = r#"{"id": 1, "balance": 50.0, "age": "invalid", "name": "Test"}"#;
    let result: Result<User, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// Edge cases

#[test]
fn test_lenient_zero_values() {
    let json = r#"{"id": "0", "balance": "0.0", "age": "0", "name": ""}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(
        user,
        User {
            id: 0,
            balance: 0.0,
            age: Some(0),
            name: String::new(),
        }
    );
}

#[test]
fn test_lenient_large_numbers() {
    let json = r#"{"id": "18446744073709551615", "balance": "999999999.99", "age": "4294967295", "name": "Max"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, u64::MAX);
    assert_eq!(user.balance, 999999999.99);
}

#[test]
fn test_lenient_negative_floats() {
    #[derive(Debug, Lenient, PartialEq)]
    struct SignedFloat {
        value: f64,
    }

    let json = r#"{"value": "-123.456"}"#;
    let data: SignedFloat = serde_json::from_str(json).unwrap();
    assert_eq!(data.value, -123.456);
}

#[test]
fn test_lenient_scientific_notation() {
    #[derive(Debug, Lenient, PartialEq)]
    struct Scientific {
        value: f64,
    }

    let json = r#"{"value": "1.23e10"}"#;
    let data: Scientific = serde_json::from_str(json).unwrap();
    assert_eq!(data.value, 1.23e10);
}

#[test]
fn test_lenient_preserves_non_numeric_fields() {
    let json = r#"{"id": "42", "balance": "99.99", "age": "30", "name": "Test User 🚀"}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Test User 🚀");
}
