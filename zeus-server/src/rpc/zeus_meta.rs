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
pub struct ZeusCatalog {
    // message fields
    pub db_schemas: ::protobuf::RepeatedField<ZeusDBSchema>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZeusCatalog {}

impl ZeusCatalog {
    pub fn new() -> ZeusCatalog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZeusCatalog {
        static mut instance: ::protobuf::lazy::Lazy<ZeusCatalog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZeusCatalog,
        };
        unsafe {
            instance.get(ZeusCatalog::new)
        }
    }

    // repeated .ZeusDBSchema db_schemas = 1;

    pub fn clear_db_schemas(&mut self) {
        self.db_schemas.clear();
    }

    // Param is passed by value, moved
    pub fn set_db_schemas(&mut self, v: ::protobuf::RepeatedField<ZeusDBSchema>) {
        self.db_schemas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_db_schemas(&mut self) -> &mut ::protobuf::RepeatedField<ZeusDBSchema> {
        &mut self.db_schemas
    }

    // Take field
    pub fn take_db_schemas(&mut self) -> ::protobuf::RepeatedField<ZeusDBSchema> {
        ::std::mem::replace(&mut self.db_schemas, ::protobuf::RepeatedField::new())
    }

    pub fn get_db_schemas(&self) -> &[ZeusDBSchema] {
        &self.db_schemas
    }

    fn get_db_schemas_for_reflect(&self) -> &::protobuf::RepeatedField<ZeusDBSchema> {
        &self.db_schemas
    }

    fn mut_db_schemas_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ZeusDBSchema> {
        &mut self.db_schemas
    }
}

impl ::protobuf::Message for ZeusCatalog {
    fn is_initialized(&self) -> bool {
        for v in &self.db_schemas {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.db_schemas)?;
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
        for value in &self.db_schemas {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.db_schemas {
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ZeusCatalog {
    fn new() -> ZeusCatalog {
        ZeusCatalog::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZeusCatalog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ZeusDBSchema>>(
                    "db_schemas",
                    ZeusCatalog::get_db_schemas_for_reflect,
                    ZeusCatalog::mut_db_schemas_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZeusCatalog>(
                    "ZeusCatalog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZeusCatalog {
    fn clear(&mut self) {
        self.clear_db_schemas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ZeusCatalog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZeusCatalog {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZeusDBSchema {
    // message fields
    pub name: ::std::string::String,
    pub id: i32,
    pub tables: ::std::collections::HashMap<i32, ZeusTableSchema>,
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

    // repeated .ZeusDBSchema.TablesEntry tables = 3;

    pub fn clear_tables(&mut self) {
        self.tables.clear();
    }

    // Param is passed by value, moved
    pub fn set_tables(&mut self, v: ::std::collections::HashMap<i32, ZeusTableSchema>) {
        self.tables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tables(&mut self) -> &mut ::std::collections::HashMap<i32, ZeusTableSchema> {
        &mut self.tables
    }

    // Take field
    pub fn take_tables(&mut self) -> ::std::collections::HashMap<i32, ZeusTableSchema> {
        ::std::mem::replace(&mut self.tables, ::std::collections::HashMap::new())
    }

    pub fn get_tables(&self) -> &::std::collections::HashMap<i32, ZeusTableSchema> {
        &self.tables
    }

    fn get_tables_for_reflect(&self) -> &::std::collections::HashMap<i32, ZeusTableSchema> {
        &self.tables
    }

    fn mut_tables_for_reflect(&mut self) -> &mut ::std::collections::HashMap<i32, ZeusTableSchema> {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusTableSchema>>(wire_type, is, &mut self.tables)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusTableSchema>>(3, &self.tables);
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
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusTableSchema>>(3, &self.tables, os)?;
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
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusTableSchema>>(
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
    pub columns: ::std::collections::HashMap<i32, ZeusColumnSchema>,
    pub format: ::std::string::String,
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

    // repeated .ZeusTableSchema.ColumnsEntry columns = 3;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::std::collections::HashMap<i32, ZeusColumnSchema>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::std::collections::HashMap<i32, ZeusColumnSchema> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::std::collections::HashMap<i32, ZeusColumnSchema> {
        ::std::mem::replace(&mut self.columns, ::std::collections::HashMap::new())
    }

    pub fn get_columns(&self) -> &::std::collections::HashMap<i32, ZeusColumnSchema> {
        &self.columns
    }

    fn get_columns_for_reflect(&self) -> &::std::collections::HashMap<i32, ZeusColumnSchema> {
        &self.columns
    }

    fn mut_columns_for_reflect(&mut self) -> &mut ::std::collections::HashMap<i32, ZeusColumnSchema> {
        &mut self.columns
    }

    // string format = 4;

    pub fn clear_format(&mut self) {
        self.format.clear();
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: ::std::string::String) {
        self.format = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_format(&mut self) -> &mut ::std::string::String {
        &mut self.format
    }

    // Take field
    pub fn take_format(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.format, ::std::string::String::new())
    }

    pub fn get_format(&self) -> &str {
        &self.format
    }

    fn get_format_for_reflect(&self) -> &::std::string::String {
        &self.format
    }

    fn mut_format_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.format
    }
}

impl ::protobuf::Message for ZeusTableSchema {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusColumnSchema>>(wire_type, is, &mut self.columns)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.format)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusColumnSchema>>(3, &self.columns);
        if !self.format.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.format);
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
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusColumnSchema>>(3, &self.columns, os)?;
        if !self.format.is_empty() {
            os.write_string(4, &self.format)?;
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
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ZeusColumnSchema>>(
                    "columns",
                    ZeusTableSchema::get_columns_for_reflect,
                    ZeusTableSchema::mut_columns_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "format",
                    ZeusTableSchema::get_format_for_reflect,
                    ZeusTableSchema::mut_format_for_reflect,
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
        self.clear_columns();
        self.clear_format();
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
pub struct ColumnValue {
    // message fields
    pub string_value: ::std::string::String,
    pub float_value: f32,
    pub i32_value: i32,
    pub i64_value: i64,
    pub bool_value: bool,
    pub double_value: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnValue {}

impl ColumnValue {
    pub fn new() -> ColumnValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnValue {
        static mut instance: ::protobuf::lazy::Lazy<ColumnValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnValue,
        };
        unsafe {
            instance.get(ColumnValue::new)
        }
    }

    // string string_value = 1;

    pub fn clear_string_value(&mut self) {
        self.string_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.string_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        &mut self.string_value
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.string_value, ::std::string::String::new())
    }

    pub fn get_string_value(&self) -> &str {
        &self.string_value
    }

    fn get_string_value_for_reflect(&self) -> &::std::string::String {
        &self.string_value
    }

    fn mut_string_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.string_value
    }

    // float float_value = 2;

    pub fn clear_float_value(&mut self) {
        self.float_value = 0.;
    }

    // Param is passed by value, moved
    pub fn set_float_value(&mut self, v: f32) {
        self.float_value = v;
    }

    pub fn get_float_value(&self) -> f32 {
        self.float_value
    }

    fn get_float_value_for_reflect(&self) -> &f32 {
        &self.float_value
    }

    fn mut_float_value_for_reflect(&mut self) -> &mut f32 {
        &mut self.float_value
    }

    // int32 i32_value = 3;

    pub fn clear_i32_value(&mut self) {
        self.i32_value = 0;
    }

    // Param is passed by value, moved
    pub fn set_i32_value(&mut self, v: i32) {
        self.i32_value = v;
    }

    pub fn get_i32_value(&self) -> i32 {
        self.i32_value
    }

    fn get_i32_value_for_reflect(&self) -> &i32 {
        &self.i32_value
    }

    fn mut_i32_value_for_reflect(&mut self) -> &mut i32 {
        &mut self.i32_value
    }

    // int64 i64_value = 4;

    pub fn clear_i64_value(&mut self) {
        self.i64_value = 0;
    }

    // Param is passed by value, moved
    pub fn set_i64_value(&mut self, v: i64) {
        self.i64_value = v;
    }

    pub fn get_i64_value(&self) -> i64 {
        self.i64_value
    }

    fn get_i64_value_for_reflect(&self) -> &i64 {
        &self.i64_value
    }

    fn mut_i64_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.i64_value
    }

    // bool bool_value = 5;

    pub fn clear_bool_value(&mut self) {
        self.bool_value = false;
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.bool_value = v;
    }

    pub fn get_bool_value(&self) -> bool {
        self.bool_value
    }

    fn get_bool_value_for_reflect(&self) -> &bool {
        &self.bool_value
    }

    fn mut_bool_value_for_reflect(&mut self) -> &mut bool {
        &mut self.bool_value
    }

    // double double_value = 6;

    pub fn clear_double_value(&mut self) {
        self.double_value = 0.;
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.double_value = v;
    }

    pub fn get_double_value(&self) -> f64 {
        self.double_value
    }

    fn get_double_value_for_reflect(&self) -> &f64 {
        &self.double_value
    }

    fn mut_double_value_for_reflect(&mut self) -> &mut f64 {
        &mut self.double_value
    }
}

impl ::protobuf::Message for ColumnValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.string_value)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.float_value = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.i32_value = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.i64_value = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.bool_value = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.double_value = tmp;
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
        if !self.string_value.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.string_value);
        }
        if self.float_value != 0. {
            my_size += 5;
        }
        if self.i32_value != 0 {
            my_size += ::protobuf::rt::value_size(3, self.i32_value, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.i64_value != 0 {
            my_size += ::protobuf::rt::value_size(4, self.i64_value, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bool_value != false {
            my_size += 2;
        }
        if self.double_value != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.string_value.is_empty() {
            os.write_string(1, &self.string_value)?;
        }
        if self.float_value != 0. {
            os.write_float(2, self.float_value)?;
        }
        if self.i32_value != 0 {
            os.write_int32(3, self.i32_value)?;
        }
        if self.i64_value != 0 {
            os.write_int64(4, self.i64_value)?;
        }
        if self.bool_value != false {
            os.write_bool(5, self.bool_value)?;
        }
        if self.double_value != 0. {
            os.write_double(6, self.double_value)?;
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

impl ::protobuf::MessageStatic for ColumnValue {
    fn new() -> ColumnValue {
        ColumnValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_value",
                    ColumnValue::get_string_value_for_reflect,
                    ColumnValue::mut_string_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_value",
                    ColumnValue::get_float_value_for_reflect,
                    ColumnValue::mut_float_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "i32_value",
                    ColumnValue::get_i32_value_for_reflect,
                    ColumnValue::mut_i32_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "i64_value",
                    ColumnValue::get_i64_value_for_reflect,
                    ColumnValue::mut_i64_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_value",
                    ColumnValue::get_bool_value_for_reflect,
                    ColumnValue::mut_bool_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_value",
                    ColumnValue::get_double_value_for_reflect,
                    ColumnValue::mut_double_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnValue>(
                    "ColumnValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnValue {
    fn clear(&mut self) {
        self.clear_string_value();
        self.clear_float_value();
        self.clear_i32_value();
        self.clear_i64_value();
        self.clear_bool_value();
        self.clear_double_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColumnValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZeusColumnSchema {
    // message fields
    pub name: ::std::string::String,
    pub id: i32,
    pub column_type: ColumnType,
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

    // .ColumnType column_type = 3;

    pub fn clear_column_type(&mut self) {
        self.column_type = ColumnType::BOOL;
    }

    // Param is passed by value, moved
    pub fn set_column_type(&mut self, v: ColumnType) {
        self.column_type = v;
    }

    pub fn get_column_type(&self) -> ColumnType {
        self.column_type
    }

    fn get_column_type_for_reflect(&self) -> &ColumnType {
        &self.column_type
    }

    fn mut_column_type_for_reflect(&mut self) -> &mut ColumnType {
        &mut self.column_type
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.column_type, 3, &mut self.unknown_fields)?
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
        if self.column_type != ColumnType::BOOL {
            my_size += ::protobuf::rt::enum_size(3, self.column_type);
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
        if self.column_type != ColumnType::BOOL {
            os.write_enum(3, self.column_type.value())?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ColumnType>>(
                    "column_type",
                    ZeusColumnSchema::get_column_type_for_reflect,
                    ZeusColumnSchema::mut_column_type_for_reflect,
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
        self.clear_column_type();
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
pub enum ColumnType {
    BOOL = 0,
    INT8 = 1,
    INT16 = 2,
    INT32 = 3,
    INT64 = 4,
    FLOAT4 = 5,
    FLOAT8 = 6,
    TIMESTAMP = 7,
    STRING = 8,
}

impl ::protobuf::ProtobufEnum for ColumnType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ColumnType> {
        match value {
            0 => ::std::option::Option::Some(ColumnType::BOOL),
            1 => ::std::option::Option::Some(ColumnType::INT8),
            2 => ::std::option::Option::Some(ColumnType::INT16),
            3 => ::std::option::Option::Some(ColumnType::INT32),
            4 => ::std::option::Option::Some(ColumnType::INT64),
            5 => ::std::option::Option::Some(ColumnType::FLOAT4),
            6 => ::std::option::Option::Some(ColumnType::FLOAT8),
            7 => ::std::option::Option::Some(ColumnType::TIMESTAMP),
            8 => ::std::option::Option::Some(ColumnType::STRING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ColumnType] = &[
            ColumnType::BOOL,
            ColumnType::INT8,
            ColumnType::INT16,
            ColumnType::INT32,
            ColumnType::INT64,
            ColumnType::FLOAT4,
            ColumnType::FLOAT8,
            ColumnType::TIMESTAMP,
            ColumnType::STRING,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ColumnType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ColumnType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ColumnType {
}

impl ::std::default::Default for ColumnType {
    fn default() -> Self {
        ColumnType::BOOL
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fzeus_meta.proto\";\n\x0bZeusCatalog\x12,\n\ndb_schemas\x18\x01\x20\
    \x03(\x0b2\r.ZeusDBSchemaR\tdbSchemas\"\xcc\x01\n\x0cZeusDBSchema\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\x02\x20\
    \x01(\x05R\x02id\x121\n\x06tables\x18\x03\x20\x03(\x0b2\x19.ZeusDBSchema\
    .TablesEntryR\x06tables\x12\x18\n\x07version\x18\x04\x20\x01(\rR\x07vers\
    ion\x1aK\n\x0bTablesEntry\x12\x10\n\x03key\x18\x01\x20\x01(\x05R\x03key\
    \x12&\n\x05value\x18\x02\x20\x01(\x0b2\x10.ZeusTableSchemaR\x05value:\
    \x028\x01\"\xd5\x01\n\x0fZeusTableSchema\x12\x12\n\x04name\x18\x01\x20\
    \x01(\tR\x04name\x12\x0e\n\x02id\x18\x02\x20\x01(\x05R\x02id\x127\n\x07c\
    olumns\x18\x03\x20\x03(\x0b2\x1d.ZeusTableSchema.ColumnsEntryR\x07column\
    s\x12\x16\n\x06format\x18\x04\x20\x01(\tR\x06format\x1aM\n\x0cColumnsEnt\
    ry\x12\x10\n\x03key\x18\x01\x20\x01(\x05R\x03key\x12'\n\x05value\x18\x02\
    \x20\x01(\x0b2\x11.ZeusColumnSchemaR\x05value:\x028\x01\"\xcd\x01\n\x0bC\
    olumnValue\x12!\n\x0cstring_value\x18\x01\x20\x01(\tR\x0bstringValue\x12\
    \x1f\n\x0bfloat_value\x18\x02\x20\x01(\x02R\nfloatValue\x12\x1b\n\ti32_v\
    alue\x18\x03\x20\x01(\x05R\x08i32Value\x12\x1b\n\ti64_value\x18\x04\x20\
    \x01(\x03R\x08i64Value\x12\x1d\n\nbool_value\x18\x05\x20\x01(\x08R\tbool\
    Value\x12!\n\x0cdouble_value\x18\x06\x20\x01(\x01R\x0bdoubleValue\"d\n\
    \x10ZeusColumnSchema\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\
    \x0e\n\x02id\x18\x02\x20\x01(\x05R\x02id\x12,\n\x0bcolumn_type\x18\x03\
    \x20\x01(\x0e2\x0b.ColumnTypeR\ncolumnType\"+\n\x10GetSchemaRequest\x12\
    \x17\n\x07db_name\x18\x01\x20\x01(\tR\x06dbName\"?\n\x11GetSchemaRespons\
    e\x12*\n\tdb_schema\x18\x01\x20\x01(\x0b2\r.ZeusDBSchemaR\x08dbSchema*t\
    \n\nColumnType\x12\x08\n\x04BOOL\x10\0\x12\x08\n\x04INT8\x10\x01\x12\t\n\
    \x05INT16\x10\x02\x12\t\n\x05INT32\x10\x03\x12\t\n\x05INT64\x10\x04\x12\
    \n\n\x06FLOAT4\x10\x05\x12\n\n\x06FLOAT8\x10\x06\x12\r\n\tTIMESTAMP\x10\
    \x07\x12\n\n\x06STRING\x10\x082I\n\x0fZeusMetaService\x126\n\x0bGetDBSch\
    ema\x12\x11.GetSchemaRequest\x1a\x12.GetSchemaResponse\"\0B\x16\n\x12io.\
    github.zeus.rpcP\x01J\xc4\x11\n\x06\x12\x04\0\0<\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x10\n\x08\n\x01\x08\x12\x03\x02\0+\n\x0b\n\x04\x08\xe7\x07\0\
    \x12\x03\x02\0+\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x02\x07\x13\n\r\n\
    \x06\x08\xe7\x07\0\x02\0\x12\x03\x02\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x02\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x02\
    \x16*\n\x08\n\x01\x08\x12\x03\x03\0\"\n\x0b\n\x04\x08\xe7\x07\x01\x12\
    \x03\x03\0\"\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x03\x07\x1a\n\r\n\
    \x06\x08\xe7\x07\x01\x02\0\x12\x03\x03\x07\x1a\n\x0e\n\x07\x08\xe7\x07\
    \x01\x02\0\x01\x12\x03\x03\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\
    \x03\x03\x1d!\n\n\n\x02\x04\0\x12\x04\x05\0\x07\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x05\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x04)\n\x0c\n\
    \x05\x04\0\x02\0\x04\x12\x03\x06\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\x06\r\x19\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x1a$\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x06'(\n\n\n\x02\x04\x01\x12\x04\t\0\x0e\x01\n\n\
    \n\x03\x04\x01\x01\x12\x03\t\x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\
    \x04\x14\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\n\x04\t\x16\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\n\
    \x0b\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n\x12\x13\n\x0b\n\x04\x04\
    \x01\x02\x01\x12\x03\x0b\x04\x11\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\
    \x0b\x04\n\x14\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0b\x04\t\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\x0b\n\x0c\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x0b\x0f\x10\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x0c\x04+\n\
    \r\n\x05\x04\x01\x02\x02\x04\x12\x04\x0c\x04\x0b\x11\n\x0c\n\x05\x04\x01\
    \x02\x02\x06\x12\x03\x0c\x04\x1f\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\
    \x0c\x20&\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0c)*\n\x0b\n\x04\x04\
    \x01\x02\x03\x12\x03\r\x04\x17\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\r\
    \x04\x0c+\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\r\x04\n\n\x0c\n\x05\
    \x04\x01\x02\x03\x01\x12\x03\r\x0b\x12\n\x0c\n\x05\x04\x01\x02\x03\x03\
    \x12\x03\r\x15\x16\n\n\n\x02\x04\x02\x12\x04\x10\0\x15\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x10\x08\x17\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x11\x04\
    \x14\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x11\x04\x10\x19\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x11\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x11\x0b\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x11\x12\x13\n\x0b\n\
    \x04\x04\x02\x02\x01\x12\x03\x12\x04\x11\n\r\n\x05\x04\x02\x02\x01\x04\
    \x12\x04\x12\x04\x11\x14\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x12\x04\
    \t\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x12\n\x0c\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03\x12\x0f\x10\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x13\
    \x04-\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x13\x04\x12\x11\n\x0c\n\x05\
    \x04\x02\x02\x02\x06\x12\x03\x13\x04\x20\n\x0c\n\x05\x04\x02\x02\x02\x01\
    \x12\x03\x13!(\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x13+,\n\"\n\x04\
    \x04\x02\x02\x03\x12\x03\x14\x04\x16\"\x15\x20storage\x20engine\x20name\
    \n\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04\x14\x04\x13-\n\x0c\n\x05\x04\
    \x02\x02\x03\x05\x12\x03\x14\x04\n\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\
    \x03\x14\x0b\x11\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03\x14\x14\x15\n\n\
    \n\x02\x05\0\x12\x04\x17\0!\x01\n\n\n\x03\x05\0\x01\x12\x03\x17\x05\x0f\
    \n\x0b\n\x04\x05\0\x02\0\x12\x03\x18\x04\r\n\x0c\n\x05\x05\0\x02\0\x01\
    \x12\x03\x18\x04\x08\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x18\x0b\x0c\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03\x19\x04\r\n\x0c\n\x05\x05\0\x02\x01\x01\
    \x12\x03\x19\x04\x08\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x19\x0b\x0c\n\
    \x0b\n\x04\x05\0\x02\x02\x12\x03\x1a\x04\x0e\n\x0c\n\x05\x05\0\x02\x02\
    \x01\x12\x03\x1a\x04\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x1a\x0c\r\n\
    \x0b\n\x04\x05\0\x02\x03\x12\x03\x1b\x04\x0e\n\x0c\n\x05\x05\0\x02\x03\
    \x01\x12\x03\x1b\x04\t\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x1b\x0c\r\n\
    \x0b\n\x04\x05\0\x02\x04\x12\x03\x1c\x04\x0e\n\x0c\n\x05\x05\0\x02\x04\
    \x01\x12\x03\x1c\x04\t\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x1c\x0c\r\n\
    \x0b\n\x04\x05\0\x02\x05\x12\x03\x1d\x04\x0f\n\x0c\n\x05\x05\0\x02\x05\
    \x01\x12\x03\x1d\x04\n\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x1d\r\x0e\n\
    \x0b\n\x04\x05\0\x02\x06\x12\x03\x1e\x04\x0f\n\x0c\n\x05\x05\0\x02\x06\
    \x01\x12\x03\x1e\x04\n\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\x1e\r\x0e\n\
    \x0b\n\x04\x05\0\x02\x07\x12\x03\x1f\x04\x12\n\x0c\n\x05\x05\0\x02\x07\
    \x01\x12\x03\x1f\x04\r\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\x1f\x10\x11\
    \n\x0b\n\x04\x05\0\x02\x08\x12\x03\x20\x04\x0f\n\x0c\n\x05\x05\0\x02\x08\
    \x01\x12\x03\x20\x04\n\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x20\r\x0e\n\
    \n\n\x02\x04\x03\x12\x04#\0*\x01\n\n\n\x03\x04\x03\x01\x12\x03#\x08\x13\
    \n\x0b\n\x04\x04\x03\x02\0\x12\x03$\x04\x1c\n\r\n\x05\x04\x03\x02\0\x04\
    \x12\x04$\x04#\x15\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03$\x04\n\n\x0c\n\
    \x05\x04\x03\x02\0\x01\x12\x03$\x0b\x17\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03$\x1a\x1b\n\x0b\n\x04\x04\x03\x02\x01\x12\x03%\x04\x1a\n\r\n\x05\
    \x04\x03\x02\x01\x04\x12\x04%\x04$\x1c\n\x0c\n\x05\x04\x03\x02\x01\x05\
    \x12\x03%\x04\t\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03%\n\x15\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x03%\x18\x19\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x03&\x04\x18\n\r\n\x05\x04\x03\x02\x02\x04\x12\x04&\x04%\x1a\n\x0c\
    \n\x05\x04\x03\x02\x02\x05\x12\x03&\x04\t\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x03&\n\x13\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03&\x16\x17\n\
    \x0b\n\x04\x04\x03\x02\x03\x12\x03'\x04\x18\n\r\n\x05\x04\x03\x02\x03\
    \x04\x12\x04'\x04&\x18\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03'\x04\t\n\
    \x0c\n\x05\x04\x03\x02\x03\x01\x12\x03'\n\x13\n\x0c\n\x05\x04\x03\x02\
    \x03\x03\x12\x03'\x16\x17\n\x0b\n\x04\x04\x03\x02\x04\x12\x03(\x04\x18\n\
    \r\n\x05\x04\x03\x02\x04\x04\x12\x04(\x04'\x18\n\x0c\n\x05\x04\x03\x02\
    \x04\x05\x12\x03(\x04\x08\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03(\t\x13\
    \n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x03(\x16\x17\n\x0b\n\x04\x04\x03\
    \x02\x05\x12\x03)\x04\x1c\n\r\n\x05\x04\x03\x02\x05\x04\x12\x04)\x04(\
    \x18\n\x0c\n\x05\x04\x03\x02\x05\x05\x12\x03)\x04\n\n\x0c\n\x05\x04\x03\
    \x02\x05\x01\x12\x03)\x0b\x17\n\x0c\n\x05\x04\x03\x02\x05\x03\x12\x03)\
    \x1a\x1b\n\n\n\x02\x04\x04\x12\x04,\00\x01\n\n\n\x03\x04\x04\x01\x12\x03\
    ,\x08\x18\n\x0b\n\x04\x04\x04\x02\0\x12\x03-\x04\x14\n\r\n\x05\x04\x04\
    \x02\0\x04\x12\x04-\x04,\x1a\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03-\x04\
    \n\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03-\x0b\x0f\n\x0c\n\x05\x04\x04\
    \x02\0\x03\x12\x03-\x12\x13\n\x0b\n\x04\x04\x04\x02\x01\x12\x03.\x04\x11\
    \n\r\n\x05\x04\x04\x02\x01\x04\x12\x04.\x04-\x14\n\x0c\n\x05\x04\x04\x02\
    \x01\x05\x12\x03.\x04\t\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03.\n\x0c\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x03.\x0f\x10\n\x0b\n\x04\x04\x04\x02\
    \x02\x12\x03/\x04\x1f\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04/\x04.\x11\n\
    \x0c\n\x05\x04\x04\x02\x02\x06\x12\x03/\x04\x0e\n\x0c\n\x05\x04\x04\x02\
    \x02\x01\x12\x03/\x0f\x1a\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03/\x1d\
    \x1e\n\n\n\x02\x04\x05\x12\x042\04\x01\n\n\n\x03\x04\x05\x01\x12\x032\
    \x08\x18\n\x0b\n\x04\x04\x05\x02\0\x12\x033\x04\x17\n\r\n\x05\x04\x05\
    \x02\0\x04\x12\x043\x042\x1a\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x033\x04\
    \n\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x033\x0b\x12\n\x0c\n\x05\x04\x05\
    \x02\0\x03\x12\x033\x15\x16\n\n\n\x02\x04\x06\x12\x046\08\x01\n\n\n\x03\
    \x04\x06\x01\x12\x036\x08\x19\n\x0b\n\x04\x04\x06\x02\0\x12\x037\x04\x1f\
    \n\r\n\x05\x04\x06\x02\0\x04\x12\x047\x046\x1b\n\x0c\n\x05\x04\x06\x02\0\
    \x06\x12\x037\x04\x10\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x037\x11\x1a\n\
    \x0c\n\x05\x04\x06\x02\0\x03\x12\x037\x1d\x1e\n\n\n\x02\x06\0\x12\x04:\0\
    <\x01\n\n\n\x03\x06\0\x01\x12\x03:\x08\x17\n\x0b\n\x04\x06\0\x02\0\x12\
    \x03;\x04C\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03;\x08\x13\n\x0c\n\x05\x06\
    \0\x02\0\x02\x12\x03;\x14$\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03;/@b\x06pr\
    oto3\
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
