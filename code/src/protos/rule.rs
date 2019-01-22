// This file is generated by rust-protobuf 2.2.4. Do not edit
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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Rule {
    // message fields
    pub field_type: Rule_RuleType,
    pub value: ::bytes::Bytes,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Rule {
    pub fn new() -> Rule {
        ::std::default::Default::default()
    }

    // .Rule.RuleType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = Rule_RuleType::RULE_UNSET;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Rule_RuleType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Rule_RuleType {
        self.field_type
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::bytes::Bytes) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.value, ::bytes::Bytes::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
}

impl ::protobuf::Message for Rule {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.value)?;
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
        if self.field_type != Rule_RuleType::RULE_UNSET {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != Rule_RuleType::RULE_UNSET {
            os.write_enum(1, self.field_type.value())?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
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

    fn new() -> Rule {
        Rule::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Rule_RuleType>>(
                    "type",
                    |m: &Rule| { &m.field_type },
                    |m: &mut Rule| { &mut m.field_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "value",
                    |m: &Rule| { &m.value },
                    |m: &mut Rule| { &mut m.value },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Rule>(
                    "Rule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Rule {
        static mut instance: ::protobuf::lazy::Lazy<Rule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Rule,
        };
        unsafe {
            instance.get(Rule::new)
        }
    }
}

impl ::protobuf::Clear for Rule {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Rule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Rule_RuleType {
    RULE_UNSET = 0,
    OWNER_HOLDINGS_INFINITE = 100,
    ALL_HOLDINGS_INFINITE = 101,
    NOT_TRANSFERABLE = 102,
    REQUIRE_SOURCE_TYPES = 103,
    REQUIRE_TARGET_TYPES = 104,
    REQUIRE_SOURCE_QUANTITIES = 105,
    REQUIRE_TARGET_QUANTITIES = 106,
    EXCHANGE_ONCE = 200,
    EXCHANGE_ONCE_PER_ACCOUNT = 201,
    EXCHANGE_LIMITED_TO_ACCOUNTS = 202,
}

impl ::protobuf::ProtobufEnum for Rule_RuleType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Rule_RuleType> {
        match value {
            0 => ::std::option::Option::Some(Rule_RuleType::RULE_UNSET),
            100 => ::std::option::Option::Some(Rule_RuleType::OWNER_HOLDINGS_INFINITE),
            101 => ::std::option::Option::Some(Rule_RuleType::ALL_HOLDINGS_INFINITE),
            102 => ::std::option::Option::Some(Rule_RuleType::NOT_TRANSFERABLE),
            103 => ::std::option::Option::Some(Rule_RuleType::REQUIRE_SOURCE_TYPES),
            104 => ::std::option::Option::Some(Rule_RuleType::REQUIRE_TARGET_TYPES),
            105 => ::std::option::Option::Some(Rule_RuleType::REQUIRE_SOURCE_QUANTITIES),
            106 => ::std::option::Option::Some(Rule_RuleType::REQUIRE_TARGET_QUANTITIES),
            200 => ::std::option::Option::Some(Rule_RuleType::EXCHANGE_ONCE),
            201 => ::std::option::Option::Some(Rule_RuleType::EXCHANGE_ONCE_PER_ACCOUNT),
            202 => ::std::option::Option::Some(Rule_RuleType::EXCHANGE_LIMITED_TO_ACCOUNTS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Rule_RuleType] = &[
            Rule_RuleType::RULE_UNSET,
            Rule_RuleType::OWNER_HOLDINGS_INFINITE,
            Rule_RuleType::ALL_HOLDINGS_INFINITE,
            Rule_RuleType::NOT_TRANSFERABLE,
            Rule_RuleType::REQUIRE_SOURCE_TYPES,
            Rule_RuleType::REQUIRE_TARGET_TYPES,
            Rule_RuleType::REQUIRE_SOURCE_QUANTITIES,
            Rule_RuleType::REQUIRE_TARGET_QUANTITIES,
            Rule_RuleType::EXCHANGE_ONCE,
            Rule_RuleType::EXCHANGE_ONCE_PER_ACCOUNT,
            Rule_RuleType::EXCHANGE_LIMITED_TO_ACCOUNTS,
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
                ::protobuf::reflect::EnumDescriptor::new("Rule_RuleType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Rule_RuleType {
}

impl ::std::default::Default for Rule_RuleType {
    fn default() -> Self {
        Rule_RuleType::RULE_UNSET
    }
}

impl ::protobuf::reflect::ProtobufValue for Rule_RuleType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nrule.proto\"\xf4\x02\n\x04Rule\x12\"\n\x04type\x18\x01\x20\x01(\x0e2\
    \x0e.Rule.RuleTypeR\x04type\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05\
    value\"\xb1\x02\n\x08RuleType\x12\x0e\n\nRULE_UNSET\x10\0\x12\x1b\n\x17O\
    WNER_HOLDINGS_INFINITE\x10d\x12\x19\n\x15ALL_HOLDINGS_INFINITE\x10e\x12\
    \x14\n\x10NOT_TRANSFERABLE\x10f\x12\x18\n\x14REQUIRE_SOURCE_TYPES\x10g\
    \x12\x18\n\x14REQUIRE_TARGET_TYPES\x10h\x12\x1d\n\x19REQUIRE_SOURCE_QUAN\
    TITIES\x10i\x12\x1d\n\x19REQUIRE_TARGET_QUANTITIES\x10j\x12\x12\n\rEXCHA\
    NGE_ONCE\x10\xc8\x01\x12\x1e\n\x19EXCHANGE_ONCE_PER_ACCOUNT\x10\xc9\x01\
    \x12!\n\x1cEXCHANGE_LIMITED_TO_ACCOUNTS\x10\xca\x01b\x06proto3\
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
