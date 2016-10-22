extern crate protobuf;
extern crate serde_json;

#[cfg(test)]
mod tests;

pub fn proto_to_json(message: &protobuf::Message) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    for field in message.descriptor().fields() {
        map.insert(field.name().to_string(), field_to_json(message, field));
    }

    return serde_json::Value::Object(map);
}

fn field_to_json(message: &protobuf::Message,
                 field_descriptor: &protobuf::reflect::FieldDescriptor) -> serde_json::Value {
    if field_descriptor.is_repeated() {
        return repeated_field_to_json(message, field_descriptor);
    } else {
        return singular_field_to_json(message, field_descriptor);
    }
}

// Extracts a Vec<T> from a repeated proto field.
// Most field types already have a function for extracting a Vec<T> directly,
// however a few (e.g. Message) only have "len" and "get_item(i)" functions.
// This function uses the len & get_item functions in order to create vector.
fn extract_vec_shim<T>(
    message: &protobuf::Message,
    get_size_fn: &Fn(&protobuf::Message) -> usize,
    extract_one_fn: &Fn(&protobuf::Message, usize) -> T) -> Vec<T> {

    let size = get_size_fn(message);
    let mut v = Vec::new();
    for i in 0..size {
        v.push(extract_one_fn(message, i));
    }
    return v;
}

fn repeated_to_serde_array<T>(
    message: &protobuf::Message,
    extract_fn: &Fn(&protobuf::Message) -> Vec<T>,
    convert_one_fn: &Fn(T) -> serde_json::Value) -> serde_json::Value {

    return serde_json::Value::Array(
        extract_fn(message).into_iter().map(convert_one_fn).collect());
}

fn repeated_field_to_json(message: &protobuf::Message,
                          field_descriptor: &protobuf::reflect::FieldDescriptor) -> serde_json::Value {
    use protobuf::descriptor::FieldDescriptorProto_Type;
    use serde_json::Value;

    match field_descriptor.proto().get_field_type() {
        FieldDescriptorProto_Type::TYPE_DOUBLE => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_f64(m).to_vec(),
                &Value::F64);
        },
        FieldDescriptorProto_Type::TYPE_FLOAT => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_f32(m).to_vec(),
                &|v| Value::F64(v as f64));
        },
        FieldDescriptorProto_Type::TYPE_INT32 |
        FieldDescriptorProto_Type::TYPE_SINT32 |
        FieldDescriptorProto_Type::TYPE_SFIXED32 => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_i32(m).to_vec(),
                &|v| Value::I64(v as i64));
        },
        FieldDescriptorProto_Type::TYPE_INT64 |
        FieldDescriptorProto_Type::TYPE_SINT64 |
        FieldDescriptorProto_Type::TYPE_SFIXED64 => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_i64(m).to_vec(),
                &Value::I64);
        },
        FieldDescriptorProto_Type::TYPE_UINT32 |
        FieldDescriptorProto_Type::TYPE_FIXED32 => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_u32(m).to_vec(),
                &|v| Value::U64(v as u64));
        },
        FieldDescriptorProto_Type::TYPE_UINT64 |
        FieldDescriptorProto_Type::TYPE_FIXED64 => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_u64(m).to_vec(),
                &Value::U64);
        },
        FieldDescriptorProto_Type::TYPE_BOOL => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_bool(m).to_vec(),
                &Value::Bool);
        },
        FieldDescriptorProto_Type::TYPE_STRING => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_str(m).to_vec(),
                &Value::String);
        },
        FieldDescriptorProto_Type::TYPE_BYTES => {
            return repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_bytes(m).to_vec(),
                &|v| Value::String(std::str::from_utf8(&v).unwrap().to_string()));
        },
//        FieldDescriptorProto_Type::TYPE_MESSAGE => {
//            return repeated_to_serde_array(
//                message,
//                &|m1| extract_vec_shim(
//                    m1,
//                    &|m2| field_descriptor.len_field(m2),
//                    &|m2, i| field_descriptor.get_rep_message_item(m2, i),
//                ),
//                &proto_to_json);
//        },
        _ => unimplemented!(),
    }
}

fn singular_field_to_json(message: &protobuf::Message,
                          field_descriptor: &protobuf::reflect::FieldDescriptor) -> serde_json::Value {
    use protobuf::descriptor::FieldDescriptorProto_Type;
    use serde_json::Value;

    println!("Considering: {:?}", field_descriptor.proto());
    
    match field_descriptor.proto().get_field_type() {
        FieldDescriptorProto_Type::TYPE_DOUBLE => {
            return Value::F64(field_descriptor.get_f64(message));
        },
        FieldDescriptorProto_Type::TYPE_FLOAT => {
            return Value::F64(field_descriptor.get_f32(message) as f64);
        },
        FieldDescriptorProto_Type::TYPE_INT32 |
        FieldDescriptorProto_Type::TYPE_SINT32 |
        FieldDescriptorProto_Type::TYPE_SFIXED32 => {
            return Value::I64(field_descriptor.get_i32(message) as i64);
        },
        FieldDescriptorProto_Type::TYPE_INT64 |
        FieldDescriptorProto_Type::TYPE_SINT64 |
        FieldDescriptorProto_Type::TYPE_SFIXED64 => {
            return Value::I64(field_descriptor.get_i64(message));
        },
        FieldDescriptorProto_Type::TYPE_UINT32 |
        FieldDescriptorProto_Type::TYPE_FIXED32 => {
            return Value::U64(field_descriptor.get_u32(message) as u64);
        },
        FieldDescriptorProto_Type::TYPE_UINT64 |
        FieldDescriptorProto_Type::TYPE_FIXED64 => {
            return Value::U64(field_descriptor.get_u64(message));
        },
        FieldDescriptorProto_Type::TYPE_BOOL => {
            return Value::Bool(field_descriptor.get_bool(message));
        },
        FieldDescriptorProto_Type::TYPE_STRING => {
            return Value::String(field_descriptor.get_str(message).to_string());
        },
        FieldDescriptorProto_Type::TYPE_BYTES => {
            return Value::String(
                std::str::from_utf8(
                    field_descriptor.get_bytes(message)).unwrap().to_string());
        },
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            let sub_message: &protobuf::Message =
                field_descriptor.get_message(message);
            return proto_to_json(sub_message);
        },
        _ => unimplemented!(),
    }
}
