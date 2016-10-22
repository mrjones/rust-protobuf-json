extern crate serde_json;

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
