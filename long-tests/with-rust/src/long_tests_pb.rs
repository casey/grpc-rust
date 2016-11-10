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
pub struct EchoRequest {
    // message fields
    pub payload: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EchoRequest {}

impl EchoRequest {
    pub fn new() -> EchoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EchoRequest {
        static mut instance: ::protobuf::lazy::Lazy<EchoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EchoRequest,
        };
        unsafe {
            instance.get(EchoRequest::new)
        }
    }

    // string payload = 1;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::string::String) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::string::String {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payload, ::std::string::String::new())
    }

    pub fn get_payload(&self) -> &str {
        &self.payload
    }

    fn get_payload_for_reflect(&self) -> &::std::string::String {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payload
    }
}

impl ::protobuf::Message for EchoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payload));
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
        if self.payload != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(1, &self.payload);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.payload != ::std::string::String::new() {
            try!(os.write_string(1, &self.payload));
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
        ::std::any::TypeId::of::<EchoRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EchoRequest {
    fn new() -> EchoRequest {
        EchoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<EchoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payload",
                    EchoRequest::get_payload_for_reflect,
                    EchoRequest::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EchoRequest>(
                    "EchoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EchoRequest {
    fn clear(&mut self) {
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EchoRequest {
    fn eq(&self, other: &EchoRequest) -> bool {
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EchoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EchoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,Default)]
pub struct EchoResponse {
    // message fields
    pub payload: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EchoResponse {}

impl EchoResponse {
    pub fn new() -> EchoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EchoResponse {
        static mut instance: ::protobuf::lazy::Lazy<EchoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EchoResponse,
        };
        unsafe {
            instance.get(EchoResponse::new)
        }
    }

    // string payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::string::String) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::string::String {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.payload, ::std::string::String::new())
    }

    pub fn get_payload(&self) -> &str {
        &self.payload
    }

    fn get_payload_for_reflect(&self) -> &::std::string::String {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.payload
    }
}

impl ::protobuf::Message for EchoResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.payload));
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
        if self.payload != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(2, &self.payload);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.payload != ::std::string::String::new() {
            try!(os.write_string(2, &self.payload));
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
        ::std::any::TypeId::of::<EchoResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EchoResponse {
    fn new() -> EchoResponse {
        EchoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EchoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "payload",
                    EchoResponse::get_payload_for_reflect,
                    EchoResponse::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EchoResponse>(
                    "EchoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EchoResponse {
    fn clear(&mut self) {
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EchoResponse {
    fn eq(&self, other: &EchoResponse) -> bool {
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EchoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EchoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,Default)]
pub struct CharCountRequest {
    // message fields
    pub part: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CharCountRequest {}

impl CharCountRequest {
    pub fn new() -> CharCountRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CharCountRequest {
        static mut instance: ::protobuf::lazy::Lazy<CharCountRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CharCountRequest,
        };
        unsafe {
            instance.get(CharCountRequest::new)
        }
    }

    // string part = 1;

    pub fn clear_part(&mut self) {
        self.part.clear();
    }

    // Param is passed by value, moved
    pub fn set_part(&mut self, v: ::std::string::String) {
        self.part = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_part(&mut self) -> &mut ::std::string::String {
        &mut self.part
    }

    // Take field
    pub fn take_part(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.part, ::std::string::String::new())
    }

    pub fn get_part(&self) -> &str {
        &self.part
    }

    fn get_part_for_reflect(&self) -> &::std::string::String {
        &self.part
    }

    fn mut_part_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.part
    }
}

impl ::protobuf::Message for CharCountRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.part));
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
        if self.part != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(1, &self.part);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.part != ::std::string::String::new() {
            try!(os.write_string(1, &self.part));
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
        ::std::any::TypeId::of::<CharCountRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CharCountRequest {
    fn new() -> CharCountRequest {
        CharCountRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CharCountRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "part",
                    CharCountRequest::get_part_for_reflect,
                    CharCountRequest::mut_part_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CharCountRequest>(
                    "CharCountRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CharCountRequest {
    fn clear(&mut self) {
        self.clear_part();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CharCountRequest {
    fn eq(&self, other: &CharCountRequest) -> bool {
        self.part == other.part &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CharCountRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CharCountRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,Default)]
pub struct CharCountResponse {
    // message fields
    pub char_count: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CharCountResponse {}

impl CharCountResponse {
    pub fn new() -> CharCountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CharCountResponse {
        static mut instance: ::protobuf::lazy::Lazy<CharCountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CharCountResponse,
        };
        unsafe {
            instance.get(CharCountResponse::new)
        }
    }

    // uint64 char_count = 1;

    pub fn clear_char_count(&mut self) {
        self.char_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_char_count(&mut self, v: u64) {
        self.char_count = v;
    }

    pub fn get_char_count(&self) -> u64 {
        self.char_count
    }

    fn get_char_count_for_reflect(&self) -> &u64 {
        &self.char_count
    }

    fn mut_char_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.char_count
    }
}

impl ::protobuf::Message for CharCountResponse {
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
                    let tmp = try!(is.read_uint64());
                    self.char_count = tmp;
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
        if self.char_count != 0 {
            my_size += ::protobuf::rt::value_size(1, self.char_count, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.char_count != 0 {
            try!(os.write_uint64(1, self.char_count));
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
        ::std::any::TypeId::of::<CharCountResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CharCountResponse {
    fn new() -> CharCountResponse {
        CharCountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CharCountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "char_count",
                    CharCountResponse::get_char_count_for_reflect,
                    CharCountResponse::mut_char_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CharCountResponse>(
                    "CharCountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CharCountResponse {
    fn clear(&mut self) {
        self.clear_char_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CharCountResponse {
    fn eq(&self, other: &CharCountResponse) -> bool {
        self.char_count == other.char_count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CharCountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CharCountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x6c, 0x6f, 0x6e, 0x67, 0x5f, 0x74, 0x65, 0x73, 0x74, 0x73, 0x5f, 0x70, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x27, 0x0a, 0x0b, 0x45, 0x63, 0x68, 0x6f, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22, 0x28,
    0x0a, 0x0c, 0x45, 0x63, 0x68, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18,
    0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22, 0x26, 0x0a, 0x10, 0x43, 0x68, 0x61, 0x72,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04,
    0x70, 0x61, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x72, 0x74,
    0x22, 0x32, 0x0a, 0x11, 0x43, 0x68, 0x61, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x72, 0x5f, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x68, 0x61, 0x72, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x32, 0x6b, 0x0a, 0x09, 0x4c, 0x6f, 0x6e, 0x67, 0x54, 0x65, 0x73, 0x74,
    0x73, 0x12, 0x25, 0x0a, 0x04, 0x65, 0x63, 0x68, 0x6f, 0x12, 0x0c, 0x2e, 0x45, 0x63, 0x68, 0x6f,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x0d, 0x2e, 0x45, 0x63, 0x68, 0x6f, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x37, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x72,
    0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x11, 0x2e, 0x43, 0x68, 0x61, 0x72, 0x43, 0x6f, 0x75,
    0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x12, 0x2e, 0x43, 0x68, 0x61, 0x72,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x28,
    0x01, 0x4a, 0x9e, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x15, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00,
    0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x03, 0x04, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x03, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x03, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x06, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x07, 0x04, 0x06, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x07, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x07, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07,
    0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0a, 0x00, 0x0c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x0b, 0x04, 0x0a, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x12, 0x13,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x0f, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x0f, 0x04, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x0b, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x18, 0x19, 0x0a, 0x0a,
    0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x12, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00,
    0x01, 0x12, 0x03, 0x12, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x13, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x13, 0x0e, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x24, 0x30, 0x0a, 0x0b, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x04, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x14, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x14, 0x1b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14,
    0x36, 0x47, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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