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

fn collect_repeated(
    message: &protobuf::Message,
    field_descriptor: &protobuf::reflect::FieldDescriptor,
    extract_fn: &Fn(&protobuf::Message, usize) -> serde_json::Value) -> Vec<serde_json::Value> {

    let mut jsons = vec![];
    for i in 0..field_descriptor.len_field(message) {
        jsons.push(extract_fn(message, i));
    }
    return jsons;
}

fn repeated_field_to_json(message: &protobuf::Message,
                          field_descriptor: &protobuf::reflect::FieldDescriptor) -> serde_json::Value {
    use protobuf::descriptor::FieldDescriptorProto_Type;
    use serde_json::Value;

    match field_descriptor.proto().get_field_type() {
        FieldDescriptorProto_Type::TYPE_DOUBLE => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return Value::F64(field_descriptor.get_rep_f64(msg)[i]);
            }));
        },
        FieldDescriptorProto_Type::TYPE_FLOAT => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return Value::F64(field_descriptor.get_rep_f32(msg)[i] as f64);
            }));
        },
        FieldDescriptorProto_Type::TYPE_INT32 => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return Value::I64(field_descriptor.get_rep_i32(msg)[i] as i64);
            }));
        },
        FieldDescriptorProto_Type::TYPE_INT64 => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return Value::I64(field_descriptor.get_rep_i64(msg)[i]);
            }));
        },
        FieldDescriptorProto_Type::TYPE_UINT32 => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return Value::U64(field_descriptor.get_rep_u32(msg)[i] as u64);
            }));
        },
        FieldDescriptorProto_Type::TYPE_UINT64 => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return Value::U64(field_descriptor.get_rep_u64(msg)[i]);
            }));
        },
        FieldDescriptorProto_Type::TYPE_STRING => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return Value::String(field_descriptor.get_rep_str_item(msg, i).to_string());
            }));
        },
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            return Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return proto_to_json(field_descriptor.get_rep_message_item(msg, i));
            }));
        },
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
