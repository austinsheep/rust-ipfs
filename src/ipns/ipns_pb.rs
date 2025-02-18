// This file is generated by rust-protobuf 2.0.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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
#![allow(bare_trait_objects)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct IpnsEntry {
    // message fields
    pub value: ::std::vec::Vec<u8>,
    pub signature: ::std::vec::Vec<u8>,
    pub validityType: IpnsEntry_ValidityType,
    pub validity: ::std::vec::Vec<u8>,
    pub sequence: u64,
    pub ttl: u64,
    pub pubKey: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl IpnsEntry {
    pub fn new() -> IpnsEntry {
        ::std::default::Default::default()
    }

    // bytes value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    // bytes signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    // .IpnsEntry.ValidityType validityType = 3;

    pub fn clear_validityType(&mut self) {
        self.validityType = IpnsEntry_ValidityType::EOL;
    }

    // Param is passed by value, moved
    pub fn set_validityType(&mut self, v: IpnsEntry_ValidityType) {
        self.validityType = v;
    }

    pub fn get_validityType(&self) -> IpnsEntry_ValidityType {
        self.validityType
    }

    // bytes validity = 4;

    pub fn clear_validity(&mut self) {
        self.validity.clear();
    }

    // Param is passed by value, moved
    pub fn set_validity(&mut self, v: ::std::vec::Vec<u8>) {
        self.validity = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_validity(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.validity
    }

    // Take field
    pub fn take_validity(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.validity, ::std::vec::Vec::new())
    }

    pub fn get_validity(&self) -> &[u8] {
        &self.validity
    }

    // uint64 sequence = 5;

    pub fn clear_sequence(&mut self) {
        self.sequence = 0;
    }

    // Param is passed by value, moved
    pub fn set_sequence(&mut self, v: u64) {
        self.sequence = v;
    }

    pub fn get_sequence(&self) -> u64 {
        self.sequence
    }

    // uint64 ttl = 6;

    pub fn clear_ttl(&mut self) {
        self.ttl = 0;
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: u64) {
        self.ttl = v;
    }

    pub fn get_ttl(&self) -> u64 {
        self.ttl
    }

    // bytes pubKey = 7;

    pub fn clear_pubKey(&mut self) {
        self.pubKey.clear();
    }

    // Param is passed by value, moved
    pub fn set_pubKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.pubKey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pubKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.pubKey
    }

    // Take field
    pub fn take_pubKey(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.pubKey, ::std::vec::Vec::new())
    }

    pub fn get_pubKey(&self) -> &[u8] {
        &self.pubKey
    }
}

