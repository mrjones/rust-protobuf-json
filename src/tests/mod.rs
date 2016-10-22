extern crate serde_json;
extern crate std;

mod test_proto;

#[test]
fn simple_proto_to_json() {
    let mut p = test_proto::SimpleProto::new();
    p.set_int_field(100);

    let actual = super::proto_to_json(&p);
    let mut expected = serde_json::Map::new();
    expected.insert("int_field".to_string(), serde_json::Value::I64(100));
    assert_eq!(serde_json::Value::Object(expected), actual);
    assert_eq!("{\"int_field\":100}", serde_json::to_string(&actual).unwrap());
}

#[test]
fn full_proto_to_json() {
    let mut p = test_proto::FullProto::new();
    p.set_double_field(1.0);
    p.set_float_field(2.0);
    p.set_int32_field(3);
    p.set_int64_field(4);
    p.set_uint32_field(5);
    p.set_uint64_field(6);
    p.set_sint32_field(7);
    p.set_sint64_field(8);
    p.set_fixed32_field(9);
    p.set_fixed64_field(10);
    p.set_sfixed32_field(11);
    p.set_sfixed64_field(12);
    p.set_bool_field(true);
    p.set_string_field("string_val".to_string());
    p.set_bytes_field(vec![1, 2, 3]);

    let actual = super::proto_to_json(&p);
    let mut expected = serde_json::Map::new();
    expected.insert("double_field".to_string(), serde_json::Value::F64(1.0));
    expected.insert("float_field".to_string(), serde_json::Value::F64(2.0));
    expected.insert("int32_field".to_string(), serde_json::Value::I64(3));
    expected.insert("int64_field".to_string(), serde_json::Value::I64(4));
    expected.insert("uint32_field".to_string(), serde_json::Value::U64(5));
    expected.insert("uint64_field".to_string(), serde_json::Value::U64(6));
    expected.insert("sint32_field".to_string(), serde_json::Value::I64(7));
    expected.insert("sint64_field".to_string(), serde_json::Value::I64(8));
    expected.insert("fixed32_field".to_string(), serde_json::Value::U64(9));
    expected.insert("fixed64_field".to_string(), serde_json::Value::U64(10));
    expected.insert("sfixed32_field".to_string(), serde_json::Value::I64(11));
    expected.insert("sfixed64_field".to_string(), serde_json::Value::I64(12));
    expected.insert("bool_field".to_string(), serde_json::Value::Bool(true));
    expected.insert("string_field".to_string(),
                    serde_json::Value::String("string_val".to_string()));
    expected.insert("bytes_field".to_string(),
                    serde_json::Value::String(
                        std::str::from_utf8(&[1,2,3]).unwrap().to_string()));
    assert_eq!(serde_json::Value::Object(expected), actual);
}
