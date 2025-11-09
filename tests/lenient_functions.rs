use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct TestLenient {
    #[serde(deserialize_with = "serde_tuplex::lenient")]
    value: u64,
    normal: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestLenientOption {
    #[serde(deserialize_with = "serde_tuplex::lenient_option", default)]
    value: Option<u32>,
    other: String,
}

#[test]
fn test_lenient_with_number() {
    let json = r#"{"value": 42, "normal": "test"}"#;
    let result: TestLenient = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 42);
    assert_eq!(result.normal, "test");
}

#[test]
fn test_lenient_with_string() {
    let json = r#"{"value": "99", "normal": "test"}"#;
    let result: TestLenient = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 99);
    assert_eq!(result.normal, "test");
}

#[test]
fn test_lenient_invalid_string() {
    let json = r#"{"value": "not_a_number", "normal": "test"}"#;
    let result: Result<TestLenient, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_lenient_option_with_number() {
    let json = r#"{"value": 100, "other": "data"}"#;
    let result: TestLenientOption = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(100));
    assert_eq!(result.other, "data");
}

#[test]
fn test_lenient_option_with_string() {
    let json = r#"{"value": "200", "other": "data"}"#;
    let result: TestLenientOption = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(200));
    assert_eq!(result.other, "data");
}

#[test]
fn test_lenient_option_with_null() {
    let json = r#"{"value": null, "other": "data"}"#;
    let result: TestLenientOption = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, None);
    assert_eq!(result.other, "data");
}

#[test]
fn test_lenient_option_missing_field() {
    let json = r#"{"other": "data"}"#;
    let result: TestLenientOption = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, None);
    assert_eq!(result.other, "data");
}

#[test]
fn test_lenient_option_invalid_string() {
    let json = r#"{"value": "invalid", "other": "data"}"#;
    let result: Result<TestLenientOption, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_lenient_with_float_string() {
    #[derive(Debug, Deserialize)]
    struct FloatTest {
        #[serde(deserialize_with = "serde_tuplex::lenient")]
        value: f64,
    }

    let json = r#"{"value": "3.25"}"#;
    let result: FloatTest = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 3.25);
}

#[test]
fn test_lenient_with_float_number() {
    #[derive(Debug, Deserialize)]
    struct FloatTest {
        #[serde(deserialize_with = "serde_tuplex::lenient")]
        value: f64,
    }

    let json = r#"{"value": 2.5}"#;
    let result: FloatTest = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 2.5);
}

#[test]
fn test_lenient_negative_number() {
    #[derive(Debug, Deserialize)]
    struct SignedTest {
        #[serde(deserialize_with = "serde_tuplex::lenient")]
        value: i64,
    }

    let json = r#"{"value": -9876}"#;
    let result: SignedTest = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, -9876);
}

#[test]
fn test_lenient_negative_string() {
    #[derive(Debug, Deserialize)]
    struct SignedTest {
        #[serde(deserialize_with = "serde_tuplex::lenient")]
        value: i32,
    }

    let json = r#"{"value": "-1234"}"#;
    let result: SignedTest = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, -1234);
}

#[test]
fn test_lenient_zero_values() {
    let json = r#"{"value": "0", "normal": "test"}"#;
    let result: TestLenient = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 0);
}

#[test]
fn test_lenient_option_zero_string() {
    let json = r#"{"value": "0", "other": "test"}"#;
    let result: TestLenientOption = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, Some(0));
}

#[test]
fn test_lenient_large_numbers() {
    #[derive(Debug, Deserialize)]
    struct LargeTest {
        #[serde(deserialize_with = "serde_tuplex::lenient")]
        value: u64,
    }

    let json = r#"{"value": "18446744073709551615"}"#;
    let result: LargeTest = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, u64::MAX);
}
