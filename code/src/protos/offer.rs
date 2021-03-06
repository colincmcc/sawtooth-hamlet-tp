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
pub struct Offer {
    // message fields
    pub id: ::std::string::String,
    pub label: ::std::string::String,
    pub description: ::std::string::String,
    pub owners: ::protobuf::RepeatedField<::std::string::String>,
    pub source: ::std::string::String,
    pub source_quantity: i64,
    pub target: ::std::string::String,
    pub target_quantity: i64,
    pub rules: ::protobuf::RepeatedField<super::rule::Rule>,
    pub status: Offer_Status,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Offer {
    pub fn new() -> Offer {
        ::std::default::Default::default()
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    // string label = 2;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.label, ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    // string description = 3;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    // repeated string owners = 4;

    pub fn clear_owners(&mut self) {
        self.owners.clear();
    }

    // Param is passed by value, moved
    pub fn set_owners(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.owners = v;
    }

    // Mutable pointer to the field.
    pub fn mut_owners(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.owners
    }

    // Take field
    pub fn take_owners(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.owners, ::protobuf::RepeatedField::new())
    }

    pub fn get_owners(&self) -> &[::std::string::String] {
        &self.owners
    }

    // string source = 5;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        &mut self.source
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.source, ::std::string::String::new())
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    // sint64 source_quantity = 6;

    pub fn clear_source_quantity(&mut self) {
        self.source_quantity = 0;
    }

    // Param is passed by value, moved
    pub fn set_source_quantity(&mut self, v: i64) {
        self.source_quantity = v;
    }

    pub fn get_source_quantity(&self) -> i64 {
        self.source_quantity
    }

    // string target = 7;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        &mut self.target
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.target, ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        &self.target
    }

    // sint64 target_quantity = 8;

    pub fn clear_target_quantity(&mut self) {
        self.target_quantity = 0;
    }

    // Param is passed by value, moved
    pub fn set_target_quantity(&mut self, v: i64) {
        self.target_quantity = v;
    }

    pub fn get_target_quantity(&self) -> i64 {
        self.target_quantity
    }

    // repeated .Rule rules = 9;

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<super::rule::Rule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<super::rule::Rule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<super::rule::Rule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    pub fn get_rules(&self) -> &[super::rule::Rule] {
        &self.rules
    }

    // .Offer.Status status = 10;

    pub fn clear_status(&mut self) {
        self.status = Offer_Status::STATUS_UNSET;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Offer_Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> Offer_Status {
        self.status
    }
}

