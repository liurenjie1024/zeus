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
pub struct ColumnValue {
    // message fields
    pub string_value: ::std::string::String,
    pub float_value: f32,
    pub i32_value: i32,
    pub i64_value: i64,
    pub bool_value: bool,
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
pub struct RowResult {
    // message fields
    pub columns: ::protobuf::RepeatedField<ColumnValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RowResult {}

impl RowResult {
    pub fn new() -> RowResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RowResult {
        static mut instance: ::protobuf::lazy::Lazy<RowResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RowResult,
        };
        unsafe {
            instance.get(RowResult::new)
        }
    }

    // repeated .ColumnValue columns = 1;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<ColumnValue>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<ColumnValue> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<ColumnValue> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[ColumnValue] {
        &self.columns
    }

    fn get_columns_for_reflect(&self) -> &::protobuf::RepeatedField<ColumnValue> {
        &self.columns
    }

    fn mut_columns_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ColumnValue> {
        &mut self.columns
    }
}

impl ::protobuf::Message for RowResult {
    fn is_initialized(&self) -> bool {
        for v in &self.columns {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns)?;
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
        for value in &self.columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.columns {
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

impl ::protobuf::MessageStatic for RowResult {
    fn new() -> RowResult {
        RowResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<RowResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ColumnValue>>(
                    "columns",
                    RowResult::get_columns_for_reflect,
                    RowResult::mut_columns_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RowResult>(
                    "RowResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RowResult {
    fn clear(&mut self) {
        self.clear_columns();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RowResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RowResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryResult {
    // message fields
    pub code: StatusCode,
    pub rows: ::protobuf::RepeatedField<RowResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryResult {}

impl QueryResult {
    pub fn new() -> QueryResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryResult {
        static mut instance: ::protobuf::lazy::Lazy<QueryResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryResult,
        };
        unsafe {
            instance.get(QueryResult::new)
        }
    }

    // .StatusCode code = 1;

    pub fn clear_code(&mut self) {
        self.code = StatusCode::OK;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: StatusCode) {
        self.code = v;
    }

    pub fn get_code(&self) -> StatusCode {
        self.code
    }

    fn get_code_for_reflect(&self) -> &StatusCode {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut StatusCode {
        &mut self.code
    }

    // repeated .RowResult rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<RowResult>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<RowResult> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<RowResult> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[RowResult] {
        &self.rows
    }

    fn get_rows_for_reflect(&self) -> &::protobuf::RepeatedField<RowResult> {
        &self.rows
    }

    fn mut_rows_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RowResult> {
        &mut self.rows
    }
}

impl ::protobuf::Message for QueryResult {
    fn is_initialized(&self) -> bool {
        for v in &self.rows {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows)?;
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
        if self.code != StatusCode::OK {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != StatusCode::OK {
            os.write_enum(1, self.code.value())?;
        }
        for v in &self.rows {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for QueryResult {
    fn new() -> QueryResult {
        QueryResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StatusCode>>(
                    "code",
                    QueryResult::get_code_for_reflect,
                    QueryResult::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RowResult>>(
                    "rows",
                    QueryResult::get_rows_for_reflect,
                    QueryResult::mut_rows_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryResult>(
                    "QueryResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryResult {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanNode {
    // message fields
    pub db_id: i32,
    pub table_id: i32,
    pub columns: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanNode {}

impl ScanNode {
    pub fn new() -> ScanNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanNode {
        static mut instance: ::protobuf::lazy::Lazy<ScanNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanNode,
        };
        unsafe {
            instance.get(ScanNode::new)
        }
    }

    // int32 db_id = 1;

    pub fn clear_db_id(&mut self) {
        self.db_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_db_id(&mut self, v: i32) {
        self.db_id = v;
    }

    pub fn get_db_id(&self) -> i32 {
        self.db_id
    }

    fn get_db_id_for_reflect(&self) -> &i32 {
        &self.db_id
    }

    fn mut_db_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.db_id
    }

    // int32 table_id = 2;

    pub fn clear_table_id(&mut self) {
        self.table_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i32) {
        self.table_id = v;
    }

    pub fn get_table_id(&self) -> i32 {
        self.table_id
    }

    fn get_table_id_for_reflect(&self) -> &i32 {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.table_id
    }

    // repeated int32 columns = 3;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::std::vec::Vec<i32>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.columns, ::std::vec::Vec::new())
    }

    pub fn get_columns(&self) -> &[i32] {
        &self.columns
    }

    fn get_columns_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.columns
    }

    fn mut_columns_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.columns
    }
}

impl ::protobuf::Message for ScanNode {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.db_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.table_id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.columns)?;
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
        if self.db_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.db_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.table_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.table_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.columns {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.db_id != 0 {
            os.write_int32(1, self.db_id)?;
        }
        if self.table_id != 0 {
            os.write_int32(2, self.table_id)?;
        }
        for v in &self.columns {
            os.write_int32(3, *v)?;
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

impl ::protobuf::MessageStatic for ScanNode {
    fn new() -> ScanNode {
        ScanNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "db_id",
                    ScanNode::get_db_id_for_reflect,
                    ScanNode::mut_db_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "table_id",
                    ScanNode::get_table_id_for_reflect,
                    ScanNode::mut_table_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "columns",
                    ScanNode::get_columns_for_reflect,
                    ScanNode::mut_columns_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanNode>(
                    "ScanNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanNode {
    fn clear(&mut self) {
        self.clear_db_id();
        self.clear_table_id();
        self.clear_columns();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlanNode {
    // message fields
    pub node_id: i32,
    pub children_num: i32,
    pub plan_node_type: PlanNodeType,
    pub scan_node: ::protobuf::SingularPtrField<ScanNode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlanNode {}

impl PlanNode {
    pub fn new() -> PlanNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlanNode {
        static mut instance: ::protobuf::lazy::Lazy<PlanNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlanNode,
        };
        unsafe {
            instance.get(PlanNode::new)
        }
    }

    // int32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: i32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> i32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &i32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.node_id
    }

    // int32 children_num = 2;

    pub fn clear_children_num(&mut self) {
        self.children_num = 0;
    }

    // Param is passed by value, moved
    pub fn set_children_num(&mut self, v: i32) {
        self.children_num = v;
    }

    pub fn get_children_num(&self) -> i32 {
        self.children_num
    }

    fn get_children_num_for_reflect(&self) -> &i32 {
        &self.children_num
    }

    fn mut_children_num_for_reflect(&mut self) -> &mut i32 {
        &mut self.children_num
    }

    // .PlanNodeType plan_node_type = 3;

    pub fn clear_plan_node_type(&mut self) {
        self.plan_node_type = PlanNodeType::SCAN_NODE;
    }

    // Param is passed by value, moved
    pub fn set_plan_node_type(&mut self, v: PlanNodeType) {
        self.plan_node_type = v;
    }

    pub fn get_plan_node_type(&self) -> PlanNodeType {
        self.plan_node_type
    }

    fn get_plan_node_type_for_reflect(&self) -> &PlanNodeType {
        &self.plan_node_type
    }

    fn mut_plan_node_type_for_reflect(&mut self) -> &mut PlanNodeType {
        &mut self.plan_node_type
    }

    // .ScanNode scan_node = 4;

    pub fn clear_scan_node(&mut self) {
        self.scan_node.clear();
    }

    pub fn has_scan_node(&self) -> bool {
        self.scan_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scan_node(&mut self, v: ScanNode) {
        self.scan_node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scan_node(&mut self) -> &mut ScanNode {
        if self.scan_node.is_none() {
            self.scan_node.set_default();
        }
        self.scan_node.as_mut().unwrap()
    }

    // Take field
    pub fn take_scan_node(&mut self) -> ScanNode {
        self.scan_node.take().unwrap_or_else(|| ScanNode::new())
    }

    pub fn get_scan_node(&self) -> &ScanNode {
        self.scan_node.as_ref().unwrap_or_else(|| ScanNode::default_instance())
    }

    fn get_scan_node_for_reflect(&self) -> &::protobuf::SingularPtrField<ScanNode> {
        &self.scan_node
    }

    fn mut_scan_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ScanNode> {
        &mut self.scan_node
    }
}

impl ::protobuf::Message for PlanNode {
    fn is_initialized(&self) -> bool {
        for v in &self.scan_node {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.node_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.children_num = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.plan_node_type = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scan_node)?;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.children_num != 0 {
            my_size += ::protobuf::rt::value_size(2, self.children_num, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.plan_node_type != PlanNodeType::SCAN_NODE {
            my_size += ::protobuf::rt::enum_size(3, self.plan_node_type);
        }
        if let Some(ref v) = self.scan_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_int32(1, self.node_id)?;
        }
        if self.children_num != 0 {
            os.write_int32(2, self.children_num)?;
        }
        if self.plan_node_type != PlanNodeType::SCAN_NODE {
            os.write_enum(3, self.plan_node_type.value())?;
        }
        if let Some(ref v) = self.scan_node.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PlanNode {
    fn new() -> PlanNode {
        PlanNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlanNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "node_id",
                    PlanNode::get_node_id_for_reflect,
                    PlanNode::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "children_num",
                    PlanNode::get_children_num_for_reflect,
                    PlanNode::mut_children_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PlanNodeType>>(
                    "plan_node_type",
                    PlanNode::get_plan_node_type_for_reflect,
                    PlanNode::mut_plan_node_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScanNode>>(
                    "scan_node",
                    PlanNode::get_scan_node_for_reflect,
                    PlanNode::mut_scan_node_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlanNode>(
                    "PlanNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlanNode {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_children_num();
        self.clear_plan_node_type();
        self.clear_scan_node();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlanNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlanNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryPlan {
    // message fields
    pub plan_id: i32,
    pub nodes: ::protobuf::RepeatedField<PlanNode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryPlan {}

impl QueryPlan {
    pub fn new() -> QueryPlan {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryPlan {
        static mut instance: ::protobuf::lazy::Lazy<QueryPlan> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryPlan,
        };
        unsafe {
            instance.get(QueryPlan::new)
        }
    }

    // int32 plan_id = 1;

    pub fn clear_plan_id(&mut self) {
        self.plan_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_plan_id(&mut self, v: i32) {
        self.plan_id = v;
    }

    pub fn get_plan_id(&self) -> i32 {
        self.plan_id
    }

    fn get_plan_id_for_reflect(&self) -> &i32 {
        &self.plan_id
    }

    fn mut_plan_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.plan_id
    }

    // repeated .PlanNode nodes = 2;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<PlanNode>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<PlanNode> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<PlanNode> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[PlanNode] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<PlanNode> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PlanNode> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for QueryPlan {
    fn is_initialized(&self) -> bool {
        for v in &self.nodes {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.plan_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
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
        if self.plan_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.plan_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.plan_id != 0 {
            os.write_int32(1, self.plan_id)?;
        }
        for v in &self.nodes {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for QueryPlan {
    fn new() -> QueryPlan {
        QueryPlan::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryPlan>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "plan_id",
                    QueryPlan::get_plan_id_for_reflect,
                    QueryPlan::mut_plan_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlanNode>>(
                    "nodes",
                    QueryPlan::get_nodes_for_reflect,
                    QueryPlan::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryPlan>(
                    "QueryPlan",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryPlan {
    fn clear(&mut self) {
        self.clear_plan_id();
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryPlan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryPlan {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryRequest {
    // message fields
    pub plan: ::protobuf::SingularPtrField<QueryPlan>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryRequest {}

impl QueryRequest {
    pub fn new() -> QueryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryRequest {
        static mut instance: ::protobuf::lazy::Lazy<QueryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryRequest,
        };
        unsafe {
            instance.get(QueryRequest::new)
        }
    }

    // .QueryPlan plan = 1;

    pub fn clear_plan(&mut self) {
        self.plan.clear();
    }

    pub fn has_plan(&self) -> bool {
        self.plan.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plan(&mut self, v: QueryPlan) {
        self.plan = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plan(&mut self) -> &mut QueryPlan {
        if self.plan.is_none() {
            self.plan.set_default();
        }
        self.plan.as_mut().unwrap()
    }

    // Take field
    pub fn take_plan(&mut self) -> QueryPlan {
        self.plan.take().unwrap_or_else(|| QueryPlan::new())
    }

    pub fn get_plan(&self) -> &QueryPlan {
        self.plan.as_ref().unwrap_or_else(|| QueryPlan::default_instance())
    }

    fn get_plan_for_reflect(&self) -> &::protobuf::SingularPtrField<QueryPlan> {
        &self.plan
    }

    fn mut_plan_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<QueryPlan> {
        &mut self.plan
    }
}

impl ::protobuf::Message for QueryRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.plan {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.plan)?;
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
        if let Some(ref v) = self.plan.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.plan.as_ref() {
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

impl ::protobuf::MessageStatic for QueryRequest {
    fn new() -> QueryRequest {
        QueryRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<QueryPlan>>(
                    "plan",
                    QueryRequest::get_plan_for_reflect,
                    QueryRequest::mut_plan_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryRequest>(
                    "QueryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryRequest {
    fn clear(&mut self) {
        self.clear_plan();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Row {
    // message fields
    pub db_id: i32,
    pub table_id: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Row {}

impl Row {
    pub fn new() -> Row {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Row {
        static mut instance: ::protobuf::lazy::Lazy<Row> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Row,
        };
        unsafe {
            instance.get(Row::new)
        }
    }

    // int32 db_id = 1;

    pub fn clear_db_id(&mut self) {
        self.db_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_db_id(&mut self, v: i32) {
        self.db_id = v;
    }

    pub fn get_db_id(&self) -> i32 {
        self.db_id
    }

    fn get_db_id_for_reflect(&self) -> &i32 {
        &self.db_id
    }

    fn mut_db_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.db_id
    }

    // int32 table_id = 2;

    pub fn clear_table_id(&mut self) {
        self.table_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i32) {
        self.table_id = v;
    }

    pub fn get_table_id(&self) -> i32 {
        self.table_id
    }

    fn get_table_id_for_reflect(&self) -> &i32 {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.table_id
    }
}

impl ::protobuf::Message for Row {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.db_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.table_id = tmp;
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
        if self.db_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.db_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.table_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.table_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.db_id != 0 {
            os.write_int32(1, self.db_id)?;
        }
        if self.table_id != 0 {
            os.write_int32(2, self.table_id)?;
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

impl ::protobuf::MessageStatic for Row {
    fn new() -> Row {
        Row::new()
    }

    fn descriptor_static(_: ::std::option::Option<Row>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "db_id",
                    Row::get_db_id_for_reflect,
                    Row::mut_db_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "table_id",
                    Row::get_table_id_for_reflect,
                    Row::mut_table_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Row>(
                    "Row",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Row {
    fn clear(&mut self) {
        self.clear_db_id();
        self.clear_table_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Row {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Row {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StatusCode {
    OK = 0,
}

impl ::protobuf::ProtobufEnum for StatusCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StatusCode> {
        match value {
            0 => ::std::option::Option::Some(StatusCode::OK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StatusCode] = &[
            StatusCode::OK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<StatusCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StatusCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StatusCode {
}

impl ::std::default::Default for StatusCode {
    fn default() -> Self {
        StatusCode::OK
    }
}

impl ::protobuf::reflect::ProtobufValue for StatusCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PlanNodeType {
    SCAN_NODE = 0,
}

impl ::protobuf::ProtobufEnum for PlanNodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PlanNodeType> {
        match value {
            0 => ::std::option::Option::Some(PlanNodeType::SCAN_NODE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PlanNodeType] = &[
            PlanNodeType::SCAN_NODE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<PlanNodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PlanNodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PlanNodeType {
}

impl ::std::default::Default for PlanNodeType {
    fn default() -> Self {
        PlanNodeType::SCAN_NODE
    }
}

impl ::protobuf::reflect::ProtobufValue for PlanNodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18protobuf/zeus_data.proto\"\xaa\x01\n\x0bColumnValue\x12!\n\x0cstri\
    ng_value\x18\x01\x20\x01(\tR\x0bstringValue\x12\x1f\n\x0bfloat_value\x18\
    \x02\x20\x01(\x02R\nfloatValue\x12\x1b\n\ti32_value\x18\x03\x20\x01(\x05\
    R\x08i32Value\x12\x1b\n\ti64_value\x18\x04\x20\x01(\x03R\x08i64Value\x12\
    \x1d\n\nbool_value\x18\x05\x20\x01(\x08R\tboolValue\"3\n\tRowResult\x12&\
    \n\x07columns\x18\x01\x20\x03(\x0b2\x0c.ColumnValueR\x07columns\"N\n\x0b\
    QueryResult\x12\x1f\n\x04code\x18\x01\x20\x01(\x0e2\x0b.StatusCodeR\x04c\
    ode\x12\x1e\n\x04rows\x18\x02\x20\x03(\x0b2\n.RowResultR\x04rows\"T\n\
    \x08ScanNode\x12\x13\n\x05db_id\x18\x01\x20\x01(\x05R\x04dbId\x12\x19\n\
    \x08table_id\x18\x02\x20\x01(\x05R\x07tableId\x12\x18\n\x07columns\x18\
    \x03\x20\x03(\x05R\x07columns\"\xa3\x01\n\x08PlanNode\x12\x17\n\x07node_\
    id\x18\x01\x20\x01(\x05R\x06nodeId\x12!\n\x0cchildren_num\x18\x02\x20\
    \x01(\x05R\x0bchildrenNum\x123\n\x0eplan_node_type\x18\x03\x20\x01(\x0e2\
    \r.PlanNodeTypeR\x0cplanNodeType\x12&\n\tscan_node\x18\x04\x20\x01(\x0b2\
    \t.ScanNodeR\x08scanNode\"E\n\tQueryPlan\x12\x17\n\x07plan_id\x18\x01\
    \x20\x01(\x05R\x06planId\x12\x1f\n\x05nodes\x18\x02\x20\x03(\x0b2\t.Plan\
    NodeR\x05nodes\".\n\x0cQueryRequest\x12\x1e\n\x04plan\x18\x01\x20\x01(\
    \x0b2\n.QueryPlanR\x04plan\"5\n\x03Row\x12\x13\n\x05db_id\x18\x01\x20\
    \x01(\x05R\x04dbId\x12\x19\n\x08table_id\x18\x02\x20\x01(\x05R\x07tableI\
    d*\x14\n\nStatusCode\x12\x06\n\x02OK\x10\0*\x1d\n\x0cPlanNodeType\x12\r\
    \n\tSCAN_NODE\x10\029\n\x0fZeusDataService\x12&\n\x05Query\x12\r.QueryRe\
    quest\x1a\x0c.QueryResult\"\0B\x16\n\x12io.github.zeus.rpcP\x01J\xd8\x0f\
    \n\x06\x12\x04\0\0>\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x08\
    \x12\x03\x02\0+\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x02\0+\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x02\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x02\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x02\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x02\x16*\n\x08\n\x01\x08\x12\x03\
    \x03\0\"\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x03\0\"\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x03\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x03\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x03\x07\x1a\n\
    \x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\x03\x1d!\n\n\n\x02\x04\0\x12\x04\
    \x05\0\x0b\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x13\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x06\x04\x1c\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x06\x04\x05\
    \x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\0\x02\
    \0\x01\x12\x03\x06\x0b\x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x1a\
    \x1b\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x04\x1a\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x04\x07\x04\x06\x1c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x07\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\n\x15\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x07\x18\x19\n\x0b\n\x04\x04\0\x02\x02\x12\x03\
    \x08\x04\x18\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x08\x04\x07\x1a\n\x0c\n\
    \x05\x04\0\x02\x02\x05\x12\x03\x08\x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\x08\n\x13\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x08\x16\x17\n\
    \x0b\n\x04\x04\0\x02\x03\x12\x03\t\x04\x18\n\r\n\x05\x04\0\x02\x03\x04\
    \x12\x04\t\x04\x08\x18\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\t\x04\t\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x03\t\n\x13\n\x0c\n\x05\x04\0\x02\x03\
    \x03\x12\x03\t\x16\x17\n\x0b\n\x04\x04\0\x02\x04\x12\x03\n\x04\x18\n\r\n\
    \x05\x04\0\x02\x04\x04\x12\x04\n\x04\t\x18\n\x0c\n\x05\x04\0\x02\x04\x05\
    \x12\x03\n\x04\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\n\t\x13\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03\n\x16\x17\n\n\n\x02\x04\x01\x12\x04\r\0\
    \x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\r\x08\x11\n\x0b\n\x04\x04\x01\x02\
    \0\x12\x03\x0e\x04%\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x0e\x04\x0c\n\
    \x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x0e\r\x18\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x0e\x19\x20\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0e#$\n\n\
    \n\x02\x05\0\x12\x04\x11\0\x13\x01\n\n\n\x03\x05\0\x01\x12\x03\x11\x05\
    \x0f\n\x0b\n\x04\x05\0\x02\0\x12\x03\x12\x04\x0b\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x12\x04\x06\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x12\t\n\n\n\
    \n\x02\x04\x02\x12\x04\x15\0\x18\x01\n\n\n\x03\x04\x02\x01\x12\x03\x15\
    \x08\x13\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x16\x04\x18\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04\x16\x04\x15\x15\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\
    \x16\x04\x0e\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x16\x0f\x13\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03\x16\x16\x17\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03\x17\x04\x20\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x17\x04\x0c\
    \n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x17\r\x16\n\x0c\n\x05\x04\x02\
    \x02\x01\x01\x12\x03\x17\x17\x1b\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\
    \x17\x1e\x1f\n\n\n\x02\x05\x01\x12\x04\x1a\0\x1c\x01\n\n\n\x03\x05\x01\
    \x01\x12\x03\x1a\x05\x11\n\x0b\n\x04\x05\x01\x02\0\x12\x03\x1b\x04\x12\n\
    \x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x1b\x04\r\n\x0c\n\x05\x05\x01\x02\0\
    \x02\x12\x03\x1b\x10\x11\n\n\n\x02\x04\x03\x12\x04\x1e\0\"\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03\x1e\x08\x10\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1f\
    \x04\x14\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x1f\x04\x1e\x12\n\x0c\n\x05\
    \x04\x03\x02\0\x05\x12\x03\x1f\x04\t\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x03\x1f\n\x0f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1f\x12\x13\n\x0b\n\
    \x04\x04\x03\x02\x01\x12\x03\x20\x04\x17\n\r\n\x05\x04\x03\x02\x01\x04\
    \x12\x04\x20\x04\x1f\x14\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x20\x04\
    \t\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x20\n\x12\n\x0c\n\x05\x04\x03\
    \x02\x01\x03\x12\x03\x20\x15\x16\n\x0b\n\x04\x04\x03\x02\x02\x12\x03!\
    \x04\x1f\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03!\x04\x0c\n\x0c\n\x05\
    \x04\x03\x02\x02\x05\x12\x03!\r\x12\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\
    \x03!\x13\x1a\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03!\x1d\x1e\n\n\n\x02\
    \x04\x04\x12\x04%\0+\x01\n\n\n\x03\x04\x04\x01\x12\x03%\x08\x10\n\x0b\n\
    \x04\x04\x04\x02\0\x12\x03&\x04\x16\n\r\n\x05\x04\x04\x02\0\x04\x12\x04&\
    \x04%\x12\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03&\x04\t\n\x0c\n\x05\x04\
    \x04\x02\0\x01\x12\x03&\n\x11\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03&\x14\
    \x15\n\x0b\n\x04\x04\x04\x02\x01\x12\x03'\x04\x1b\n\r\n\x05\x04\x04\x02\
    \x01\x04\x12\x04'\x04&\x16\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03'\x04\
    \t\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03'\n\x16\n\x0c\n\x05\x04\x04\
    \x02\x01\x03\x12\x03'\x19\x1a\n\x0b\n\x04\x04\x04\x02\x02\x12\x03(\x04$\
    \n\r\n\x05\x04\x04\x02\x02\x04\x12\x04(\x04'\x1b\n\x0c\n\x05\x04\x04\x02\
    \x02\x06\x12\x03(\x04\x10\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03(\x11\
    \x1f\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03(\"#\n\x0b\n\x04\x04\x04\x02\
    \x03\x12\x03*\x04\x1b\n\r\n\x05\x04\x04\x02\x03\x04\x12\x04*\x04($\n\x0c\
    \n\x05\x04\x04\x02\x03\x06\x12\x03*\x04\x0c\n\x0c\n\x05\x04\x04\x02\x03\
    \x01\x12\x03*\r\x16\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03*\x19\x1a\n\n\
    \n\x02\x04\x05\x12\x04-\00\x01\n\n\n\x03\x04\x05\x01\x12\x03-\x08\x11\n\
    \x0b\n\x04\x04\x05\x02\0\x12\x03.\x04\x16\n\r\n\x05\x04\x05\x02\0\x04\
    \x12\x04.\x04-\x13\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03.\x04\t\n\x0c\n\
    \x05\x04\x05\x02\0\x01\x12\x03.\n\x11\n\x0c\n\x05\x04\x05\x02\0\x03\x12\
    \x03.\x14\x15\n\x0b\n\x04\x04\x05\x02\x01\x12\x03/\x04\x20\n\x0c\n\x05\
    \x04\x05\x02\x01\x04\x12\x03/\x04\x0c\n\x0c\n\x05\x04\x05\x02\x01\x06\
    \x12\x03/\r\x15\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03/\x16\x1b\n\x0c\n\
    \x05\x04\x05\x02\x01\x03\x12\x03/\x1e\x1f\n\n\n\x02\x04\x06\x12\x042\04\
    \x01\n\n\n\x03\x04\x06\x01\x12\x032\x08\x14\n\x0b\n\x04\x04\x06\x02\0\
    \x12\x033\x04\x17\n\r\n\x05\x04\x06\x02\0\x04\x12\x043\x042\x16\n\x0c\n\
    \x05\x04\x06\x02\0\x06\x12\x033\x04\r\n\x0c\n\x05\x04\x06\x02\0\x01\x12\
    \x033\x0e\x12\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x033\x15\x16\n'\n\x02\
    \x04\x07\x12\x047\0:\x01\x1a\x1b\x20TODO:\x20Remove\x20this\x20message\n\
    \n\n\n\x03\x04\x07\x01\x12\x037\x08\x0b\n\x0b\n\x04\x04\x07\x02\0\x12\
    \x038\x04\x14\n\r\n\x05\x04\x07\x02\0\x04\x12\x048\x047\r\n\x0c\n\x05\
    \x04\x07\x02\0\x05\x12\x038\x04\t\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x038\
    \n\x0f\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x038\x12\x13\n\x0b\n\x04\x04\
    \x07\x02\x01\x12\x039\x04\x17\n\r\n\x05\x04\x07\x02\x01\x04\x12\x049\x04\
    8\x14\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x039\x04\t\n\x0c\n\x05\x04\x07\
    \x02\x01\x01\x12\x039\n\x12\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x039\x15\
    \x16\n\n\n\x02\x06\0\x12\x04<\0>\x01\n\n\n\x03\x06\0\x01\x12\x03<\x08\
    \x17\n\x0b\n\x04\x06\0\x02\0\x12\x03=\x044\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03=\x08\r\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03=\x0e\x1a\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x03=%0b\x06proto3\
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
