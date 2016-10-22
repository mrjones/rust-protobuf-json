extern crate protobuf;
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

    p.mut_sub_message_field().set_sub_string_field(
        "sub_string_value".to_string());

    p.set_repeated_double_field(vec![100.0, 200.0]);
    p.set_repeated_float_field(vec![101.0, 201.0]);
    p.set_repeated_int32_field(vec![102, 202]);
    p.set_repeated_int64_field(vec![103, 203]);
    p.set_repeated_uint32_field(vec![104, 204]);
    p.set_repeated_uint64_field(vec![105, 205]);
    p.set_repeated_sint32_field(vec![106, 206]);
    p.set_repeated_sint64_field(vec![107, 207]);
    p.set_repeated_fixed32_field(vec![108, 208]);
    p.set_repeated_fixed64_field(vec![109, 209]);
    p.set_repeated_sfixed32_field(vec![110, 210]);
    p.set_repeated_sfixed64_field(vec![111, 211]);
    p.set_repeated_bool_field(vec![true, false]);
    p.set_repeated_string_field(protobuf::RepeatedField::from_vec(
        vec!["string1".to_string(), "string2".to_string()]));
    p.set_repeated_bytes_field(protobuf::RepeatedField::from_vec(
        vec![vec![1,2,3], vec![10, 11, 12]]));
    p.set_repeated_sub_message_field(protobuf::RepeatedField::from_vec(
        vec!["sub_string1", "sub_string2"].iter().map(|s| {
            let mut sub_message = test_proto::SubMessage::new();
            sub_message.set_sub_string_field(s.to_string());
            return sub_message;
        }).collect()));
    
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

    {
        let mut sub_expected = serde_json::Map::new();
        sub_expected.insert(
            "sub_string_field".to_string(),
            serde_json::Value::String("sub_string_value".to_string()));
        expected.insert("sub_message_field".to_string(),
                        serde_json::Value::Object(sub_expected));
    }

    expected.insert("repeated_double_field".to_string(),
                    to_serde_array(vec![100., 200.], &serde_json::Value::F64));
    expected.insert("repeated_float_field".to_string(),
                    to_serde_array(vec![101., 201.], &serde_json::Value::F64));
    expected.insert("repeated_int32_field".to_string(),
                    to_serde_array(vec![102, 202], &serde_json::Value::I64));
    expected.insert("repeated_int64_field".to_string(),
                    to_serde_array(vec![103, 203], &serde_json::Value::I64));
    expected.insert("repeated_uint32_field".to_string(),
                    to_serde_array(vec![104, 204], &serde_json::Value::U64));
    expected.insert("repeated_uint64_field".to_string(),
                    to_serde_array(vec![105, 205], &serde_json::Value::U64));
    expected.insert("repeated_sint32_field".to_string(),
                    to_serde_array(vec![106, 206], &serde_json::Value::I64));
    expected.insert("repeated_sint64_field".to_string(),
                    to_serde_array(vec![107, 207], &serde_json::Value::I64));
    expected.insert("repeated_fixed32_field".to_string(),
                    to_serde_array(vec![108, 208], &serde_json::Value::U64));
    expected.insert("repeated_fixed64_field".to_string(),
                    to_serde_array(vec![109, 209], &serde_json::Value::U64));
    expected.insert("repeated_sfixed32_field".to_string(),
                    to_serde_array(vec![110, 210], &serde_json::Value::I64));
    expected.insert("repeated_sfixed64_field".to_string(),
                    to_serde_array(vec![111, 211], &serde_json::Value::I64));
    expected.insert("repeated_bool_field".to_string(),
                    to_serde_array(vec![true, false], &serde_json::Value::Bool));
    expected.insert("repeated_string_field".to_string(),
                    to_serde_array(
                        vec!["string1".to_string(), "string2".to_string()],
                        &serde_json::Value::String));
    expected.insert("repeated_bytes_field".to_string(),
                    to_serde_array(
                        vec![vec![1,2,3], vec![10,11,12]].iter().map(
                            |x| std::str::from_utf8(x).unwrap().to_string()).collect(),
                        &serde_json::Value::String));

    {
        expected.insert(
            "repeated_sub_message_field".to_string(),
            serde_json::Value::Array(
                vec!["sub_string1", "sub_string2"].iter().map(|s| {
                    let mut sub_expected = serde_json::Map::new();
                    sub_expected.insert(
                        "sub_string_field".to_string(),
                        serde_json::Value::String(s.to_string()));
                    return serde_json::Value::Object(sub_expected);
                }).collect()));
    }

    
    assert_eq!(serde_json::Value::Object(expected), actual);
}

fn to_serde_array<T>(v: Vec<T>, conv_fn: &Fn(T) -> serde_json::Value) -> serde_json::Value {
    return serde_json::Value::Array(v.into_iter().map(conv_fn).collect());
}
