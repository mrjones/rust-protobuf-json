extern crate protobuf;
extern crate serde_json;

use protobuf::Message;
use protobuf::descriptor::FieldDescriptorProto_Type;
use protobuf::reflect::FieldDescriptor;

#[cfg(test)]
mod tests;

pub fn proto_to_json(message: &dyn Message) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    for field in message.descriptor().fields() {
        match field_to_json(message, field) {
            Some(x) => {
                map.insert(field.name().to_string(), x)
            },
            None => continue
        };
    }
    serde_json::Value::Object(map)
}

fn field_to_json(m: &dyn Message, fd: &FieldDescriptor) -> Option<serde_json::Value> {
    if fd.is_repeated() {
        match fd.len_field(m) {
            0 => None,
            _ => Some(repeated_field_to_json(m, fd)),
        }
    } else if fd.has_field(m) {
        Some(singular_field_to_json(m, fd))
    } else {
        None
    }
}

// Extracts a Vec<T> from a repeated proto field.
// Most field types already have a function for extracting a Vec<T> directly,
// however a few (e.g. Message) only have "len" and "get_item(i)" functions.
// This function uses the len & get_item functions in order to create vector.
#[allow(dead_code)]
fn extract_vec_shim<'a, T>(
    message: &'a dyn Message,
    get_size_fn: &dyn Fn(&dyn Message) -> usize,
    extract_one_fn: &dyn Fn(&'a dyn Message, usize) -> &'a T) -> Vec<&'a T> {

    let size = get_size_fn(message);
    let mut v = Vec::new();
    for i in 0..size {
        v.push(extract_one_fn(message, i));
    }
    v
}

fn repeated_entry_as_value(pbval: &dyn protobuf::reflect::ProtobufValue) -> serde_json::Value {
    use protobuf::reflect::ProtobufValueRef;
    use serde_json::Value;
    use serde_json::Number;

    // |x: &dyn protobuf::reflect::ProtobufValue| serde_json::Value::from(x)
    match pbval.as_ref() {
        ProtobufValueRef::Bool(x) => Value::Bool(x),
        ProtobufValueRef::U32(x) => Value::Number(Number::from(x)),
        ProtobufValueRef::U64(x) => Value::Number(Number::from(x)),
        ProtobufValueRef::I32(x) => Value::Number(Number::from(x)),
        ProtobufValueRef::I64(x) => Value::Number(Number::from(x)),
        ProtobufValueRef::F32(x) => Value::Number(Number::from_f64(x as f64).expect("repeated f32")),
        ProtobufValueRef::F64(x) => Value::Number(Number::from_f64(x).expect("repeated f64")),
        ProtobufValueRef::Enum(x) => serde_json::Value::String(x.name().to_string()),
        ProtobufValueRef::String(x) => serde_json::Value::String(x.to_string()),
        ProtobufValueRef::Bytes(x) => serde_json::Value::String(std::str::from_utf8(x).expect("repeated bytes to string").to_string()),
        ProtobufValueRef::Message(x) => proto_to_json(x),
    }
}

fn repeated_field_to_json(message: &dyn Message,
                          field_descriptor: &FieldDescriptor) -> serde_json::Value {


    match field_descriptor.get_reflect(message) {
        // rep_field is a "&dyn protobuf::reflect::repeated::ReflectRepeatedRef"
        protobuf::reflect::ReflectFieldRef::Repeated(rep_field) => {
            return serde_json::Value::Array(
                rep_field.into_iter().map(repeated_entry_as_value).collect());
        },
        _ => panic!("No repeated field"),
    }

}

fn singular_field_to_json(message: &dyn protobuf::Message,
                          field_descriptor: &protobuf::reflect::FieldDescriptor) -> serde_json::Value {
    match field_descriptor.proto().get_field_type() {
        FieldDescriptorProto_Type::TYPE_DOUBLE => {
            serde_json::Value::from(field_descriptor.get_f64(message))
        },
        FieldDescriptorProto_Type::TYPE_FLOAT => {
            serde_json::Value::from(f64::from(field_descriptor.get_f32(message)))
        },
        FieldDescriptorProto_Type::TYPE_INT32 |
        FieldDescriptorProto_Type::TYPE_SINT32 |
        FieldDescriptorProto_Type::TYPE_SFIXED32 => {
            serde_json::Value::from(i64::from(field_descriptor.get_i32(message)))
        },
        FieldDescriptorProto_Type::TYPE_INT64 |
        FieldDescriptorProto_Type::TYPE_SINT64 |
        FieldDescriptorProto_Type::TYPE_SFIXED64 => {
            serde_json::Value::from(field_descriptor.get_i64(message))
        },
        FieldDescriptorProto_Type::TYPE_UINT32 |
        FieldDescriptorProto_Type::TYPE_FIXED32 => {
            serde_json::Value::from(u64::from(field_descriptor.get_u32(message)))
        },
        FieldDescriptorProto_Type::TYPE_UINT64 |
        FieldDescriptorProto_Type::TYPE_FIXED64 => {
            serde_json::Value::from(field_descriptor.get_u64(message))
        },
        FieldDescriptorProto_Type::TYPE_BOOL => {
            serde_json::Value::Bool(field_descriptor.get_bool(message))
        },
        FieldDescriptorProto_Type::TYPE_STRING => {
            serde_json::Value::String(field_descriptor.get_str(message).to_string())
        },
        FieldDescriptorProto_Type::TYPE_BYTES => {
            serde_json::Value::String(
                std::str::from_utf8(
                    field_descriptor.get_bytes(message)).unwrap().to_string())
        },
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            let sub_message: &dyn protobuf::Message =
                field_descriptor.get_message(message);
            proto_to_json(sub_message)
        },
        FieldDescriptorProto_Type::TYPE_ENUM => {
            serde_json::Value::String(
                field_descriptor.get_enum(message).name().to_string())

        },
        FieldDescriptorProto_Type::TYPE_GROUP => unimplemented!(),
    }
}
