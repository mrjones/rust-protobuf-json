// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct SimpleProto {
    // message fields
    int_field: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SimpleProto {}

impl SimpleProto {
    pub fn new() -> SimpleProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SimpleProto {
        static mut instance: ::protobuf::lazy::Lazy<SimpleProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SimpleProto,
        };
        unsafe {
            instance.get(|| {
                SimpleProto {
                    int_field: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 int_field = 1;

    pub fn clear_int_field(&mut self) {
        self.int_field = ::std::option::Option::None;
    }

    pub fn has_int_field(&self) -> bool {
        self.int_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int_field(&mut self, v: i64) {
        self.int_field = ::std::option::Option::Some(v);
    }

    pub fn get_int_field(&self) -> i64 {
        self.int_field.unwrap_or(0)
    }
}

impl ::protobuf::Message for SimpleProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.int_field = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.int_field {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.int_field {
            try!(os.write_int64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SimpleProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SimpleProto {
    fn new() -> SimpleProto {
        SimpleProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SimpleProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "int_field",
                    SimpleProto::has_int_field,
                    SimpleProto::get_int_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SimpleProto>(
                    "SimpleProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SimpleProto {
    fn clear(&mut self) {
        self.clear_int_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SimpleProto {
    fn eq(&self, other: &SimpleProto) -> bool {
        self.int_field == other.int_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SimpleProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FullProto {
    // message fields
    double_field: ::std::option::Option<f64>,
    float_field: ::std::option::Option<f32>,
    int32_field: ::std::option::Option<i32>,
    int64_field: ::std::option::Option<i64>,
    uint32_field: ::std::option::Option<u32>,
    uint64_field: ::std::option::Option<u64>,
    sint32_field: ::std::option::Option<i32>,
    sint64_field: ::std::option::Option<i64>,
    fixed32_field: ::std::option::Option<u32>,
    fixed64_field: ::std::option::Option<u64>,
    sfixed32_field: ::std::option::Option<i32>,
    sfixed64_field: ::std::option::Option<i64>,
    bool_field: ::std::option::Option<bool>,
    string_field: ::protobuf::SingularField<::std::string::String>,
    bytes_field: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sub_message_field: ::protobuf::SingularPtrField<SubMessage>,
    repeated_double_field: ::std::vec::Vec<f64>,
    repeated_float_field: ::std::vec::Vec<f64>,
    repeated_int32_field: ::std::vec::Vec<i32>,
    repeated_int64_field: ::std::vec::Vec<i64>,
    repeated_uint32_field: ::std::vec::Vec<u32>,
    repeated_uint64_field: ::std::vec::Vec<u64>,
    repeated_sint32_field: ::std::vec::Vec<i32>,
    repeated_sint64_field: ::std::vec::Vec<i64>,
    repeated_fixed32_field: ::std::vec::Vec<u32>,
    repeated_fixed64_field: ::std::vec::Vec<u64>,
    repeated_sfixed32_field: ::std::vec::Vec<i32>,
    repeated_sfixed64_field: ::std::vec::Vec<i64>,
    repeated_bool_field: ::std::vec::Vec<bool>,
    repeated_string_field: ::protobuf::RepeatedField<::std::string::String>,
    repeated_bytes_field: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FullProto {}

impl FullProto {
    pub fn new() -> FullProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FullProto {
        static mut instance: ::protobuf::lazy::Lazy<FullProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FullProto,
        };
        unsafe {
            instance.get(|| {
                FullProto {
                    double_field: ::std::option::Option::None,
                    float_field: ::std::option::Option::None,
                    int32_field: ::std::option::Option::None,
                    int64_field: ::std::option::Option::None,
                    uint32_field: ::std::option::Option::None,
                    uint64_field: ::std::option::Option::None,
                    sint32_field: ::std::option::Option::None,
                    sint64_field: ::std::option::Option::None,
                    fixed32_field: ::std::option::Option::None,
                    fixed64_field: ::std::option::Option::None,
                    sfixed32_field: ::std::option::Option::None,
                    sfixed64_field: ::std::option::Option::None,
                    bool_field: ::std::option::Option::None,
                    string_field: ::protobuf::SingularField::none(),
                    bytes_field: ::protobuf::SingularField::none(),
                    sub_message_field: ::protobuf::SingularPtrField::none(),
                    repeated_double_field: ::std::vec::Vec::new(),
                    repeated_float_field: ::std::vec::Vec::new(),
                    repeated_int32_field: ::std::vec::Vec::new(),
                    repeated_int64_field: ::std::vec::Vec::new(),
                    repeated_uint32_field: ::std::vec::Vec::new(),
                    repeated_uint64_field: ::std::vec::Vec::new(),
                    repeated_sint32_field: ::std::vec::Vec::new(),
                    repeated_sint64_field: ::std::vec::Vec::new(),
                    repeated_fixed32_field: ::std::vec::Vec::new(),
                    repeated_fixed64_field: ::std::vec::Vec::new(),
                    repeated_sfixed32_field: ::std::vec::Vec::new(),
                    repeated_sfixed64_field: ::std::vec::Vec::new(),
                    repeated_bool_field: ::std::vec::Vec::new(),
                    repeated_string_field: ::protobuf::RepeatedField::new(),
                    repeated_bytes_field: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.double_field = ::std::option::Option::None;
    }

    pub fn has_double_field(&self) -> bool {
        self.double_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: f64) {
        self.double_field = ::std::option::Option::Some(v);
    }

    pub fn get_double_field(&self) -> f64 {
        self.double_field.unwrap_or(0.)
    }

    // optional float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.float_field = ::std::option::Option::None;
    }

    pub fn has_float_field(&self) -> bool {
        self.float_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.float_field = ::std::option::Option::Some(v);
    }

    pub fn get_float_field(&self) -> f32 {
        self.float_field.unwrap_or(0.)
    }

    // optional int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.int32_field = ::std::option::Option::None;
    }

    pub fn has_int32_field(&self) -> bool {
        self.int32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: i32) {
        self.int32_field = ::std::option::Option::Some(v);
    }

    pub fn get_int32_field(&self) -> i32 {
        self.int32_field.unwrap_or(0)
    }

    // optional int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.int64_field = ::std::option::Option::None;
    }

    pub fn has_int64_field(&self) -> bool {
        self.int64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: i64) {
        self.int64_field = ::std::option::Option::Some(v);
    }

    pub fn get_int64_field(&self) -> i64 {
        self.int64_field.unwrap_or(0)
    }

    // optional uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.uint32_field = ::std::option::Option::None;
    }

    pub fn has_uint32_field(&self) -> bool {
        self.uint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: u32) {
        self.uint32_field = ::std::option::Option::Some(v);
    }

    pub fn get_uint32_field(&self) -> u32 {
        self.uint32_field.unwrap_or(0)
    }

    // optional uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.uint64_field = ::std::option::Option::None;
    }

    pub fn has_uint64_field(&self) -> bool {
        self.uint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: u64) {
        self.uint64_field = ::std::option::Option::Some(v);
    }

    pub fn get_uint64_field(&self) -> u64 {
        self.uint64_field.unwrap_or(0)
    }

    // optional sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.sint32_field = ::std::option::Option::None;
    }

    pub fn has_sint32_field(&self) -> bool {
        self.sint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: i32) {
        self.sint32_field = ::std::option::Option::Some(v);
    }

    pub fn get_sint32_field(&self) -> i32 {
        self.sint32_field.unwrap_or(0)
    }

    // optional sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.sint64_field = ::std::option::Option::None;
    }

    pub fn has_sint64_field(&self) -> bool {
        self.sint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: i64) {
        self.sint64_field = ::std::option::Option::Some(v);
    }

    pub fn get_sint64_field(&self) -> i64 {
        self.sint64_field.unwrap_or(0)
    }

    // optional fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.fixed32_field = ::std::option::Option::None;
    }

    pub fn has_fixed32_field(&self) -> bool {
        self.fixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: u32) {
        self.fixed32_field = ::std::option::Option::Some(v);
    }

    pub fn get_fixed32_field(&self) -> u32 {
        self.fixed32_field.unwrap_or(0)
    }

    // optional fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.fixed64_field = ::std::option::Option::None;
    }

    pub fn has_fixed64_field(&self) -> bool {
        self.fixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: u64) {
        self.fixed64_field = ::std::option::Option::Some(v);
    }

    pub fn get_fixed64_field(&self) -> u64 {
        self.fixed64_field.unwrap_or(0)
    }

    // optional sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.sfixed32_field = ::std::option::Option::None;
    }

    pub fn has_sfixed32_field(&self) -> bool {
        self.sfixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: i32) {
        self.sfixed32_field = ::std::option::Option::Some(v);
    }

    pub fn get_sfixed32_field(&self) -> i32 {
        self.sfixed32_field.unwrap_or(0)
    }

    // optional sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.sfixed64_field = ::std::option::Option::None;
    }

    pub fn has_sfixed64_field(&self) -> bool {
        self.sfixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: i64) {
        self.sfixed64_field = ::std::option::Option::Some(v);
    }

    pub fn get_sfixed64_field(&self) -> i64 {
        self.sfixed64_field.unwrap_or(0)
    }

    // optional bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.bool_field = ::std::option::Option::None;
    }

    pub fn has_bool_field(&self) -> bool {
        self.bool_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: bool) {
        self.bool_field = ::std::option::Option::Some(v);
    }

    pub fn get_bool_field(&self) -> bool {
        self.bool_field.unwrap_or(false)
    }

    // optional string string_field = 14;

    pub fn clear_string_field(&mut self) {
        self.string_field.clear();
    }

    pub fn has_string_field(&self) -> bool {
        self.string_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::std::string::String) {
        self.string_field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_field(&mut self) -> &mut ::std::string::String {
        if self.string_field.is_none() {
            self.string_field.set_default();
        };
        self.string_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::std::string::String {
        self.string_field.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string_field(&self) -> &str {
        match self.string_field.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes bytes_field = 15;

    pub fn clear_bytes_field(&mut self) {
        self.bytes_field.clear();
    }

    pub fn has_bytes_field(&self) -> bool {
        self.bytes_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_field(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_field(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bytes_field.is_none() {
            self.bytes_field.set_default();
        };
        self.bytes_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes_field(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes_field.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes_field(&self) -> &[u8] {
        match self.bytes_field.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .SubMessage sub_message_field = 16;

    pub fn clear_sub_message_field(&mut self) {
        self.sub_message_field.clear();
    }

    pub fn has_sub_message_field(&self) -> bool {
        self.sub_message_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sub_message_field(&mut self, v: SubMessage) {
        self.sub_message_field = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_message_field(&mut self) -> &mut SubMessage {
        if self.sub_message_field.is_none() {
            self.sub_message_field.set_default();
        };
        self.sub_message_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_sub_message_field(&mut self) -> SubMessage {
        self.sub_message_field.take().unwrap_or_else(|| SubMessage::new())
    }

    pub fn get_sub_message_field(&self) -> &SubMessage {
        self.sub_message_field.as_ref().unwrap_or_else(|| SubMessage::default_instance())
    }

    // repeated double repeated_double_field = 100;

    pub fn clear_repeated_double_field(&mut self) {
        self.repeated_double_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_double_field(&mut self, v: ::std::vec::Vec<f64>) {
        self.repeated_double_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_double_field(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.repeated_double_field
    }

    // Take field
    pub fn take_repeated_double_field(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.repeated_double_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_double_field(&self) -> &[f64] {
        &self.repeated_double_field
    }

    // repeated double repeated_float_field = 101;

    pub fn clear_repeated_float_field(&mut self) {
        self.repeated_float_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_float_field(&mut self, v: ::std::vec::Vec<f64>) {
        self.repeated_float_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_float_field(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.repeated_float_field
    }

    // Take field
    pub fn take_repeated_float_field(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.repeated_float_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_float_field(&self) -> &[f64] {
        &self.repeated_float_field
    }

    // repeated int32 repeated_int32_field = 102;

    pub fn clear_repeated_int32_field(&mut self) {
        self.repeated_int32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_int32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.repeated_int32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_int32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.repeated_int32_field
    }

    // Take field
    pub fn take_repeated_int32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.repeated_int32_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_int32_field(&self) -> &[i32] {
        &self.repeated_int32_field
    }

    // repeated int64 repeated_int64_field = 103;

    pub fn clear_repeated_int64_field(&mut self) {
        self.repeated_int64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_int64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.repeated_int64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_int64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.repeated_int64_field
    }

    // Take field
    pub fn take_repeated_int64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.repeated_int64_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_int64_field(&self) -> &[i64] {
        &self.repeated_int64_field
    }

    // repeated uint32 repeated_uint32_field = 104;

    pub fn clear_repeated_uint32_field(&mut self) {
        self.repeated_uint32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_uint32_field(&mut self, v: ::std::vec::Vec<u32>) {
        self.repeated_uint32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_uint32_field(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.repeated_uint32_field
    }

    // Take field
    pub fn take_repeated_uint32_field(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.repeated_uint32_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_uint32_field(&self) -> &[u32] {
        &self.repeated_uint32_field
    }

    // repeated uint64 repeated_uint64_field = 105;

    pub fn clear_repeated_uint64_field(&mut self) {
        self.repeated_uint64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_uint64_field(&mut self, v: ::std::vec::Vec<u64>) {
        self.repeated_uint64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_uint64_field(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.repeated_uint64_field
    }

    // Take field
    pub fn take_repeated_uint64_field(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.repeated_uint64_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_uint64_field(&self) -> &[u64] {
        &self.repeated_uint64_field
    }

    // repeated sint32 repeated_sint32_field = 106;

    pub fn clear_repeated_sint32_field(&mut self) {
        self.repeated_sint32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_sint32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.repeated_sint32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_sint32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.repeated_sint32_field
    }

    // Take field
    pub fn take_repeated_sint32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.repeated_sint32_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_sint32_field(&self) -> &[i32] {
        &self.repeated_sint32_field
    }

    // repeated sint64 repeated_sint64_field = 107;

    pub fn clear_repeated_sint64_field(&mut self) {
        self.repeated_sint64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_sint64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.repeated_sint64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_sint64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.repeated_sint64_field
    }

    // Take field
    pub fn take_repeated_sint64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.repeated_sint64_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_sint64_field(&self) -> &[i64] {
        &self.repeated_sint64_field
    }

    // repeated fixed32 repeated_fixed32_field = 108;

    pub fn clear_repeated_fixed32_field(&mut self) {
        self.repeated_fixed32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_fixed32_field(&mut self, v: ::std::vec::Vec<u32>) {
        self.repeated_fixed32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_fixed32_field(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.repeated_fixed32_field
    }

    // Take field
    pub fn take_repeated_fixed32_field(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.repeated_fixed32_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_fixed32_field(&self) -> &[u32] {
        &self.repeated_fixed32_field
    }

    // repeated fixed64 repeated_fixed64_field = 109;

    pub fn clear_repeated_fixed64_field(&mut self) {
        self.repeated_fixed64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_fixed64_field(&mut self, v: ::std::vec::Vec<u64>) {
        self.repeated_fixed64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_fixed64_field(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.repeated_fixed64_field
    }

    // Take field
    pub fn take_repeated_fixed64_field(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.repeated_fixed64_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_fixed64_field(&self) -> &[u64] {
        &self.repeated_fixed64_field
    }

    // repeated sfixed32 repeated_sfixed32_field = 110;

    pub fn clear_repeated_sfixed32_field(&mut self) {
        self.repeated_sfixed32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_sfixed32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.repeated_sfixed32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_sfixed32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.repeated_sfixed32_field
    }

    // Take field
    pub fn take_repeated_sfixed32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.repeated_sfixed32_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_sfixed32_field(&self) -> &[i32] {
        &self.repeated_sfixed32_field
    }

    // repeated sfixed64 repeated_sfixed64_field = 111;

    pub fn clear_repeated_sfixed64_field(&mut self) {
        self.repeated_sfixed64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_sfixed64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.repeated_sfixed64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_sfixed64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.repeated_sfixed64_field
    }

    // Take field
    pub fn take_repeated_sfixed64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.repeated_sfixed64_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_sfixed64_field(&self) -> &[i64] {
        &self.repeated_sfixed64_field
    }

    // repeated bool repeated_bool_field = 112;

    pub fn clear_repeated_bool_field(&mut self) {
        self.repeated_bool_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_bool_field(&mut self, v: ::std::vec::Vec<bool>) {
        self.repeated_bool_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_bool_field(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.repeated_bool_field
    }

    // Take field
    pub fn take_repeated_bool_field(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.repeated_bool_field, ::std::vec::Vec::new())
    }

    pub fn get_repeated_bool_field(&self) -> &[bool] {
        &self.repeated_bool_field
    }

    // repeated string repeated_string_field = 113;

    pub fn clear_repeated_string_field(&mut self) {
        self.repeated_string_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_string_field(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.repeated_string_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_string_field(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.repeated_string_field
    }

    // Take field
    pub fn take_repeated_string_field(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.repeated_string_field, ::protobuf::RepeatedField::new())
    }

    pub fn get_repeated_string_field(&self) -> &[::std::string::String] {
        &self.repeated_string_field
    }

    // repeated bytes repeated_bytes_field = 114;

    pub fn clear_repeated_bytes_field(&mut self) {
        self.repeated_bytes_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_repeated_bytes_field(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.repeated_bytes_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_repeated_bytes_field(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.repeated_bytes_field
    }

    // Take field
    pub fn take_repeated_bytes_field(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.repeated_bytes_field, ::protobuf::RepeatedField::new())
    }

    pub fn get_repeated_bytes_field(&self) -> &[::std::vec::Vec<u8>] {
        &self.repeated_bytes_field
    }
}

impl ::protobuf::Message for FullProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.double_field = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.float_field = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.int32_field = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.int64_field = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.uint32_field = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.uint64_field = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.sint32_field = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.sint64_field = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.fixed32_field = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.fixed64_field = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sfixed32());
                    self.sfixed32_field = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.sfixed64_field = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.bool_field = ::std::option::Option::Some(tmp);
                },
                14 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string_field));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes_field));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sub_message_field));
                },
                100 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.repeated_double_field));
                },
                101 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.repeated_float_field));
                },
                102 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.repeated_int32_field));
                },
                103 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.repeated_int64_field));
                },
                104 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.repeated_uint32_field));
                },
                105 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.repeated_uint64_field));
                },
                106 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.repeated_sint32_field));
                },
                107 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.repeated_sint64_field));
                },
                108 => {
                    try!(::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.repeated_fixed32_field));
                },
                109 => {
                    try!(::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.repeated_fixed64_field));
                },
                110 => {
                    try!(::protobuf::rt::read_repeated_sfixed32_into(wire_type, is, &mut self.repeated_sfixed32_field));
                },
                111 => {
                    try!(::protobuf::rt::read_repeated_sfixed64_into(wire_type, is, &mut self.repeated_sfixed64_field));
                },
                112 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.repeated_bool_field));
                },
                113 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.repeated_string_field));
                },
                114 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.repeated_bytes_field));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.double_field.is_some() {
            my_size += 9;
        };
        if self.float_field.is_some() {
            my_size += 5;
        };
        for value in &self.int32_field {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.int64_field {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint32_field {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint64_field {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sint32_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, *value);
        };
        for value in &self.sint64_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, *value);
        };
        if self.fixed32_field.is_some() {
            my_size += 5;
        };
        if self.fixed64_field.is_some() {
            my_size += 9;
        };
        if self.sfixed32_field.is_some() {
            my_size += 5;
        };
        if self.sfixed64_field.is_some() {
            my_size += 9;
        };
        if self.bool_field.is_some() {
            my_size += 2;
        };
        for value in &self.string_field {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        for value in &self.bytes_field {
            my_size += ::protobuf::rt::bytes_size(15, &value);
        };
        for value in &self.sub_message_field {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += 10 * self.repeated_double_field.len() as u32;
        my_size += 10 * self.repeated_float_field.len() as u32;
        for value in &self.repeated_int32_field {
            my_size += ::protobuf::rt::value_size(102, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.repeated_int64_field {
            my_size += ::protobuf::rt::value_size(103, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.repeated_uint32_field {
            my_size += ::protobuf::rt::value_size(104, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.repeated_uint64_field {
            my_size += ::protobuf::rt::value_size(105, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.repeated_sint32_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(106, *value);
        };
        for value in &self.repeated_sint64_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(107, *value);
        };
        my_size += 6 * self.repeated_fixed32_field.len() as u32;
        my_size += 10 * self.repeated_fixed64_field.len() as u32;
        my_size += 6 * self.repeated_sfixed32_field.len() as u32;
        my_size += 10 * self.repeated_sfixed64_field.len() as u32;
        my_size += 3 * self.repeated_bool_field.len() as u32;
        for value in &self.repeated_string_field {
            my_size += ::protobuf::rt::string_size(113, &value);
        };
        for value in &self.repeated_bytes_field {
            my_size += ::protobuf::rt::bytes_size(114, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.double_field {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.float_field {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.int32_field {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.int64_field {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.uint32_field {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.uint64_field {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.sint32_field {
            try!(os.write_sint32(7, v));
        };
        if let Some(v) = self.sint64_field {
            try!(os.write_sint64(8, v));
        };
        if let Some(v) = self.fixed32_field {
            try!(os.write_fixed32(9, v));
        };
        if let Some(v) = self.fixed64_field {
            try!(os.write_fixed64(10, v));
        };
        if let Some(v) = self.sfixed32_field {
            try!(os.write_sfixed32(11, v));
        };
        if let Some(v) = self.sfixed64_field {
            try!(os.write_sfixed64(12, v));
        };
        if let Some(v) = self.bool_field {
            try!(os.write_bool(13, v));
        };
        if let Some(v) = self.string_field.as_ref() {
            try!(os.write_string(14, &v));
        };
        if let Some(v) = self.bytes_field.as_ref() {
            try!(os.write_bytes(15, &v));
        };
        if let Some(v) = self.sub_message_field.as_ref() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.repeated_double_field {
            try!(os.write_double(100, *v));
        };
        for v in &self.repeated_float_field {
            try!(os.write_double(101, *v));
        };
        for v in &self.repeated_int32_field {
            try!(os.write_int32(102, *v));
        };
        for v in &self.repeated_int64_field {
            try!(os.write_int64(103, *v));
        };
        for v in &self.repeated_uint32_field {
            try!(os.write_uint32(104, *v));
        };
        for v in &self.repeated_uint64_field {
            try!(os.write_uint64(105, *v));
        };
        for v in &self.repeated_sint32_field {
            try!(os.write_sint32(106, *v));
        };
        for v in &self.repeated_sint64_field {
            try!(os.write_sint64(107, *v));
        };
        for v in &self.repeated_fixed32_field {
            try!(os.write_fixed32(108, *v));
        };
        for v in &self.repeated_fixed64_field {
            try!(os.write_fixed64(109, *v));
        };
        for v in &self.repeated_sfixed32_field {
            try!(os.write_sfixed32(110, *v));
        };
        for v in &self.repeated_sfixed64_field {
            try!(os.write_sfixed64(111, *v));
        };
        for v in &self.repeated_bool_field {
            try!(os.write_bool(112, *v));
        };
        for v in &self.repeated_string_field {
            try!(os.write_string(113, &v));
        };
        for v in &self.repeated_bytes_field {
            try!(os.write_bytes(114, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FullProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FullProto {
    fn new() -> FullProto {
        FullProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FullProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "double_field",
                    FullProto::has_double_field,
                    FullProto::get_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "float_field",
                    FullProto::has_float_field,
                    FullProto::get_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "int32_field",
                    FullProto::has_int32_field,
                    FullProto::get_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "int64_field",
                    FullProto::has_int64_field,
                    FullProto::get_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "uint32_field",
                    FullProto::has_uint32_field,
                    FullProto::get_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "uint64_field",
                    FullProto::has_uint64_field,
                    FullProto::get_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sint32_field",
                    FullProto::has_sint32_field,
                    FullProto::get_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sint64_field",
                    FullProto::has_sint64_field,
                    FullProto::get_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "fixed32_field",
                    FullProto::has_fixed32_field,
                    FullProto::get_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fixed64_field",
                    FullProto::has_fixed64_field,
                    FullProto::get_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sfixed32_field",
                    FullProto::has_sfixed32_field,
                    FullProto::get_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sfixed64_field",
                    FullProto::has_sfixed64_field,
                    FullProto::get_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bool_field",
                    FullProto::has_bool_field,
                    FullProto::get_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "string_field",
                    FullProto::has_string_field,
                    FullProto::get_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes_field",
                    FullProto::has_bytes_field,
                    FullProto::get_bytes_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sub_message_field",
                    FullProto::has_sub_message_field,
                    FullProto::get_sub_message_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "repeated_double_field",
                    FullProto::get_repeated_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "repeated_float_field",
                    FullProto::get_repeated_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "repeated_int32_field",
                    FullProto::get_repeated_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "repeated_int64_field",
                    FullProto::get_repeated_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "repeated_uint32_field",
                    FullProto::get_repeated_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "repeated_uint64_field",
                    FullProto::get_repeated_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "repeated_sint32_field",
                    FullProto::get_repeated_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "repeated_sint64_field",
                    FullProto::get_repeated_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "repeated_fixed32_field",
                    FullProto::get_repeated_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "repeated_fixed64_field",
                    FullProto::get_repeated_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "repeated_sfixed32_field",
                    FullProto::get_repeated_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "repeated_sfixed64_field",
                    FullProto::get_repeated_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "repeated_bool_field",
                    FullProto::get_repeated_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "repeated_string_field",
                    FullProto::get_repeated_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "repeated_bytes_field",
                    FullProto::get_repeated_bytes_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FullProto>(
                    "FullProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FullProto {
    fn clear(&mut self) {
        self.clear_double_field();
        self.clear_float_field();
        self.clear_int32_field();
        self.clear_int64_field();
        self.clear_uint32_field();
        self.clear_uint64_field();
        self.clear_sint32_field();
        self.clear_sint64_field();
        self.clear_fixed32_field();
        self.clear_fixed64_field();
        self.clear_sfixed32_field();
        self.clear_sfixed64_field();
        self.clear_bool_field();
        self.clear_string_field();
        self.clear_bytes_field();
        self.clear_sub_message_field();
        self.clear_repeated_double_field();
        self.clear_repeated_float_field();
        self.clear_repeated_int32_field();
        self.clear_repeated_int64_field();
        self.clear_repeated_uint32_field();
        self.clear_repeated_uint64_field();
        self.clear_repeated_sint32_field();
        self.clear_repeated_sint64_field();
        self.clear_repeated_fixed32_field();
        self.clear_repeated_fixed64_field();
        self.clear_repeated_sfixed32_field();
        self.clear_repeated_sfixed64_field();
        self.clear_repeated_bool_field();
        self.clear_repeated_string_field();
        self.clear_repeated_bytes_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FullProto {
    fn eq(&self, other: &FullProto) -> bool {
        self.double_field == other.double_field &&
        self.float_field == other.float_field &&
        self.int32_field == other.int32_field &&
        self.int64_field == other.int64_field &&
        self.uint32_field == other.uint32_field &&
        self.uint64_field == other.uint64_field &&
        self.sint32_field == other.sint32_field &&
        self.sint64_field == other.sint64_field &&
        self.fixed32_field == other.fixed32_field &&
        self.fixed64_field == other.fixed64_field &&
        self.sfixed32_field == other.sfixed32_field &&
        self.sfixed64_field == other.sfixed64_field &&
        self.bool_field == other.bool_field &&
        self.string_field == other.string_field &&
        self.bytes_field == other.bytes_field &&
        self.sub_message_field == other.sub_message_field &&
        self.repeated_double_field == other.repeated_double_field &&
        self.repeated_float_field == other.repeated_float_field &&
        self.repeated_int32_field == other.repeated_int32_field &&
        self.repeated_int64_field == other.repeated_int64_field &&
        self.repeated_uint32_field == other.repeated_uint32_field &&
        self.repeated_uint64_field == other.repeated_uint64_field &&
        self.repeated_sint32_field == other.repeated_sint32_field &&
        self.repeated_sint64_field == other.repeated_sint64_field &&
        self.repeated_fixed32_field == other.repeated_fixed32_field &&
        self.repeated_fixed64_field == other.repeated_fixed64_field &&
        self.repeated_sfixed32_field == other.repeated_sfixed32_field &&
        self.repeated_sfixed64_field == other.repeated_sfixed64_field &&
        self.repeated_bool_field == other.repeated_bool_field &&
        self.repeated_string_field == other.repeated_string_field &&
        self.repeated_bytes_field == other.repeated_bytes_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FullProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SubMessage {
    // message fields
    sub_string_field: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubMessage {}

impl SubMessage {
    pub fn new() -> SubMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubMessage {
        static mut instance: ::protobuf::lazy::Lazy<SubMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubMessage,
        };
        unsafe {
            instance.get(|| {
                SubMessage {
                    sub_string_field: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string sub_string_field = 1;

    pub fn clear_sub_string_field(&mut self) {
        self.sub_string_field.clear();
    }

    pub fn has_sub_string_field(&self) -> bool {
        self.sub_string_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sub_string_field(&mut self, v: ::std::string::String) {
        self.sub_string_field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_string_field(&mut self) -> &mut ::std::string::String {
        if self.sub_string_field.is_none() {
            self.sub_string_field.set_default();
        };
        self.sub_string_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_sub_string_field(&mut self) -> ::std::string::String {
        self.sub_string_field.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sub_string_field(&self) -> &str {
        match self.sub_string_field.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SubMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sub_string_field));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.sub_string_field {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sub_string_field.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SubMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubMessage {
    fn new() -> SubMessage {
        SubMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sub_string_field",
                    SubMessage::has_sub_string_field,
                    SubMessage::get_sub_string_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubMessage>(
                    "SubMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubMessage {
    fn clear(&mut self) {
        self.clear_sub_string_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SubMessage {
    fn eq(&self, other: &SubMessage) -> bool {
        self.sub_string_field == other.sub_string_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SubMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x2a, 0x0a, 0x0b, 0x53, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x6e, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x69, 0x6e, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x22, 0xdd,
    0x0a, 0x0a, 0x09, 0x46, 0x75, 0x6c, 0x6c, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x21, 0x0a, 0x0c,
    0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x0b, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12,
    0x1f, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65,
    0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x75, 0x69, 0x6e,
    0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74,
    0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x52, 0x0b,
    0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73,
    0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x12, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x23,
    0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69,
    0x65, 0x6c, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65,
    0x64, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78,
    0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0f,
    0x52, 0x0d, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12,
    0x25, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x10, 0x52, 0x0d, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36,
    0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x6c,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x74, 0x72,
    0x69, 0x6e, 0x67, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x37, 0x0a, 0x11, 0x73, 0x75, 0x62,
    0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x10,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x53, 0x75, 0x62, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x52, 0x0f, 0x73, 0x75, 0x62, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x46, 0x69, 0x65,
    0x6c, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x64,
    0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x64, 0x20, 0x03, 0x28,
    0x01, 0x52, 0x13, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x44, 0x6f, 0x75, 0x62, 0x6c,
    0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74,
    0x65, 0x64, 0x5f, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x65,
    0x20, 0x03, 0x28, 0x01, 0x52, 0x12, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x46, 0x6c,
    0x6f, 0x61, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x5f, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x66, 0x20, 0x03, 0x28, 0x05, 0x52, 0x12, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64,
    0x49, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x72, 0x65,
    0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x67, 0x20, 0x03, 0x28, 0x03, 0x52, 0x12, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74,
    0x65, 0x64, 0x49, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x32, 0x0a, 0x15,
    0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x68, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x13, 0x72, 0x65, 0x70,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x55, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x12, 0x32, 0x0a, 0x15, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x75, 0x69, 0x6e,
    0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x69, 0x20, 0x03, 0x28, 0x04, 0x52,
    0x13, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x55, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64,
    0x5f, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x6a, 0x20,
    0x03, 0x28, 0x11, 0x52, 0x13, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x53, 0x69, 0x6e,
    0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x5f, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x6b, 0x20, 0x03, 0x28, 0x12, 0x52, 0x13, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x53, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x34, 0x0a, 0x16,
    0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x6c, 0x20, 0x03, 0x28, 0x07, 0x52, 0x14, 0x72, 0x65,
    0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x46, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65,
    0x6c, 0x64, 0x12, 0x34, 0x0a, 0x16, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x6d, 0x20, 0x03,
    0x28, 0x06, 0x52, 0x14, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x46, 0x69, 0x78, 0x65,
    0x64, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x36, 0x0a, 0x17, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x5f, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x6e, 0x20, 0x03, 0x28, 0x0f, 0x52, 0x15, 0x72, 0x65, 0x70, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x53, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x12, 0x36, 0x0a, 0x17, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x73, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x6f, 0x20, 0x03, 0x28,
    0x10, 0x52, 0x15, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x53, 0x66, 0x69, 0x78, 0x65,
    0x64, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x2e, 0x0a, 0x13, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x5f, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x70, 0x20, 0x03, 0x28, 0x08, 0x52, 0x11, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x42,
    0x6f, 0x6f, 0x6c, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x71, 0x20, 0x03, 0x28, 0x09, 0x52, 0x13, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x30, 0x0a, 0x14,
    0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x72, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x12, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x42, 0x79, 0x74, 0x65, 0x73, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x22, 0x36,
    0x0a, 0x0a, 0x53, 0x75, 0x62, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x10,
    0x73, 0x75, 0x62, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x73, 0x75, 0x62, 0x53, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4a, 0x93, 0x13, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x33,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x02, 0x00, 0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x03, 0x02, 0x02, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x14, 0x15, 0x0a, 0x4c, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x07, 0x00, 0x2f, 0x01, 0x1a, 0x40, 0x20, 0x53, 0x65, 0x65, 0x20, 0x68, 0x74, 0x74, 0x70,
    0x73, 0x3a, 0x2f, 0x2f, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x2d, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x73, 0x2f, 0x64, 0x6f, 0x63, 0x73, 0x2f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x07, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x02, 0x07, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x09, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x09, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x09, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x16, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0a, 0x02, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0a, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0b,
    0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0b, 0x02, 0x0a,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x0c, 0x02, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x0c, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0c, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x04, 0x0d, 0x02, 0x0c, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x0d, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12,
    0x03, 0x0e, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x04, 0x0e,
    0x02, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0e, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0e, 0x09, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0e, 0x18, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x0f, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x07, 0x04, 0x12, 0x04, 0x0f, 0x02, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x0f, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x0f, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x10, 0x02, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x04, 0x12, 0x04, 0x10, 0x02, 0x0f, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x10, 0x02, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x10, 0x0a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x09, 0x12, 0x03, 0x11, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12,
    0x04, 0x11, 0x02, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x11, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x11, 0x0a,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x11, 0x1a, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x12, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x12, 0x02, 0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x12, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x12, 0x1c, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x03, 0x13,
    0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x13, 0x02, 0x12,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x13, 0x0b, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x13, 0x1c, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x0c, 0x12, 0x03, 0x14, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c,
    0x04, 0x12, 0x04, 0x14, 0x02, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x05,
    0x12, 0x03, 0x14, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x14, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x14, 0x14,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x15, 0x02, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x15, 0x02, 0x14, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x15, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x15, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0d, 0x03, 0x12, 0x03, 0x15, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0e, 0x12,
    0x03, 0x16, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x04, 0x12, 0x04, 0x16,
    0x02, 0x15, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x16, 0x02,
    0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x16, 0x08, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x16, 0x16, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x18, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0f, 0x04, 0x12, 0x04, 0x18, 0x02, 0x16, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0f, 0x06, 0x12, 0x03, 0x18, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x01,
    0x12, 0x03, 0x18, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x03, 0x12, 0x03,
    0x18, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x10, 0x12, 0x03, 0x1a, 0x02, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x10, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x10, 0x03, 0x12, 0x03, 0x1a, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x11,
    0x12, 0x03, 0x1b, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x04, 0x12, 0x03,
    0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x05, 0x12, 0x03, 0x1b, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x03, 0x12, 0x03, 0x1b, 0x29, 0x2c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x12, 0x12, 0x03, 0x1c, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x12, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12,
    0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12, 0x01, 0x12,
    0x03, 0x1c, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12, 0x03, 0x12, 0x03, 0x1c,
    0x28, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x13, 0x12, 0x03, 0x1d, 0x02, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x13, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x13, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x13, 0x01, 0x12, 0x03, 0x1d, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x13, 0x03, 0x12, 0x03, 0x1d, 0x28, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x14, 0x12,
    0x03, 0x1e, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x04, 0x12, 0x03, 0x1e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x03, 0x12, 0x03, 0x1e, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x15, 0x12, 0x03, 0x1f, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x15, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x05,
    0x12, 0x03, 0x1f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x01, 0x12, 0x03,
    0x1f, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x03, 0x12, 0x03, 0x1f, 0x2a,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x16, 0x12, 0x03, 0x20, 0x02, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x16, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x16, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x16, 0x01, 0x12, 0x03, 0x20, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x16,
    0x03, 0x12, 0x03, 0x20, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x17, 0x12, 0x03,
    0x21, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x04, 0x12, 0x03, 0x21, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x01, 0x12, 0x03, 0x21, 0x12, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x17, 0x03, 0x12, 0x03, 0x21, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x18, 0x12, 0x03, 0x22, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18,
    0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x05, 0x12,
    0x03, 0x22, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x01, 0x12, 0x03, 0x22,
    0x13, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x03, 0x12, 0x03, 0x22, 0x2c, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x19, 0x12, 0x03, 0x23, 0x02, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x19, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x19, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x19, 0x01, 0x12, 0x03, 0x23, 0x13, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x19, 0x03,
    0x12, 0x03, 0x23, 0x2c, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1a, 0x12, 0x03, 0x24,
    0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x24, 0x14, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x1a, 0x03, 0x12, 0x03, 0x24, 0x2e, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x1b, 0x12, 0x03, 0x25, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x04,
    0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x05, 0x12, 0x03,
    0x25, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x01, 0x12, 0x03, 0x25, 0x14,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x03, 0x12, 0x03, 0x25, 0x2e, 0x31, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1c, 0x12, 0x03, 0x26, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x1c, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x1c, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1c,
    0x01, 0x12, 0x03, 0x26, 0x10, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1c, 0x03, 0x12,
    0x03, 0x26, 0x26, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1d, 0x12, 0x03, 0x27, 0x02,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x27, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x1d, 0x03, 0x12, 0x03, 0x27, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x1e, 0x12, 0x03, 0x28, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x04, 0x12,
    0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x05, 0x12, 0x03, 0x28,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x01, 0x12, 0x03, 0x28, 0x11, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x03, 0x12, 0x03, 0x28, 0x28, 0x2b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x31, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x31, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x32, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x32, 0x02,
    0x31, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x09, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x1c, 0x1d, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
