use serde_tuplex::TupleLenient;

#[derive(Debug, TupleLenient, PartialEq)]
struct Measurement {
    timestamp: u64,
    value: f64,
}

#[derive(Debug, TupleLenient, PartialEq)]
struct SensorData {
    id: u32,
    temperature: f32,
    humidity: f32,
    active: bool,
}

#[derive(Debug, TupleLenient, PartialEq)]
struct Record {
    count: u64,
    score: f64,
    label: String,
    enabled: Option<bool>,
}

#[derive(Debug, TupleLenient, PartialEq)]
struct WithSkip {
    lenient_field: u64,
    #[serde_tuplex(skip)]
    strict_field: u32,
}

// Serialization tests

#[test]
fn test_tuple_lenient_serialize_simple() {
    let measurement = Measurement {
        timestamp: 1234567890,
        value: 23.5,
    };
    let json = serde_json::to_string(&measurement).unwrap();
    assert_eq!(json, "[1234567890,23.5]");
}

#[test]
fn test_tuple_lenient_serialize_sensor() {
    let sensor = SensorData {
        id: 42,
        temperature: 25.5,
        humidity: 60.0,
        active: true,
    };
    let json = serde_json::to_string(&sensor).unwrap();
    assert_eq!(json, "[42,25.5,60.0,true]");
}

