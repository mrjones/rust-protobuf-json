extern crate protobuf;
extern crate serde_json;

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
    match field_descriptor.proto().get_field_type() {
        protobuf::descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
            return serde_json::Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return proto_to_json(field_descriptor.get_rep_message_item(msg, i));
            }));
        },
        protobuf::descriptor::FieldDescriptorProto_Type::TYPE_STRING => {
            return serde_json::Value::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                return serde_json::Value::String(field_descriptor.get_rep_str_item(msg, i).to_string());
            }));
        },
        _ => unimplemented!(),
    }
}

fn singular_field_to_json(message: &protobuf::Message,
                          field_descriptor: &protobuf::reflect::FieldDescriptor) -> serde_json::Value {
    match field_descriptor.proto().get_field_type() {
        protobuf::descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
            let sub_message: &protobuf::Message =
                field_descriptor.get_message(message);
            return proto_to_json(sub_message);
        },
        protobuf::descriptor::FieldDescriptorProto_Type::TYPE_STRING => {
            return serde_json::Value::String(field_descriptor.get_str(message).to_string());
        },
        protobuf::descriptor::FieldDescriptorProto_Type::TYPE_INT32 => {
            return serde_json::Value::I64(field_descriptor.get_i32(message) as i64);
        },
        protobuf::descriptor::FieldDescriptorProto_Type::TYPE_INT64 => {
            return serde_json::Value::I64(field_descriptor.get_i64(message));
        },
        protobuf::descriptor::FieldDescriptorProto_Type::TYPE_UINT64 => {
            return serde_json::Value::U64(field_descriptor.get_u64(message));
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
