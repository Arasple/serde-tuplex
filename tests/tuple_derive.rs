use serde_tuplex::Tuple;

#[derive(Debug, Tuple, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Tuple, PartialEq)]
struct Measurement {
    timestamp: u64,
    value: f64,
    sensor_id: u32,
}

#[derive(Debug, Tuple, PartialEq)]
struct Mixed {
    count: u32,
    name: String,
    active: bool,
}

// Serialization tests

#[test]
fn test_tuple_serialize_simple() {
    let point = Point { x: 10, y: 20 };
    let json = serde_json::to_string(&point).unwrap();
    assert_eq!(json, "[10,20]");
}

#[test]
fn test_tuple_serialize_measurement() {
    let measurement = Measurement {
        timestamp: 1234567890,
        value: 23.5,
        sensor_id: 42,
    };
    let json = serde_json::to_string(&measurement).unwrap();
    assert_eq!(json, "[1234567890,23.5,42]");
}

#[test]
fn test_tuple_serialize_mixed_types() {
    let mixed = Mixed {
        count: 100,
        name: "test".to_string(),
        active: true,
    };
    let json = serde_json::to_string(&mixed).unwrap();
    assert_eq!(json, r#"[100,"test",true]"#);
}

#[test]
fn test_tuple_serialize_zero_values() {
    let point = Point { x: 0, y: 0 };
    let json = serde_json::to_string(&point).unwrap();
    assert_eq!(json, "[0,0]");
}

// Deserialization tests

#[test]
fn test_tuple_deserialize_simple() {
    let json = "[10,20]";
    let point: Point = serde_json::from_str(json).unwrap();
    assert_eq!(point, Point { x: 10, y: 20 });
}

#[test]
fn test_tuple_deserialize_measurement() {
    let json = "[1234567890,23.5,42]";
    let measurement: Measurement = serde_json::from_str(json).unwrap();
    assert_eq!(
        measurement,
        Measurement {
            timestamp: 1234567890,
            value: 23.5,
            sensor_id: 42,
        }
    );
}

#[test]
fn test_tuple_deserialize_mixed_types() {
    let json = r#"[100,"test",true]"#;
    let mixed: Mixed = serde_json::from_str(json).unwrap();
    assert_eq!(
        mixed,
        Mixed {
            count: 100,
            name: "test".to_string(),
            active: true,
        }
    );
}

#[test]
fn test_tuple_deserialize_extra_fields_error() {
    let json = "[10,20,30,40]";
    let result: Result<Point, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_tuple_deserialize_missing_field() {
    let json = "[10]";
    let result: Result<Point, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_tuple_deserialize_wrong_type() {
    let json = r#"["not_a_number",20]"#;
    let result: Result<Point, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_tuple_deserialize_empty_array() {
    let json = "[]";
    let result: Result<Point, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// Round-trip tests

#[test]
fn test_tuple_roundtrip_simple() {
    let original = Point { x: 99, y: 88 };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Point = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_tuple_roundtrip_measurement() {
    let original = Measurement {
        timestamp: 9999999999,
        value: 123.456,
        sensor_id: 7,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Measurement = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_tuple_roundtrip_mixed() {
    let original = Mixed {
        count: 0,
        name: String::new(),
        active: false,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Mixed = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_tuple_serialize_large_numbers() {
    #[derive(Debug, Tuple, PartialEq)]
    struct LargeNumbers {
        max_u64: u64,
        min_i64: i64,
    }

    let large = LargeNumbers {
        max_u64: u64::MAX,
        min_i64: i64::MIN,
    };
    let json = serde_json::to_string(&large).unwrap();
    let deserialized: LargeNumbers = serde_json::from_str(&json).unwrap();
    assert_eq!(large, deserialized);
}

#[test]
fn test_tuple_with_single_field() {
    #[derive(Debug, Tuple, PartialEq)]
    struct Single {
        value: u32,
    }

    let single = Single { value: 42 };
    let json = serde_json::to_string(&single).unwrap();
    assert_eq!(json, "[42]");

    let deserialized: Single = serde_json::from_str(&json).unwrap();
    assert_eq!(single, deserialized);
}

#[test]
fn test_tuple_with_many_fields() {
    #[derive(Debug, Tuple, PartialEq)]
    struct ManyFields {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: i8,
        f: i16,
    }

    let many = ManyFields {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: -5,
        f: -6,
    };
    let json = serde_json::to_string(&many).unwrap();
    assert_eq!(json, "[1,2,3,4,-5,-6]");

    let deserialized: ManyFields = serde_json::from_str(&json).unwrap();
    assert_eq!(many, deserialized);
}
