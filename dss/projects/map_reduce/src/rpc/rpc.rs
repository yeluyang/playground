// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `assets/pb/rpc.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct PingRequest {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PingRequest {
    fn default() -> &'a PingRequest {
        <PingRequest as ::protobuf::Message>::default_instance()
    }
}

impl PingRequest {
    pub fn new() -> PingRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for PingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PingRequest {
        PingRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<PingRequest>(
                    "PingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PingRequest {
        static mut instance: ::protobuf::lazy::Lazy<PingRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(PingRequest::new)
        }
    }
}

impl ::protobuf::Clear for PingRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingResponse {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PingResponse {
    fn default() -> &'a PingResponse {
        <PingResponse as ::protobuf::Message>::default_instance()
    }
}

impl PingResponse {
    pub fn new() -> PingResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for PingResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PingResponse {
        PingResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<PingResponse>(
                    "PingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PingResponse {
        static mut instance: ::protobuf::lazy::Lazy<PingResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(PingResponse::new)
        }
    }
}

impl ::protobuf::Clear for PingResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TaskGetRequest {
    // message fields
    pub host: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TaskGetRequest {
    fn default() -> &'a TaskGetRequest {
        <TaskGetRequest as ::protobuf::Message>::default_instance()
    }
}

impl TaskGetRequest {
    pub fn new() -> TaskGetRequest {
        ::std::default::Default::default()
    }

    // string host = 1;


    pub fn get_host(&self) -> &str {
        &self.host
    }
    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host(&mut self) -> &mut ::std::string::String {
        &mut self.host
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.host, ::std::string::String::new())
    }
}

impl ::protobuf::Message for TaskGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.host)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.host.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.host);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.host.is_empty() {
            os.write_string(1, &self.host)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TaskGetRequest {
        TaskGetRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host",
                    |m: &TaskGetRequest| { &m.host },
                    |m: &mut TaskGetRequest| { &mut m.host },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<TaskGetRequest>(
                    "TaskGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TaskGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<TaskGetRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(TaskGetRequest::new)
        }
    }
}

impl ::protobuf::Clear for TaskGetRequest {
    fn clear(&mut self) {
        self.host.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TaskGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TaskGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TaskGetResponse {
    // message fields
    pub input_path: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TaskGetResponse {
    fn default() -> &'a TaskGetResponse {
        <TaskGetResponse as ::protobuf::Message>::default_instance()
    }
}

impl TaskGetResponse {
    pub fn new() -> TaskGetResponse {
        ::std::default::Default::default()
    }

    // string input_path = 1;


    pub fn get_input_path(&self) -> &str {
        &self.input_path
    }
    pub fn clear_input_path(&mut self) {
        self.input_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_input_path(&mut self, v: ::std::string::String) {
        self.input_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input_path(&mut self) -> &mut ::std::string::String {
        &mut self.input_path
    }

    // Take field
    pub fn take_input_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.input_path, ::std::string::String::new())
    }
}

impl ::protobuf::Message for TaskGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.input_path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.input_path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.input_path);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.input_path.is_empty() {
            os.write_string(1, &self.input_path)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TaskGetResponse {
        TaskGetResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "input_path",
                    |m: &TaskGetResponse| { &m.input_path },
                    |m: &mut TaskGetResponse| { &mut m.input_path },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<TaskGetResponse>(
                    "TaskGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TaskGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<TaskGetResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(TaskGetResponse::new)
        }
    }
}

impl ::protobuf::Clear for TaskGetResponse {
    fn clear(&mut self) {
        self.input_path.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TaskGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TaskGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13assets/pb/rpc.proto\"\r\n\x0bPingRequest\"\x0e\n\x0cPingResponse\"\
    $\n\x0eTaskGetRequest\x12\x12\n\x04host\x18\x01\x20\x01(\tR\x04host\"0\n\
    \x0fTaskGetResponse\x12\x1d\n\ninput_path\x18\x01\x20\x01(\tR\tinputPath\
    2b\n\tMasterRPC\x12%\n\x04Ping\x12\x0c.PingRequest\x1a\r.PingResponse\"\
    \0\x12.\n\x07TaskGet\x12\x0f.TaskGetRequest\x1a\x10.TaskGetResponse\"\0J\
    \xe4\x02\n\x06\x12\x04\0\0\x11\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\
    \x02\x04\0\x12\x03\x02\0\x16\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x13\n\t\
    \n\x02\x04\x01\x12\x03\x04\0\x17\n\n\n\x03\x04\x01\x01\x12\x03\x04\x08\
    \x14\n\n\n\x02\x04\x02\x12\x04\x06\0\x08\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\x06\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x07\x04\x14\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03\x07\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03\x07\x0b\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x07\x12\x13\n\
    \n\n\x02\x04\x03\x12\x04\n\0\x0c\x01\n\n\n\x03\x04\x03\x01\x12\x03\n\x08\
    \x17\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x0b\x04\x1a\n\x0c\n\x05\x04\x03\
    \x02\0\x05\x12\x03\x0b\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x0b\
    \x0b\x15\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x0b\x18\x19\n\n\n\x02\x06\
    \0\x12\x04\x0e\0\x11\x01\n\n\n\x03\x06\0\x01\x12\x03\x0e\x08\x11\n\x0b\n\
    \x04\x06\0\x02\0\x12\x03\x0f\x043\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    \x0f\x08\x0c\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x0f\r\x18\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x03\x0f#/\n\x0b\n\x04\x06\0\x02\x01\x12\x03\x10\x04\
    <\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x10\x08\x0f\n\x0c\n\x05\x06\0\
    \x02\x01\x02\x12\x03\x10\x10\x1e\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\
    \x10)8b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

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