#[test]
fn test_tuple_lenient_serialize_with_string() {
    let record = Record {
        count: 100,
        score: 95.5,
        label: "test".to_string(),
        enabled: Some(true),
    };
    let json = serde_json::to_string(&record).unwrap();
    assert_eq!(json, r#"[100,95.5,"test",true]"#);
}

#[test]
fn test_tuple_lenient_serialize_with_none() {
    let record = Record {
        count: 50,
        score: 75.0,
        label: "data".to_string(),
        enabled: None,
    };
    let json = serde_json::to_string(&record).unwrap();
    assert_eq!(json, r#"[50,75.0,"data",null]"#);
}

// Deserialization with numbers

#[test]
fn test_tuple_lenient_deserialize_numbers() {
    let json = "[1234567890,23.5]";
    let measurement: Measurement = serde_json::from_str(json).unwrap();
    assert_eq!(
        measurement,
        Measurement {
            timestamp: 1234567890,
            value: 23.5,
        }
    );
}

#[test]
fn test_tuple_lenient_deserialize_sensor_numbers() {
    let json = "[42,25.5,60.0,true]";
    let sensor: SensorData = serde_json::from_str(json).unwrap();
    assert_eq!(
        sensor,
        SensorData {
            id: 42,
            temperature: 25.5,
            humidity: 60.0,
            active: true,
        }
    );
}

// Deserialization with strings (lenient parsing)

#[test]
fn test_tuple_lenient_deserialize_strings() {
    let json = r#"["1234567890","23.5"]"#;
    let measurement: Measurement = serde_json::from_str(json).unwrap();
    assert_eq!(
        measurement,
        Measurement {
            timestamp: 1234567890,
            value: 23.5,
        }
    );
}

#[test]
fn test_tuple_lenient_deserialize_sensor_strings() {
    let json = r#"["42","25.5","60.0",true]"#;
    let sensor: SensorData = serde_json::from_str(json).unwrap();
    assert_eq!(
        sensor,
        SensorData {
            id: 42,
            temperature: 25.5,
            humidity: 60.0,
            active: true,
        }
    );
}

#[test]
fn test_tuple_lenient_deserialize_mixed_types() {
    let json = r#"["100",95.5,"test",true]"#;
    let record: Record = serde_json::from_str(json).unwrap();
    assert_eq!(
        record,
        Record {
            count: 100,
            score: 95.5,
            label: "test".to_string(),
            enabled: Some(true),
        }
    );
}

#[test]
fn test_tuple_lenient_deserialize_all_strings() {
    let json = r#"["100","95.5","test",true]"#;
    let record: Record = serde_json::from_str(json).unwrap();
    assert_eq!(
        record,
        Record {
            count: 100,
            score: 95.5,
            label: "test".to_string(),
            enabled: Some(true),
        }
    );
}

// Options

#[test]
fn test_tuple_lenient_option_with_null() {
    let json = r#"[100,95.5,"test",null]"#;
    let record: Record = serde_json::from_str(json).unwrap();
    assert_eq!(
        record,
        Record {
            count: 100,
            score: 95.5,
            label: "test".to_string(),
            enabled: None,
        }
    );
}

#[test]
fn test_tuple_lenient_option_with_value() {
    let json = r#"[100,95.5,"test",false]"#;
    let record: Record = serde_json::from_str(json).unwrap();
    assert_eq!(
        record,
        Record {
            count: 100,
            score: 95.5,
            label: "test".to_string(),
            enabled: Some(false),
        }
    );
}

// Field-level control with #[serde_tuplex(skip)]

#[test]
fn test_tuple_lenient_skip_lenient_with_number() {
    let json = r#"["1000",42]"#;
    let data: WithSkip = serde_json::from_str(json).unwrap();
    assert_eq!(
        data,
        WithSkip {
            lenient_field: 1000,
            strict_field: 42,
        }
    );
}

#[test]
fn test_tuple_lenient_skip_strict_rejects_string() {
    let json = r#"[1000,"42"]"#;
    let result: Result<WithSkip, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// Round-trip tests

#[test]
fn test_tuple_lenient_roundtrip_measurement() {
    let original = Measurement {
        timestamp: 9999999999,
        value: 123.456,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Measurement = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_tuple_lenient_roundtrip_sensor() {
    let original = SensorData {
        id: 7,
        temperature: 22.3,
        humidity: 45.7,
        active: false,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: SensorData = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_tuple_lenient_roundtrip_record() {
    let original = Record {
        count: 0,
        score: 0.0,
        label: String::new(),
        enabled: None,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Record = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

// Error cases

#[test]
fn test_tuple_lenient_invalid_string() {
    let json = r#"["not_a_number",23.5]"#;
    let result: Result<Measurement, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_tuple_lenient_missing_field() {
    let json = "[1234567890]";
    let result: Result<Measurement, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_tuple_lenient_empty_array() {
    let json = "[]";
    let result: Result<Measurement, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_tuple_lenient_wrong_type_for_string() {
    let json = "[100,95.5,12345,true]";
    let result: Result<Record, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// Edge cases

#[test]
fn test_tuple_lenient_zero_values() {
    let json = r#"["0","0.0"]"#;
    let measurement: Measurement = serde_json::from_str(json).unwrap();
    assert_eq!(
        measurement,
        Measurement {
            timestamp: 0,
            value: 0.0,
        }
    );
}

#[test]
fn test_tuple_lenient_negative_numbers() {
    #[derive(Debug, TupleLenient, PartialEq)]
    struct Signed {
        a: i64,
        b: f64,
    }

    let json = r#"["-999","-123.456"]"#;
    let signed: Signed = serde_json::from_str(json).unwrap();
    assert_eq!(
        signed,
        Signed {
            a: -999,
            b: -123.456
        }
    );
}

#[test]
fn test_tuple_lenient_large_numbers() {
    let json = r#"["18446744073709551615","999999999.99"]"#;
    let measurement: Measurement = serde_json::from_str(json).unwrap();
    assert_eq!(measurement.timestamp, u64::MAX);
    assert_eq!(measurement.value, 999999999.99);
}

#[test]
fn test_tuple_lenient_scientific_notation() {
    let json = r#"[1000,"1.23e10"]"#;
    let measurement: Measurement = serde_json::from_str(json).unwrap();
    assert_eq!(measurement.timestamp, 1000);
    assert_eq!(measurement.value, 1.23e10);
}

#[test]
fn test_tuple_lenient_with_single_field() {
    #[derive(Debug, TupleLenient, PartialEq)]
    struct Single {
        value: u64,
    }

    let json = r#"["42"]"#;
    let single: Single = serde_json::from_str(json).unwrap();
    assert_eq!(single, Single { value: 42 });

    let serialized = serde_json::to_string(&single).unwrap();
    assert_eq!(serialized, "[42]");
}

#[test]
fn test_tuple_lenient_preserves_non_numeric_fields() {
    let json = r#"["100","95.5","Unicode 🎉 Test",true]"#;
    let record: Record = serde_json::from_str(json).unwrap();
    assert_eq!(record.label, "Unicode 🎉 Test");
}

#[test]
fn test_tuple_lenient_extra_fields_error() {
    let json = r#"["1234567890","23.5","extra","ignored"]"#;
    let result: Result<Measurement, _> = serde_json::from_str(json);
    // Serde's tuple deserialization is strict and doesn't allow trailing elements
    assert!(result.is_err());
}
