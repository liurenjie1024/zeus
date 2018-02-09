// This file is generated. Do not edit
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
pub struct ZeusDBSchema {
    // message fields
    pub name: ::std::string::String,
    pub id: i32,
    pub tables: ::protobuf::RepeatedField<ZeusTableSchema>,
    pub version: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZeusDBSchema {}

impl ZeusDBSchema {
    pub fn new() -> ZeusDBSchema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZeusDBSchema {
        static mut instance: ::protobuf::lazy::Lazy<ZeusDBSchema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZeusDBSchema,
        };
        unsafe {
            instance.get(ZeusDBSchema::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int32 id = 2;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // repeated .ZeusTableSchema tables = 3;

    pub fn clear_tables(&mut self) {
        self.tables.clear();
    }

    // Param is passed by value, moved
    pub fn set_tables(&mut self, v: ::protobuf::RepeatedField<ZeusTableSchema>) {
        self.tables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tables(&mut self) -> &mut ::protobuf::RepeatedField<ZeusTableSchema> {
        &mut self.tables
    }

    // Take field
    pub fn take_tables(&mut self) -> ::protobuf::RepeatedField<ZeusTableSchema> {
        ::std::mem::replace(&mut self.tables, ::protobuf::RepeatedField::new())
    }

    pub fn get_tables(&self) -> &[ZeusTableSchema] {
        &self.tables
    }

    fn get_tables_for_reflect(&self) -> &::protobuf::RepeatedField<ZeusTableSchema> {
        &self.tables
    }

    fn mut_tables_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ZeusTableSchema> {
        &mut self.tables
    }

    // uint32 version = 4;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }
}

impl ::protobuf::Message for ZeusDBSchema {
    fn is_initialized(&self) -> bool {
        for v in &self.tables {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tables)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tables {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.id != 0 {
            os.write_int32(2, self.id)?;
        }
        for v in &self.tables {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.version != 0 {
            os.write_uint32(4, self.version)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ZeusDBSchema {
    fn new() -> ZeusDBSchema {
        ZeusDBSchema::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZeusDBSchema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ZeusDBSchema::get_name_for_reflect,
                    ZeusDBSchema::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    ZeusDBSchema::get_id_for_reflect,
                    ZeusDBSchema::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ZeusTableSchema>>(
                    "tables",
                    ZeusDBSchema::get_tables_for_reflect,
                    ZeusDBSchema::mut_tables_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    ZeusDBSchema::get_version_for_reflect,
                    ZeusDBSchema::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZeusDBSchema>(
                    "ZeusDBSchema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZeusDBSchema {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_tables();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ZeusDBSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZeusDBSchema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZeusTableSchema {
    // message fields
    pub name: ::std::string::String,
    pub id: i32,
    pub fields: ::protobuf::RepeatedField<ZeusColumnSchema>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZeusTableSchema {}

impl ZeusTableSchema {
    pub fn new() -> ZeusTableSchema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZeusTableSchema {
        static mut instance: ::protobuf::lazy::Lazy<ZeusTableSchema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZeusTableSchema,
        };
        unsafe {
            instance.get(ZeusTableSchema::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int32 id = 2;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // repeated .ZeusColumnSchema fields = 3;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::protobuf::RepeatedField<ZeusColumnSchema>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::protobuf::RepeatedField<ZeusColumnSchema> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::protobuf::RepeatedField<ZeusColumnSchema> {
        ::std::mem::replace(&mut self.fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_fields(&self) -> &[ZeusColumnSchema] {
        &self.fields
    }

    fn get_fields_for_reflect(&self) -> &::protobuf::RepeatedField<ZeusColumnSchema> {
        &self.fields
    }

    fn mut_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ZeusColumnSchema> {
        &mut self.fields
    }
}

impl ::protobuf::Message for ZeusTableSchema {
    fn is_initialized(&self) -> bool {
        for v in &self.fields {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fields)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.id != 0 {
            os.write_int32(2, self.id)?;
        }
        for v in &self.fields {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ZeusTableSchema {
    fn new() -> ZeusTableSchema {
        ZeusTableSchema::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZeusTableSchema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ZeusTableSchema::get_name_for_reflect,
                    ZeusTableSchema::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    ZeusTableSchema::get_id_for_reflect,
                    ZeusTableSchema::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ZeusColumnSchema>>(
                    "fields",
                    ZeusTableSchema::get_fields_for_reflect,
                    ZeusTableSchema::mut_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZeusTableSchema>(
                    "ZeusTableSchema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZeusTableSchema {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ZeusTableSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZeusTableSchema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZeusColumnSchema {
    // message fields
    pub name: ::std::string::String,
    pub id: i32,
    pub field_type: FieldType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZeusColumnSchema {}

impl ZeusColumnSchema {
    pub fn new() -> ZeusColumnSchema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZeusColumnSchema {
        static mut instance: ::protobuf::lazy::Lazy<ZeusColumnSchema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZeusColumnSchema,
        };
        unsafe {
            instance.get(ZeusColumnSchema::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int32 id = 2;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // .FieldType field_type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = FieldType::STRING;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: FieldType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> FieldType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &FieldType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut FieldType {
        &mut self.field_type
    }
}

impl ::protobuf::Message for ZeusColumnSchema {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.field_type != FieldType::STRING {
            my_size += ::protobuf::rt::enum_size(3, self.field_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.id != 0 {
            os.write_int32(2, self.id)?;
        }
        if self.field_type != FieldType::STRING {
            os.write_enum(3, self.field_type.value())?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ZeusColumnSchema {
    fn new() -> ZeusColumnSchema {
        ZeusColumnSchema::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZeusColumnSchema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ZeusColumnSchema::get_name_for_reflect,
                    ZeusColumnSchema::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    ZeusColumnSchema::get_id_for_reflect,
                    ZeusColumnSchema::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FieldType>>(
                    "field_type",
                    ZeusColumnSchema::get_field_type_for_reflect,
                    ZeusColumnSchema::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZeusColumnSchema>(
                    "ZeusColumnSchema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZeusColumnSchema {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ZeusColumnSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZeusColumnSchema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetSchemaRequest {
    // message fields
    pub db_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetSchemaRequest {}

impl GetSchemaRequest {
    pub fn new() -> GetSchemaRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetSchemaRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetSchemaRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetSchemaRequest,
        };
        unsafe {
            instance.get(GetSchemaRequest::new)
        }
    }

    // string db_name = 1;

    pub fn clear_db_name(&mut self) {
        self.db_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_db_name(&mut self, v: ::std::string::String) {
        self.db_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_db_name(&mut self) -> &mut ::std::string::String {
        &mut self.db_name
    }

    // Take field
    pub fn take_db_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.db_name, ::std::string::String::new())
    }

    pub fn get_db_name(&self) -> &str {
        &self.db_name
    }

    fn get_db_name_for_reflect(&self) -> &::std::string::String {
        &self.db_name
    }

    fn mut_db_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.db_name
    }
}

impl ::protobuf::Message for GetSchemaRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.db_name)?;
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
        if !self.db_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.db_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.db_name.is_empty() {
            os.write_string(1, &self.db_name)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetSchemaRequest {
    fn new() -> GetSchemaRequest {
        GetSchemaRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetSchemaRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "db_name",
                    GetSchemaRequest::get_db_name_for_reflect,
                    GetSchemaRequest::mut_db_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetSchemaRequest>(
                    "GetSchemaRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetSchemaRequest {
    fn clear(&mut self) {
        self.clear_db_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetSchemaRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSchemaRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetSchemaResponse {
    // message fields
    pub db_schema: ::protobuf::SingularPtrField<ZeusDBSchema>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetSchemaResponse {}

impl GetSchemaResponse {
    pub fn new() -> GetSchemaResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetSchemaResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetSchemaResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetSchemaResponse,
        };
        unsafe {
            instance.get(GetSchemaResponse::new)
        }
    }

    // .ZeusDBSchema db_schema = 1;

    pub fn clear_db_schema(&mut self) {
        self.db_schema.clear();
    }

    pub fn has_db_schema(&self) -> bool {
        self.db_schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_db_schema(&mut self, v: ZeusDBSchema) {
        self.db_schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_db_schema(&mut self) -> &mut ZeusDBSchema {
        if self.db_schema.is_none() {
            self.db_schema.set_default();
        }
        self.db_schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_db_schema(&mut self) -> ZeusDBSchema {
        self.db_schema.take().unwrap_or_else(|| ZeusDBSchema::new())
    }

    pub fn get_db_schema(&self) -> &ZeusDBSchema {
        self.db_schema.as_ref().unwrap_or_else(|| ZeusDBSchema::default_instance())
    }

    fn get_db_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<ZeusDBSchema> {
        &self.db_schema
    }

    fn mut_db_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ZeusDBSchema> {
        &mut self.db_schema
    }
}

impl ::protobuf::Message for GetSchemaResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.db_schema {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.db_schema)?;
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
        if let Some(ref v) = self.db_schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.db_schema.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetSchemaResponse {
    fn new() -> GetSchemaResponse {
        GetSchemaResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetSchemaResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ZeusDBSchema>>(
                    "db_schema",
                    GetSchemaResponse::get_db_schema_for_reflect,
                    GetSchemaResponse::mut_db_schema_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetSchemaResponse>(
                    "GetSchemaResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetSchemaResponse {
    fn clear(&mut self) {
        self.clear_db_schema();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetSchemaResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSchemaResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FieldType {
    STRING = 0,
    BOOL = 1,
    FLOAT = 2,
    INT32 = 3,
    INT64 = 4,
    TIMESTAMP = 5,
}

impl ::protobuf::ProtobufEnum for FieldType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FieldType> {
        match value {
            0 => ::std::option::Option::Some(FieldType::STRING),
            1 => ::std::option::Option::Some(FieldType::BOOL),
            2 => ::std::option::Option::Some(FieldType::FLOAT),
            3 => ::std::option::Option::Some(FieldType::INT32),
            4 => ::std::option::Option::Some(FieldType::INT64),
            5 => ::std::option::Option::Some(FieldType::TIMESTAMP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FieldType] = &[
            FieldType::STRING,
            FieldType::BOOL,
            FieldType::FLOAT,
            FieldType::INT32,
            FieldType::INT64,
            FieldType::TIMESTAMP,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<FieldType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FieldType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FieldType {
}

impl ::std::default::Default for FieldType {
    fn default() -> Self {
        FieldType::STRING
    }
}

impl ::protobuf::reflect::ProtobufValue for FieldType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18protobuf/zeus_meta.proto\"v\n\x0cZeusDBSchema\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\x02\x20\x01(\x05R\x02id\
    \x12(\n\x06tables\x18\x03\x20\x03(\x0b2\x10.ZeusTableSchemaR\x06tables\
    \x12\x18\n\x07version\x18\x04\x20\x01(\rR\x07version\"`\n\x0fZeusTableSc\
    hema\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\
    \x02\x20\x01(\x05R\x02id\x12)\n\x06fields\x18\x03\x20\x03(\x0b2\x11.Zeus\
    ColumnSchemaR\x06fields\"a\n\x10ZeusColumnSchema\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\x02\x20\x01(\x05R\x02id\x12\
    )\n\nfield_type\x18\x03\x20\x01(\x0e2\n.FieldTypeR\tfieldType\"+\n\x10Ge\
    tSchemaRequest\x12\x17\n\x07db_name\x18\x01\x20\x01(\tR\x06dbName\"?\n\
    \x11GetSchemaResponse\x12*\n\tdb_schema\x18\x01\x20\x01(\x0b2\r.ZeusDBSc\
    hemaR\x08dbSchema*Q\n\tFieldType\x12\n\n\x06STRING\x10\0\x12\x08\n\x04BO\
    OL\x10\x01\x12\t\n\x05FLOAT\x10\x02\x12\t\n\x05INT32\x10\x03\x12\t\n\x05\
    INT64\x10\x04\x12\r\n\tTIMESTAMP\x10\x052I\n\x0fZeusMetaService\x126\n\
    \x0bGetDBSchema\x12\x11.GetSchemaRequest\x1a\x12.GetSchemaResponse\"\0B\
    \x16\n\x12io.github.zeus.rpcP\x01J\xd1\x0b\n\x06\x12\x04\0\0+\x01\n\x08\
    \n\x01\x0c\x12\x03\0\0\x10\n\x08\n\x01\x08\x12\x03\x02\0+\n\x0b\n\x04\
    \x08\xe7\x07\0\x12\x03\x02\0+\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x02\
    \x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x02\x07\x13\n\x0e\n\x07\
    \x08\xe7\x07\0\x02\0\x01\x12\x03\x02\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\
    \x07\x12\x03\x02\x16*\n\x08\n\x01\x08\x12\x03\x03\0\"\n\x0b\n\x04\x08\
    \xe7\x07\x01\x12\x03\x03\0\"\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x03\
    \x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x03\x07\x1a\n\x0e\n\x07\
    \x08\xe7\x07\x01\x02\0\x01\x12\x03\x03\x07\x1a\n\x0c\n\x05\x08\xe7\x07\
    \x01\x03\x12\x03\x03\x1d!\n\n\n\x02\x04\0\x12\x04\x05\0\n\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x05\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x04\
    \x14\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x06\x04\x05\x16\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x0b\
    \x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x12\x13\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x07\x04\x11\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x07\x04\
    \x06\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\x04\t\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\x07\n\x0c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x07\x0f\x10\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x08\x04(\n\x0c\n\x05\x04\
    \0\x02\x02\x04\x12\x03\x08\x04\x0c\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\
    \x08\r\x1c\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x08\x1d#\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03\x08&'\n\x0b\n\x04\x04\0\x02\x03\x12\x03\t\x04\
    \x17\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\t\x04\x08(\n\x0c\n\x05\x04\0\
    \x02\x03\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\t\x0b\
    \x12\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\t\x15\x16\n\n\n\x02\x04\x01\
    \x12\x04\x0c\0\x10\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0c\x08\x17\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\r\x04\x14\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\
    \r\x04\x0c\x19\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\r\x04\n\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\r\x0b\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\r\x12\x13\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0e\x04\x11\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04\x0e\x04\r\x14\n\x0c\n\x05\x04\x01\x02\x01\
    \x05\x12\x03\x0e\x04\t\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0e\n\x0c\
    \n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0e\x0f\x10\n\x0b\n\x04\x04\x01\
    \x02\x02\x12\x03\x0f\x04)\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\x0f\
    \x04\x0c\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\x0f\r\x1d\n\x0c\n\x05\
    \x04\x01\x02\x02\x01\x12\x03\x0f\x1e$\n\x0c\n\x05\x04\x01\x02\x02\x03\
    \x12\x03\x0f'(\n\n\n\x02\x05\0\x12\x04\x12\0\x19\x01\n\n\n\x03\x05\0\x01\
    \x12\x03\x12\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\x03\x13\x04\x0f\n\x0c\n\
    \x05\x05\0\x02\0\x01\x12\x03\x13\x04\n\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03\x13\r\x0e\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x14\x04\r\n\x0c\n\x05\
    \x05\0\x02\x01\x01\x12\x03\x14\x04\x08\n\x0c\n\x05\x05\0\x02\x01\x02\x12\
    \x03\x14\x0b\x0c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x15\x04\x0e\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03\x15\x04\t\n\x0c\n\x05\x05\0\x02\x02\x02\
    \x12\x03\x15\x0c\r\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x16\x04\x0e\n\x0c\n\
    \x05\x05\0\x02\x03\x01\x12\x03\x16\x04\t\n\x0c\n\x05\x05\0\x02\x03\x02\
    \x12\x03\x16\x0c\r\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x17\x04\x0e\n\x0c\n\
    \x05\x05\0\x02\x04\x01\x12\x03\x17\x04\t\n\x0c\n\x05\x05\0\x02\x04\x02\
    \x12\x03\x17\x0c\r\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x18\x04\x12\n\x0c\n\
    \x05\x05\0\x02\x05\x01\x12\x03\x18\x04\r\n\x0c\n\x05\x05\0\x02\x05\x02\
    \x12\x03\x18\x10\x11\n\n\n\x02\x04\x02\x12\x04\x1b\0\x1f\x01\n\n\n\x03\
    \x04\x02\x01\x12\x03\x1b\x08\x18\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1c\
    \x04\x14\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1c\x04\x1b\x1a\n\x0c\n\x05\
    \x04\x02\x02\0\x05\x12\x03\x1c\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03\x1c\x0b\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1c\x12\x13\n\x0b\
    \n\x04\x04\x02\x02\x01\x12\x03\x1d\x04\x11\n\r\n\x05\x04\x02\x02\x01\x04\
    \x12\x04\x1d\x04\x1c\x14\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x1d\x04\
    \t\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1d\n\x0c\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03\x1d\x0f\x10\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x1e\
    \x04\x1d\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x1e\x04\x1d\x11\n\x0c\n\
    \x05\x04\x02\x02\x02\x06\x12\x03\x1e\x04\r\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03\x1e\x0e\x18\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x1e\x1b\
    \x1c\n\n\n\x02\x04\x03\x12\x04!\0#\x01\n\n\n\x03\x04\x03\x01\x12\x03!\
    \x08\x18\n\x0b\n\x04\x04\x03\x02\0\x12\x03\"\x04\x17\n\r\n\x05\x04\x03\
    \x02\0\x04\x12\x04\"\x04!\x1a\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\"\
    \x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\"\x0b\x12\n\x0c\n\x05\x04\
    \x03\x02\0\x03\x12\x03\"\x15\x16\n\n\n\x02\x04\x04\x12\x04%\0'\x01\n\n\n\
    \x03\x04\x04\x01\x12\x03%\x08\x19\n\x0b\n\x04\x04\x04\x02\0\x12\x03&\x04\
    \x1f\n\r\n\x05\x04\x04\x02\0\x04\x12\x04&\x04%\x1b\n\x0c\n\x05\x04\x04\
    \x02\0\x06\x12\x03&\x04\x10\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03&\x11\
    \x1a\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03&\x1d\x1e\n\n\n\x02\x06\0\x12\
    \x04)\0+\x01\n\n\n\x03\x06\0\x01\x12\x03)\x08\x17\n\x0b\n\x04\x06\0\x02\
    \0\x12\x03*\x04C\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03*\x08\x13\n\x0c\n\
    \x05\x06\0\x02\0\x02\x12\x03*\x14$\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03*/\
    @b\x06proto3\
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