impl ::protobuf::Message for Offer {
    fn is_initialized(&self) -> bool {
        for v in &self.rules {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.label)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.owners)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.source)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.source_quantity = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.target)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.target_quantity = tmp;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
                },
                10 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 10, &mut self.unknown_fields)?
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.label.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.label);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        for value in &self.owners {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if !self.source.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.source);
        }
        if self.source_quantity != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(6, self.source_quantity);
        }
        if !self.target.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.target);
        }
        if self.target_quantity != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, self.target_quantity);
        }
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.status != Offer_Status::STATUS_UNSET {
            my_size += ::protobuf::rt::enum_size(10, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.label.is_empty() {
            os.write_string(2, &self.label)?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        for v in &self.owners {
            os.write_string(4, &v)?;
        };
        if !self.source.is_empty() {
            os.write_string(5, &self.source)?;
        }
        if self.source_quantity != 0 {
            os.write_sint64(6, self.source_quantity)?;
        }
        if !self.target.is_empty() {
            os.write_string(7, &self.target)?;
        }
        if self.target_quantity != 0 {
            os.write_sint64(8, self.target_quantity)?;
        }
        for v in &self.rules {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.status != Offer_Status::STATUS_UNSET {
            os.write_enum(10, self.status.value())?;
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

    fn new() -> Offer {
        Offer::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    |m: &Offer| { &m.id },
                    |m: &mut Offer| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "label",
                    |m: &Offer| { &m.label },
                    |m: &mut Offer| { &mut m.label },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    |m: &Offer| { &m.description },
                    |m: &mut Offer| { &mut m.description },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owners",
                    |m: &Offer| { &m.owners },
                    |m: &mut Offer| { &mut m.owners },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source",
                    |m: &Offer| { &m.source },
                    |m: &mut Offer| { &mut m.source },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "source_quantity",
                    |m: &Offer| { &m.source_quantity },
                    |m: &mut Offer| { &mut m.source_quantity },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    |m: &Offer| { &m.target },
                    |m: &mut Offer| { &mut m.target },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "target_quantity",
                    |m: &Offer| { &m.target_quantity },
                    |m: &mut Offer| { &mut m.target_quantity },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rule::Rule>>(
                    "rules",
                    |m: &Offer| { &m.rules },
                    |m: &mut Offer| { &mut m.rules },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Offer_Status>>(
                    "status",
                    |m: &Offer| { &m.status },
                    |m: &mut Offer| { &mut m.status },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Offer>(
                    "Offer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Offer {
        static mut instance: ::protobuf::lazy::Lazy<Offer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Offer,
        };
        unsafe {
            instance.get(Offer::new)
        }
    }
}

impl ::protobuf::Clear for Offer {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_label();
        self.clear_description();
        self.clear_owners();
        self.clear_source();
        self.clear_source_quantity();
        self.clear_target();
        self.clear_target_quantity();
        self.clear_rules();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Offer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Offer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Offer_Status {
    STATUS_UNSET = 0,
    OPEN = 1,
    CLOSED = 2,
}

impl ::protobuf::ProtobufEnum for Offer_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Offer_Status> {
        match value {
            0 => ::std::option::Option::Some(Offer_Status::STATUS_UNSET),
            1 => ::std::option::Option::Some(Offer_Status::OPEN),
            2 => ::std::option::Option::Some(Offer_Status::CLOSED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Offer_Status] = &[
            Offer_Status::STATUS_UNSET,
            Offer_Status::OPEN,
            Offer_Status::CLOSED,
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
                ::protobuf::reflect::EnumDescriptor::new("Offer_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Offer_Status {
}

impl ::std::default::Default for Offer_Status {
    fn default() -> Self {
        Offer_Status::STATUS_UNSET
    }
}

impl ::protobuf::reflect::ProtobufValue for Offer_Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OfferContainer {
    // message fields
    pub entries: ::protobuf::RepeatedField<Offer>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl OfferContainer {
    pub fn new() -> OfferContainer {
        ::std::default::Default::default()
    }

    // repeated .Offer entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<Offer>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<Offer> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<Offer> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[Offer] {
        &self.entries
    }
}

impl ::protobuf::Message for OfferContainer {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
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
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> OfferContainer {
        OfferContainer::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Offer>>(
                    "entries",
                    |m: &OfferContainer| { &m.entries },
                    |m: &mut OfferContainer| { &mut m.entries },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OfferContainer>(
                    "OfferContainer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OfferContainer {
        static mut instance: ::protobuf::lazy::Lazy<OfferContainer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OfferContainer,
        };
        unsafe {
            instance.get(OfferContainer::new)
        }
    }
}

impl ::protobuf::Clear for OfferContainer {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OfferContainer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OfferContainer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0boffer.proto\x1a\nrule.proto\"\xdf\x02\n\x05Offer\x12\x0e\n\x02id\
    \x18\x01\x20\x01(\tR\x02id\x12\x14\n\x05label\x18\x02\x20\x01(\tR\x05lab\
    el\x12\x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bdescription\x12\x16\n\
    \x06owners\x18\x04\x20\x03(\tR\x06owners\x12\x16\n\x06source\x18\x05\x20\
    \x01(\tR\x06source\x12'\n\x0fsource_quantity\x18\x06\x20\x01(\x12R\x0eso\
    urceQuantity\x12\x16\n\x06target\x18\x07\x20\x01(\tR\x06target\x12'\n\
    \x0ftarget_quantity\x18\x08\x20\x01(\x12R\x0etargetQuantity\x12\x1b\n\
    \x05rules\x18\t\x20\x03(\x0b2\x05.RuleR\x05rules\x12%\n\x06status\x18\n\
    \x20\x01(\x0e2\r.Offer.StatusR\x06status\"0\n\x06Status\x12\x10\n\x0cSTA\
    TUS_UNSET\x10\0\x12\x08\n\x04OPEN\x10\x01\x12\n\n\x06CLOSED\x10\x02\"2\n\
    \x0eOfferContainer\x12\x20\n\x07entries\x18\x01\x20\x03(\x0b2\x06.OfferR\
    \x07entriesb\x06proto3\
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