impl ::protobuf::Message for IpnsEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.validityType, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.validity)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.sequence = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ttl = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.pubKey)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.value);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.signature);
        }
        if self.validityType != IpnsEntry_ValidityType::EOL {
            my_size += ::protobuf::rt::enum_size(3, self.validityType);
        }
        if !self.validity.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.validity);
        }
        if self.sequence != 0 {
            my_size += ::protobuf::rt::value_size(5, self.sequence, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ttl != 0 {
            my_size += ::protobuf::rt::value_size(6, self.ttl, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.pubKey.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.pubKey);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_bytes(1, &self.value)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(2, &self.signature)?;
        }
        if self.validityType != IpnsEntry_ValidityType::EOL {
            os.write_enum(3, self.validityType.value())?;
        }
        if !self.validity.is_empty() {
            os.write_bytes(4, &self.validity)?;
        }
        if self.sequence != 0 {
            os.write_uint64(5, self.sequence)?;
        }
        if self.ttl != 0 {
            os.write_uint64(6, self.ttl)?;
        }
        if !self.pubKey.is_empty() {
            os.write_bytes(7, &self.pubKey)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> IpnsEntry {
        IpnsEntry::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    |m: &IpnsEntry| { &m.value },
                    |m: &mut IpnsEntry| { &mut m.value },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    |m: &IpnsEntry| { &m.signature },
                    |m: &mut IpnsEntry| { &mut m.signature },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<IpnsEntry_ValidityType>>(
                    "validityType",
                    |m: &IpnsEntry| { &m.validityType },
                    |m: &mut IpnsEntry| { &mut m.validityType },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "validity",
                    |m: &IpnsEntry| { &m.validity },
                    |m: &mut IpnsEntry| { &mut m.validity },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sequence",
                    |m: &IpnsEntry| { &m.sequence },
                    |m: &mut IpnsEntry| { &mut m.sequence },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ttl",
                    |m: &IpnsEntry| { &m.ttl },
                    |m: &mut IpnsEntry| { &mut m.ttl },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "pubKey",
                    |m: &IpnsEntry| { &m.pubKey },
                    |m: &mut IpnsEntry| { &mut m.pubKey },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IpnsEntry>(
                    "IpnsEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static IpnsEntry {
        static mut instance: ::protobuf::lazy::Lazy<IpnsEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IpnsEntry,
        };
        unsafe {
            instance.get(IpnsEntry::new)
        }
    }
}

impl ::protobuf::Clear for IpnsEntry {
    fn clear(&mut self) {
        self.clear_value();
        self.clear_signature();
        self.clear_validityType();
        self.clear_validity();
        self.clear_sequence();
        self.clear_ttl();
        self.clear_pubKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IpnsEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IpnsEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum IpnsEntry_ValidityType {
    EOL = 0,
}

impl ::protobuf::ProtobufEnum for IpnsEntry_ValidityType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<IpnsEntry_ValidityType> {
        match value {
            0 => ::std::option::Option::Some(IpnsEntry_ValidityType::EOL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [IpnsEntry_ValidityType] = &[
            IpnsEntry_ValidityType::EOL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("IpnsEntry_ValidityType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for IpnsEntry_ValidityType {
}

impl ::std::default::Default for IpnsEntry_ValidityType {
    fn default() -> Self {
        IpnsEntry_ValidityType::EOL
    }
}

impl ::protobuf::reflect::ProtobufValue for IpnsEntry_ValidityType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16src/ipns/ipns_pb.proto\"\xf7\x01\n\tIpnsEntry\x12\x14\n\x05value\
    \x18\x01\x20\x01(\x0cR\x05value\x12\x1c\n\tsignature\x18\x02\x20\x01(\
    \x0cR\tsignature\x12;\n\x0cvalidityType\x18\x03\x20\x01(\x0e2\x17.IpnsEn\
    try.ValidityTypeR\x0cvalidityType\x12\x1a\n\x08validity\x18\x04\x20\x01(\
    \x0cR\x08validity\x12\x1a\n\x08sequence\x18\x05\x20\x01(\x04R\x08sequenc\
    e\x12\x10\n\x03ttl\x18\x06\x20\x01(\x04R\x03ttl\x12\x16\n\x06pubKey\x18\
    \x07\x20\x01(\x0cR\x06pubKey\"\x17\n\x0cValidityType\x12\x07\n\x03EOL\
    \x10\0J\xd9\x07\n\x06\x12\x04\0\0\x12\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\n\n\x02\x04\0\x12\x04\x02\0\x12\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x02\x08\x11\n\x0c\n\x04\x04\0\x04\0\x12\x04\x03\x02\x05\x03\n\x0c\n\x05\
    \x04\0\x04\0\x01\x12\x03\x03\x07\x13\nD\n\x06\x04\0\x04\0\x02\0\x12\x03\
    \x04\x04\x0c\"5\x20setting\x20an\x20EOL\x20says\x20\"this\x20record\x20i\
    s\x20valid\x20until...\"\n\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\x04\
    \x04\x07\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03\x04\n\x0b\n\x17\n\x04\
    \x04\0\x02\0\x12\x03\x06\x02\x12\"\n\x20required\n\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x06\x02\x05\x03\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x02\
    \x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x08\r\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03\x06\x10\x11\n\x17\n\x04\x04\0\x02\x01\x12\x03\x07\x02\x16\
    \"\n\x20required\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x07\x02\x06\x12\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x07\x08\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07\
    \x14\x15\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x08\x02\x20\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\x08\x02\x07\x16\n\x0c\n\x05\x04\0\x02\x02\x06\x12\
    \x03\x08\x02\x0e\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x08\x0f\x1b\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\x08\x1e\x1f\n\x0b\n\x04\x04\0\x02\x03\
    \x12\x03\t\x02\x15\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\t\x02\x08\x20\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\t\x02\x07\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03\t\x08\x10\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\t\x13\x14\n\
    \x0b\n\x04\x04\0\x02\x04\x12\x03\n\x02\x16\n\r\n\x05\x04\0\x02\x04\x04\
    \x12\x04\n\x02\t\x15\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\n\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x04\x01\x12\x03\n\t\x11\n\x0c\n\x05\x04\0\x02\x04\
    \x03\x12\x03\n\x14\x15\n\x0b\n\x04\x04\0\x02\x05\x12\x03\x0b\x02\x11\n\r\
    \n\x05\x04\0\x02\x05\x04\x12\x04\x0b\x02\n\x16\n\x0c\n\x05\x04\0\x02\x05\
    \x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0b\t\x0c\
    \n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x0b\x0f\x10\n\xb5\x02\n\x04\x04\0\
    \x02\x06\x12\x03\x11\x02\x13\x1a\xa7\x02\x20in\x20order\x20for\x20nodes\
    \x20to\x20properly\x20validate\x20a\x20record\x20upon\x20receipt,\x20the\
    y\n\x20need\x20the\x20public\x20key\x20associated\x20with\x20it.\x20For\
    \x20old\x20RSA\x20keys,\x20its\x20easiest\n\x20if\x20we\x20just\x20send\
    \x20this\x20as\x20part\x20of\x20the\x20record\x20itself.\x20For\x20newer\
    \x20ed25519\n\x20keys,\x20the\x20public\x20key\x20can\x20be\x20embedded\
    \x20in\x20the\x20peerID,\x20making\x20this\x20field\n\x20unnecessary.\n\
    \n\r\n\x05\x04\0\x02\x06\x04\x12\x04\x11\x02\x0b\x11\n\x0c\n\x05\x04\0\
    \x02\x06\x05\x12\x03\x11\x02\x07\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\
    \x11\x08\x0e\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x11\x11\x12b\x06proto\
    3\
";

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
