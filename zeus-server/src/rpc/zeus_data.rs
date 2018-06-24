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
pub struct RowResult {
    // message fields
    pub columns: ::protobuf::RepeatedField<super::zeus_meta::ColumnValue>,
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
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<super::zeus_meta::ColumnValue>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_meta::ColumnValue> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<super::zeus_meta::ColumnValue> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[super::zeus_meta::ColumnValue] {
        &self.columns
    }

    fn get_columns_for_reflect(&self) -> &::protobuf::RepeatedField<super::zeus_meta::ColumnValue> {
        &self.columns
    }

    fn mut_columns_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_meta::ColumnValue> {
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_meta::ColumnValue>>(
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
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
pub struct QueryPlan {
    // message fields
    pub plan_id: ::std::string::String,
    pub root: ::protobuf::SingularPtrField<super::zeus_plan::PlanNode>,
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

    // string plan_id = 1;

    pub fn clear_plan_id(&mut self) {
        self.plan_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_plan_id(&mut self, v: ::std::string::String) {
        self.plan_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plan_id(&mut self) -> &mut ::std::string::String {
        &mut self.plan_id
    }

    // Take field
    pub fn take_plan_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.plan_id, ::std::string::String::new())
    }

    pub fn get_plan_id(&self) -> &str {
        &self.plan_id
    }

    fn get_plan_id_for_reflect(&self) -> &::std::string::String {
        &self.plan_id
    }

    fn mut_plan_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.plan_id
    }

    // .PlanNode root = 2;

    pub fn clear_root(&mut self) {
        self.root.clear();
    }

    pub fn has_root(&self) -> bool {
        self.root.is_some()
    }

    // Param is passed by value, moved
    pub fn set_root(&mut self, v: super::zeus_plan::PlanNode) {
        self.root = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root(&mut self) -> &mut super::zeus_plan::PlanNode {
        if self.root.is_none() {
            self.root.set_default();
        }
        self.root.as_mut().unwrap()
    }

    // Take field
    pub fn take_root(&mut self) -> super::zeus_plan::PlanNode {
        self.root.take().unwrap_or_else(|| super::zeus_plan::PlanNode::new())
    }

    pub fn get_root(&self) -> &super::zeus_plan::PlanNode {
        self.root.as_ref().unwrap_or_else(|| super::zeus_plan::PlanNode::default_instance())
    }

    fn get_root_for_reflect(&self) -> &::protobuf::SingularPtrField<super::zeus_plan::PlanNode> {
        &self.root
    }

    fn mut_root_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::zeus_plan::PlanNode> {
        &mut self.root
    }
}

impl ::protobuf::Message for QueryPlan {
    fn is_initialized(&self) -> bool {
        for v in &self.root {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.plan_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.root)?;
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
        if !self.plan_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.plan_id);
        }
        if let Some(ref v) = self.root.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.plan_id.is_empty() {
            os.write_string(1, &self.plan_id)?;
        }
        if let Some(ref v) = self.root.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "plan_id",
                    QueryPlan::get_plan_id_for_reflect,
                    QueryPlan::mut_plan_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_plan::PlanNode>>(
                    "root",
                    QueryPlan::get_root_for_reflect,
                    QueryPlan::mut_root_for_reflect,
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
        self.clear_root();
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
    FAILED = 1,
}

impl ::protobuf::ProtobufEnum for StatusCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StatusCode> {
        match value {
            0 => ::std::option::Option::Some(StatusCode::OK),
            1 => ::std::option::Option::Some(StatusCode::FAILED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StatusCode] = &[
            StatusCode::OK,
            StatusCode::FAILED,
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fzeus_data.proto\x1a\x0fzeus_plan.proto\x1a\x0fzeus_meta.proto\"3\n\
    \tRowResult\x12&\n\x07columns\x18\x01\x20\x03(\x0b2\x0c.ColumnValueR\x07\
    columns\"N\n\x0bQueryResult\x12\x1f\n\x04code\x18\x01\x20\x01(\x0e2\x0b.\
    StatusCodeR\x04code\x12\x1e\n\x04rows\x18\x02\x20\x03(\x0b2\n.RowResultR\
    \x04rows\"C\n\tQueryPlan\x12\x17\n\x07plan_id\x18\x01\x20\x01(\tR\x06pla\
    nId\x12\x1d\n\x04root\x18\x02\x20\x01(\x0b2\t.PlanNodeR\x04root\".\n\x0c\
    QueryRequest\x12\x1e\n\x04plan\x18\x01\x20\x01(\x0b2\n.QueryPlanR\x04pla\
    n\"5\n\x03Row\x12\x13\n\x05db_id\x18\x01\x20\x01(\x05R\x04dbId\x12\x19\n\
    \x08table_id\x18\x02\x20\x01(\x05R\x07tableId*\x20\n\nStatusCode\x12\x06\
    \n\x02OK\x10\0\x12\n\n\x06FAILED\x10\x0129\n\x0fZeusDataService\x12&\n\
    \x05Query\x12\r.QueryRequest\x1a\x0c.QueryResult\"\0B\x16\n\x12io.github\
    .zeus.rpcP\x01J\xce\x07\n\x06\x12\x04\0\0)\x01\n\x08\n\x01\x0c\x12\x03\0\
    \0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x18\n\t\n\x02\x03\x01\x12\x03\x03\
    \x07\x18\n\x08\n\x01\x08\x12\x03\x05\0+\n\t\n\x02\x08\x01\x12\x03\x05\0+\
    \n\x08\n\x01\x08\x12\x03\x06\0\"\n\t\n\x02\x08\n\x12\x03\x06\0\"\n\n\n\
    \x02\x04\0\x12\x04\t\0\x0b\x01\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x11\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\n\x04%\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03\n\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\n\r\x18\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\n\x19\x20\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n\
    #$\n\n\n\x02\x05\0\x12\x04\r\0\x10\x01\n\n\n\x03\x05\0\x01\x12\x03\r\x05\
    \x0f\n\x0b\n\x04\x05\0\x02\0\x12\x03\x0e\x04\x0b\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x0e\x04\x06\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x0e\t\n\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03\x0f\x04\x0f\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03\x0f\x04\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x0f\r\x0e\n\
    \n\n\x02\x04\x01\x12\x04\x12\0\x15\x01\n\n\n\x03\x04\x01\x01\x12\x03\x12\
    \x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x13\x04\x18\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\x13\x04\x12\x15\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\
    \x13\x04\x0e\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x13\x0f\x13\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x13\x16\x17\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x14\x04\x20\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x14\x04\x0c\
    \n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x14\r\x16\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x14\x17\x1b\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x14\x1e\x1f\n\n\n\x02\x04\x02\x12\x04\x18\0\x1b\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x18\x08\x11\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x19\x04\x17\n\
    \r\n\x05\x04\x02\x02\0\x04\x12\x04\x19\x04\x18\x13\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\x19\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x19\
    \x0b\x12\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x19\x15\x16\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\x1a\x04\x16\n\r\n\x05\x04\x02\x02\x01\x04\x12\
    \x04\x1a\x04\x19\x17\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x1a\x04\x0c\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1a\r\x11\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03\x1a\x14\x15\n\n\n\x02\x04\x03\x12\x04\x1d\0\x1f\x01\
    \n\n\n\x03\x04\x03\x01\x12\x03\x1d\x08\x14\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03\x1e\x04\x17\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x1e\x04\x1d\x16\n\
    \x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x1e\x04\r\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x03\x1e\x0e\x12\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1e\x15\
    \x16\n'\n\x02\x04\x04\x12\x04\"\0%\x01\x1a\x1b\x20TODO:\x20Remove\x20thi\
    s\x20message\n\n\n\n\x03\x04\x04\x01\x12\x03\"\x08\x0b\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03#\x04\x14\n\r\n\x05\x04\x04\x02\0\x04\x12\x04#\x04\"\r\
    \n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03#\x04\t\n\x0c\n\x05\x04\x04\x02\0\
    \x01\x12\x03#\n\x0f\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03#\x12\x13\n\x0b\
    \n\x04\x04\x04\x02\x01\x12\x03$\x04\x17\n\r\n\x05\x04\x04\x02\x01\x04\
    \x12\x04$\x04#\x14\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03$\x04\t\n\x0c\
    \n\x05\x04\x04\x02\x01\x01\x12\x03$\n\x12\n\x0c\n\x05\x04\x04\x02\x01\
    \x03\x12\x03$\x15\x16\n\n\n\x02\x06\0\x12\x04'\0)\x01\n\n\n\x03\x06\0\
    \x01\x12\x03'\x08\x17\n\x0b\n\x04\x06\0\x02\0\x12\x03(\x044\n\x0c\n\x05\
    \x06\0\x02\0\x01\x12\x03(\x08\r\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03(\x0e\
    \x1a\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03(%0b\x06proto3\
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
