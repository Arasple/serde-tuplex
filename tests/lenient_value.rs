use serde_tuplex::__private::LenientValue;

#[test]
fn test_lenient_value_from_string() {
    let json = r#""42""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: u64 = value.parse().unwrap();
    assert_eq!(parsed, 42);
}

#[test]
fn test_lenient_value_from_i64() {
    let json = r#"-123"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: i64 = value.parse().unwrap();
    assert_eq!(parsed, -123);
}

#[test]
fn test_lenient_value_from_u64() {
    let json = r#"9999"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: u64 = value.parse().unwrap();
    assert_eq!(parsed, 9999);
}

#[test]
fn test_lenient_value_from_f64() {
    let json = r#"3.25"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: f64 = value.parse().unwrap();
    assert_eq!(parsed, 3.25);
}

#[test]
fn test_lenient_value_string_to_u64() {
    let json = r#""9876""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: u64 = value.parse().unwrap();
    assert_eq!(parsed, 9876);
}

#[test]
fn test_lenient_value_string_to_f64() {
    let json = r#""2.5""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: f64 = value.parse().unwrap();
    assert_eq!(parsed, 2.5);
}

#[test]
fn test_lenient_value_i64_to_u32() {
    let json = r#"100"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: u32 = value.parse().unwrap();
    assert_eq!(parsed, 100);
}

#[test]
fn test_lenient_value_f64_to_u64() {
    let json = r#"42.0"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: u64 = value.parse().unwrap();
    assert_eq!(parsed, 42);
}

#[test]
fn test_lenient_value_string_invalid() {
    let json = r#""not_a_number""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let result: Result<u64, String> = value.parse();
    assert!(result.is_err());
}

#[test]
fn test_lenient_value_string_to_i8() {
    let json = r#""-128""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: i8 = value.parse().unwrap();
    assert_eq!(parsed, -128);
}

#[test]
fn test_lenient_value_u64_to_u8() {
    let json = r#"255"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: u8 = value.parse().unwrap();
    assert_eq!(parsed, 255);
}

#[test]
fn test_lenient_value_f64_to_f32() {
    let json = r#"1.5"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: f32 = value.parse().unwrap();
    assert_eq!(parsed, 1.5);
}

#[test]
fn test_lenient_value_negative_string_to_i64() {
    let json = r#""-999""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: i64 = value.parse().unwrap();
    assert_eq!(parsed, -999);
}

#[test]
fn test_lenient_value_zero_string() {
    let json = r#""0""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: u32 = value.parse().unwrap();
    assert_eq!(parsed, 0);
}

#[test]
fn test_lenient_value_zero_number() {
    let json = r#"0"#;
    let value: LenientValue = serde_json::from_str(json).unwrap();
    let parsed: i32 = value.parse().unwrap();
    assert_eq!(parsed, 0);
}

#[test]
fn test_lenient_value_all_numeric_types() {
    let json = r#""123""#;
    let value: LenientValue = serde_json::from_str(json).unwrap();

    assert_eq!(value.parse::<u8>().unwrap(), 123u8);
    assert_eq!(value.parse::<u16>().unwrap(), 123u16);
    assert_eq!(value.parse::<u32>().unwrap(), 123u32);
    assert_eq!(value.parse::<u64>().unwrap(), 123u64);
    assert_eq!(value.parse::<i8>().unwrap(), 123i8);
    assert_eq!(value.parse::<i16>().unwrap(), 123i16);
    assert_eq!(value.parse::<i32>().unwrap(), 123i32);
    assert_eq!(value.parse::<i64>().unwrap(), 123i64);
}
