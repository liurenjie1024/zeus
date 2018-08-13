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
pub struct ScanNode {
    // message fields
    pub db_id: i32,
    pub table_id: i32,
    pub columns: ::std::vec::Vec<i32>,
    pub filters: ::protobuf::RepeatedField<super::zeus_expr::Expression>,
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

    // repeated .Expression filters = 4;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: ::protobuf::RepeatedField<super::zeus_expr::Expression>) {
        self.filters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filters(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.filters
    }

    // Take field
    pub fn take_filters(&mut self) -> ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        ::std::mem::replace(&mut self.filters, ::protobuf::RepeatedField::new())
    }

    pub fn get_filters(&self) -> &[super::zeus_expr::Expression] {
        &self.filters
    }

    fn get_filters_for_reflect(&self) -> &::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &self.filters
    }

    fn mut_filters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.filters
    }
}

impl ::protobuf::Message for ScanNode {
    fn is_initialized(&self) -> bool {
        for v in &self.filters {
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
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.filters)?;
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
        for value in &self.filters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        for v in &self.filters {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_expr::Expression>>(
                    "filters",
                    ScanNode::get_filters_for_reflect,
                    ScanNode::mut_filters_for_reflect,
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
        self.clear_filters();
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
pub struct FilterNode {
    // message fields
    pub conditions: ::protobuf::RepeatedField<super::zeus_expr::Expression>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FilterNode {}

impl FilterNode {
    pub fn new() -> FilterNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FilterNode {
        static mut instance: ::protobuf::lazy::Lazy<FilterNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FilterNode,
        };
        unsafe {
            instance.get(FilterNode::new)
        }
    }

    // repeated .Expression conditions = 1;

    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }

    // Param is passed by value, moved
    pub fn set_conditions(&mut self, v: ::protobuf::RepeatedField<super::zeus_expr::Expression>) {
        self.conditions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_conditions(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.conditions
    }

    // Take field
    pub fn take_conditions(&mut self) -> ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        ::std::mem::replace(&mut self.conditions, ::protobuf::RepeatedField::new())
    }

    pub fn get_conditions(&self) -> &[super::zeus_expr::Expression] {
        &self.conditions
    }

    fn get_conditions_for_reflect(&self) -> &::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &self.conditions
    }

    fn mut_conditions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.conditions
    }
}

impl ::protobuf::Message for FilterNode {
    fn is_initialized(&self) -> bool {
        for v in &self.conditions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.conditions)?;
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
        for value in &self.conditions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.conditions {
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

impl ::protobuf::MessageStatic for FilterNode {
    fn new() -> FilterNode {
        FilterNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<FilterNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_expr::Expression>>(
                    "conditions",
                    FilterNode::get_conditions_for_reflect,
                    FilterNode::mut_conditions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FilterNode>(
                    "FilterNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FilterNode {
    fn clear(&mut self) {
        self.clear_conditions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FilterNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FilterNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectNode {
    // message fields
    pub items: ::protobuf::RepeatedField<ProjectNode_ProjectItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectNode {}

impl ProjectNode {
    pub fn new() -> ProjectNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectNode {
        static mut instance: ::protobuf::lazy::Lazy<ProjectNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectNode,
        };
        unsafe {
            instance.get(ProjectNode::new)
        }
    }

    // repeated .ProjectNode.ProjectItem items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ProjectNode_ProjectItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ProjectNode_ProjectItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ProjectNode_ProjectItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ProjectNode_ProjectItem] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ProjectNode_ProjectItem> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ProjectNode_ProjectItem> {
        &mut self.items
    }
}

impl ::protobuf::Message for ProjectNode {
    fn is_initialized(&self) -> bool {
        for v in &self.items {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
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

impl ::protobuf::MessageStatic for ProjectNode {
    fn new() -> ProjectNode {
        ProjectNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProjectNode_ProjectItem>>(
                    "items",
                    ProjectNode::get_items_for_reflect,
                    ProjectNode::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectNode>(
                    "ProjectNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectNode {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectNode_ProjectItem {
    // message fields
    pub expression: ::protobuf::SingularPtrField<super::zeus_expr::Expression>,
    pub alias: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectNode_ProjectItem {}

impl ProjectNode_ProjectItem {
    pub fn new() -> ProjectNode_ProjectItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectNode_ProjectItem {
        static mut instance: ::protobuf::lazy::Lazy<ProjectNode_ProjectItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectNode_ProjectItem,
        };
        unsafe {
            instance.get(ProjectNode_ProjectItem::new)
        }
    }

    // .Expression expression = 1;

    pub fn clear_expression(&mut self) {
        self.expression.clear();
    }

    pub fn has_expression(&self) -> bool {
        self.expression.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expression(&mut self, v: super::zeus_expr::Expression) {
        self.expression = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expression(&mut self) -> &mut super::zeus_expr::Expression {
        if self.expression.is_none() {
            self.expression.set_default();
        }
        self.expression.as_mut().unwrap()
    }

    // Take field
    pub fn take_expression(&mut self) -> super::zeus_expr::Expression {
        self.expression.take().unwrap_or_else(|| super::zeus_expr::Expression::new())
    }

    pub fn get_expression(&self) -> &super::zeus_expr::Expression {
        self.expression.as_ref().unwrap_or_else(|| super::zeus_expr::Expression::default_instance())
    }

    fn get_expression_for_reflect(&self) -> &::protobuf::SingularPtrField<super::zeus_expr::Expression> {
        &self.expression
    }

    fn mut_expression_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::zeus_expr::Expression> {
        &mut self.expression
    }

    // string alias = 2;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alias, ::std::string::String::new())
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    fn get_alias_for_reflect(&self) -> &::std::string::String {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }
}

impl ::protobuf::Message for ProjectNode_ProjectItem {
    fn is_initialized(&self) -> bool {
        for v in &self.expression {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expression)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alias)?;
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
        if let Some(ref v) = self.expression.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.alias.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.alias);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.expression.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.alias.is_empty() {
            os.write_string(2, &self.alias)?;
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

impl ::protobuf::MessageStatic for ProjectNode_ProjectItem {
    fn new() -> ProjectNode_ProjectItem {
        ProjectNode_ProjectItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectNode_ProjectItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_expr::Expression>>(
                    "expression",
                    ProjectNode_ProjectItem::get_expression_for_reflect,
                    ProjectNode_ProjectItem::mut_expression_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    ProjectNode_ProjectItem::get_alias_for_reflect,
                    ProjectNode_ProjectItem::mut_alias_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectNode_ProjectItem>(
                    "ProjectNode_ProjectItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectNode_ProjectItem {
    fn clear(&mut self) {
        self.clear_expression();
        self.clear_alias();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectNode_ProjectItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectNode_ProjectItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AggregationNode {
    // message fields
    pub group_by: ::protobuf::RepeatedField<super::zeus_expr::Expression>,
    pub agg_func: ::protobuf::RepeatedField<super::zeus_expr::Expression>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AggregationNode {}

impl AggregationNode {
    pub fn new() -> AggregationNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AggregationNode {
        static mut instance: ::protobuf::lazy::Lazy<AggregationNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AggregationNode,
        };
        unsafe {
            instance.get(AggregationNode::new)
        }
    }

    // repeated .Expression group_by = 1;

    pub fn clear_group_by(&mut self) {
        self.group_by.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_by(&mut self, v: ::protobuf::RepeatedField<super::zeus_expr::Expression>) {
        self.group_by = v;
    }

    // Mutable pointer to the field.
    pub fn mut_group_by(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.group_by
    }

    // Take field
    pub fn take_group_by(&mut self) -> ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        ::std::mem::replace(&mut self.group_by, ::protobuf::RepeatedField::new())
    }

    pub fn get_group_by(&self) -> &[super::zeus_expr::Expression] {
        &self.group_by
    }

    fn get_group_by_for_reflect(&self) -> &::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &self.group_by
    }

    fn mut_group_by_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.group_by
    }

    // repeated .Expression agg_func = 2;

    pub fn clear_agg_func(&mut self) {
        self.agg_func.clear();
    }

    // Param is passed by value, moved
    pub fn set_agg_func(&mut self, v: ::protobuf::RepeatedField<super::zeus_expr::Expression>) {
        self.agg_func = v;
    }

    // Mutable pointer to the field.
    pub fn mut_agg_func(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.agg_func
    }

    // Take field
    pub fn take_agg_func(&mut self) -> ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        ::std::mem::replace(&mut self.agg_func, ::protobuf::RepeatedField::new())
    }

    pub fn get_agg_func(&self) -> &[super::zeus_expr::Expression] {
        &self.agg_func
    }

    fn get_agg_func_for_reflect(&self) -> &::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &self.agg_func
    }

    fn mut_agg_func_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::zeus_expr::Expression> {
        &mut self.agg_func
    }
}

impl ::protobuf::Message for AggregationNode {
    fn is_initialized(&self) -> bool {
        for v in &self.group_by {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.agg_func {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.group_by)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.agg_func)?;
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
        for value in &self.group_by {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.agg_func {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.group_by {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.agg_func {
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

impl ::protobuf::MessageStatic for AggregationNode {
    fn new() -> AggregationNode {
        AggregationNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<AggregationNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_expr::Expression>>(
                    "group_by",
                    AggregationNode::get_group_by_for_reflect,
                    AggregationNode::mut_group_by_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_expr::Expression>>(
                    "agg_func",
                    AggregationNode::get_agg_func_for_reflect,
                    AggregationNode::mut_agg_func_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AggregationNode>(
                    "AggregationNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AggregationNode {
    fn clear(&mut self) {
        self.clear_group_by();
        self.clear_agg_func();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AggregationNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AggregationNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TopNNode {
    // message fields
    pub sort_item: ::protobuf::RepeatedField<TopNNode_SortItem>,
    pub limit: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopNNode {}

impl TopNNode {
    pub fn new() -> TopNNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopNNode {
        static mut instance: ::protobuf::lazy::Lazy<TopNNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopNNode,
        };
        unsafe {
            instance.get(TopNNode::new)
        }
    }

    // repeated .TopNNode.SortItem sort_item = 1;

    pub fn clear_sort_item(&mut self) {
        self.sort_item.clear();
    }

    // Param is passed by value, moved
    pub fn set_sort_item(&mut self, v: ::protobuf::RepeatedField<TopNNode_SortItem>) {
        self.sort_item = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sort_item(&mut self) -> &mut ::protobuf::RepeatedField<TopNNode_SortItem> {
        &mut self.sort_item
    }

    // Take field
    pub fn take_sort_item(&mut self) -> ::protobuf::RepeatedField<TopNNode_SortItem> {
        ::std::mem::replace(&mut self.sort_item, ::protobuf::RepeatedField::new())
    }

    pub fn get_sort_item(&self) -> &[TopNNode_SortItem] {
        &self.sort_item
    }

    fn get_sort_item_for_reflect(&self) -> &::protobuf::RepeatedField<TopNNode_SortItem> {
        &self.sort_item
    }

    fn mut_sort_item_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TopNNode_SortItem> {
        &mut self.sort_item
    }

    // int32 limit = 2;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: i32) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> i32 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &i32 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut i32 {
        &mut self.limit
    }
}

impl ::protobuf::Message for TopNNode {
    fn is_initialized(&self) -> bool {
        for v in &self.sort_item {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sort_item)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.limit = tmp;
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
        for value in &self.sort_item {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(2, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.sort_item {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.limit != 0 {
            os.write_int32(2, self.limit)?;
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

impl ::protobuf::MessageStatic for TopNNode {
    fn new() -> TopNNode {
        TopNNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopNNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TopNNode_SortItem>>(
                    "sort_item",
                    TopNNode::get_sort_item_for_reflect,
                    TopNNode::mut_sort_item_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "limit",
                    TopNNode::get_limit_for_reflect,
                    TopNNode::mut_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopNNode>(
                    "TopNNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopNNode {
    fn clear(&mut self) {
        self.clear_sort_item();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopNNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopNNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TopNNode_SortItem {
    // message fields
    pub expr: ::protobuf::SingularPtrField<super::zeus_expr::Expression>,
    pub desc: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopNNode_SortItem {}

impl TopNNode_SortItem {
    pub fn new() -> TopNNode_SortItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopNNode_SortItem {
        static mut instance: ::protobuf::lazy::Lazy<TopNNode_SortItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopNNode_SortItem,
        };
        unsafe {
            instance.get(TopNNode_SortItem::new)
        }
    }

    // .Expression expr = 1;

    pub fn clear_expr(&mut self) {
        self.expr.clear();
    }

    pub fn has_expr(&self) -> bool {
        self.expr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expr(&mut self, v: super::zeus_expr::Expression) {
        self.expr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expr(&mut self) -> &mut super::zeus_expr::Expression {
        if self.expr.is_none() {
            self.expr.set_default();
        }
        self.expr.as_mut().unwrap()
    }

    // Take field
    pub fn take_expr(&mut self) -> super::zeus_expr::Expression {
        self.expr.take().unwrap_or_else(|| super::zeus_expr::Expression::new())
    }

    pub fn get_expr(&self) -> &super::zeus_expr::Expression {
        self.expr.as_ref().unwrap_or_else(|| super::zeus_expr::Expression::default_instance())
    }

    fn get_expr_for_reflect(&self) -> &::protobuf::SingularPtrField<super::zeus_expr::Expression> {
        &self.expr
    }

    fn mut_expr_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::zeus_expr::Expression> {
        &mut self.expr
    }

    // bool desc = 2;

    pub fn clear_desc(&mut self) {
        self.desc = false;
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: bool) {
        self.desc = v;
    }

    pub fn get_desc(&self) -> bool {
        self.desc
    }

    fn get_desc_for_reflect(&self) -> &bool {
        &self.desc
    }

    fn mut_desc_for_reflect(&mut self) -> &mut bool {
        &mut self.desc
    }
}

impl ::protobuf::Message for TopNNode_SortItem {
    fn is_initialized(&self) -> bool {
        for v in &self.expr {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expr)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.desc = tmp;
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
        if let Some(ref v) = self.expr.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.desc != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.expr.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.desc != false {
            os.write_bool(2, self.desc)?;
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

impl ::protobuf::MessageStatic for TopNNode_SortItem {
    fn new() -> TopNNode_SortItem {
        TopNNode_SortItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopNNode_SortItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_expr::Expression>>(
                    "expr",
                    TopNNode_SortItem::get_expr_for_reflect,
                    TopNNode_SortItem::mut_expr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "desc",
                    TopNNode_SortItem::get_desc_for_reflect,
                    TopNNode_SortItem::mut_desc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopNNode_SortItem>(
                    "TopNNode_SortItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopNNode_SortItem {
    fn clear(&mut self) {
        self.clear_expr();
        self.clear_desc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopNNode_SortItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopNNode_SortItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LimitNode {
    // message fields
    pub limit: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LimitNode {}

impl LimitNode {
    pub fn new() -> LimitNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LimitNode {
        static mut instance: ::protobuf::lazy::Lazy<LimitNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LimitNode,
        };
        unsafe {
            instance.get(LimitNode::new)
        }
    }

    // int32 limit = 1;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: i32) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> i32 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &i32 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut i32 {
        &mut self.limit
    }
}

impl ::protobuf::Message for LimitNode {
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
                    self.limit = tmp;
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
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(1, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.limit != 0 {
            os.write_int32(1, self.limit)?;
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

impl ::protobuf::MessageStatic for LimitNode {
    fn new() -> LimitNode {
        LimitNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<LimitNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "limit",
                    LimitNode::get_limit_for_reflect,
                    LimitNode::mut_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LimitNode>(
                    "LimitNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LimitNode {
    fn clear(&mut self) {
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LimitNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LimitNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetRowNumNode {
    // message fields
    pub db_id: i32,
    pub table_id: i32,
    pub alias: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetRowNumNode {}

impl GetRowNumNode {
    pub fn new() -> GetRowNumNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRowNumNode {
        static mut instance: ::protobuf::lazy::Lazy<GetRowNumNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRowNumNode,
        };
        unsafe {
            instance.get(GetRowNumNode::new)
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

    // string alias = 3;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alias, ::std::string::String::new())
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    fn get_alias_for_reflect(&self) -> &::std::string::String {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }
}

impl ::protobuf::Message for GetRowNumNode {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alias)?;
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
        if !self.alias.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.alias);
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
        if !self.alias.is_empty() {
            os.write_string(3, &self.alias)?;
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

impl ::protobuf::MessageStatic for GetRowNumNode {
    fn new() -> GetRowNumNode {
        GetRowNumNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRowNumNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "db_id",
                    GetRowNumNode::get_db_id_for_reflect,
                    GetRowNumNode::mut_db_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "table_id",
                    GetRowNumNode::get_table_id_for_reflect,
                    GetRowNumNode::mut_table_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    GetRowNumNode::get_alias_for_reflect,
                    GetRowNumNode::mut_alias_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRowNumNode>(
                    "GetRowNumNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRowNumNode {
    fn clear(&mut self) {
        self.clear_db_id();
        self.clear_table_id();
        self.clear_alias();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetRowNumNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRowNumNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlanNode {
    // message fields
    pub node_id: i32,
    pub children: ::protobuf::RepeatedField<PlanNode>,
    pub plan_node_type: PlanNodeType,
    pub scan_node: ::protobuf::SingularPtrField<ScanNode>,
    pub filter_node: ::protobuf::SingularPtrField<FilterNode>,
    pub project_node: ::protobuf::SingularPtrField<ProjectNode>,
    pub agg_node: ::protobuf::SingularPtrField<AggregationNode>,
    pub topn_node: ::protobuf::SingularPtrField<TopNNode>,
    pub limit_node: ::protobuf::SingularPtrField<LimitNode>,
    pub get_row_num_node: ::protobuf::SingularPtrField<GetRowNumNode>,
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

    // repeated .PlanNode children = 2;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<PlanNode>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<PlanNode> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<PlanNode> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[PlanNode] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<PlanNode> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PlanNode> {
        &mut self.children
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

    // .FilterNode filter_node = 5;

    pub fn clear_filter_node(&mut self) {
        self.filter_node.clear();
    }

    pub fn has_filter_node(&self) -> bool {
        self.filter_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filter_node(&mut self, v: FilterNode) {
        self.filter_node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filter_node(&mut self) -> &mut FilterNode {
        if self.filter_node.is_none() {
            self.filter_node.set_default();
        }
        self.filter_node.as_mut().unwrap()
    }

    // Take field
    pub fn take_filter_node(&mut self) -> FilterNode {
        self.filter_node.take().unwrap_or_else(|| FilterNode::new())
    }

    pub fn get_filter_node(&self) -> &FilterNode {
        self.filter_node.as_ref().unwrap_or_else(|| FilterNode::default_instance())
    }

    fn get_filter_node_for_reflect(&self) -> &::protobuf::SingularPtrField<FilterNode> {
        &self.filter_node
    }

    fn mut_filter_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FilterNode> {
        &mut self.filter_node
    }

    // .ProjectNode project_node = 6;

    pub fn clear_project_node(&mut self) {
        self.project_node.clear();
    }

    pub fn has_project_node(&self) -> bool {
        self.project_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project_node(&mut self, v: ProjectNode) {
        self.project_node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project_node(&mut self) -> &mut ProjectNode {
        if self.project_node.is_none() {
            self.project_node.set_default();
        }
        self.project_node.as_mut().unwrap()
    }

    // Take field
    pub fn take_project_node(&mut self) -> ProjectNode {
        self.project_node.take().unwrap_or_else(|| ProjectNode::new())
    }

    pub fn get_project_node(&self) -> &ProjectNode {
        self.project_node.as_ref().unwrap_or_else(|| ProjectNode::default_instance())
    }

    fn get_project_node_for_reflect(&self) -> &::protobuf::SingularPtrField<ProjectNode> {
        &self.project_node
    }

    fn mut_project_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ProjectNode> {
        &mut self.project_node
    }

    // .AggregationNode agg_node = 7;

    pub fn clear_agg_node(&mut self) {
        self.agg_node.clear();
    }

    pub fn has_agg_node(&self) -> bool {
        self.agg_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agg_node(&mut self, v: AggregationNode) {
        self.agg_node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agg_node(&mut self) -> &mut AggregationNode {
        if self.agg_node.is_none() {
            self.agg_node.set_default();
        }
        self.agg_node.as_mut().unwrap()
    }

    // Take field
    pub fn take_agg_node(&mut self) -> AggregationNode {
        self.agg_node.take().unwrap_or_else(|| AggregationNode::new())
    }

    pub fn get_agg_node(&self) -> &AggregationNode {
        self.agg_node.as_ref().unwrap_or_else(|| AggregationNode::default_instance())
    }

    fn get_agg_node_for_reflect(&self) -> &::protobuf::SingularPtrField<AggregationNode> {
        &self.agg_node
    }

    fn mut_agg_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AggregationNode> {
        &mut self.agg_node
    }

    // .TopNNode topn_node = 8;

    pub fn clear_topn_node(&mut self) {
        self.topn_node.clear();
    }

    pub fn has_topn_node(&self) -> bool {
        self.topn_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topn_node(&mut self, v: TopNNode) {
        self.topn_node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_topn_node(&mut self) -> &mut TopNNode {
        if self.topn_node.is_none() {
            self.topn_node.set_default();
        }
        self.topn_node.as_mut().unwrap()
    }

    // Take field
    pub fn take_topn_node(&mut self) -> TopNNode {
        self.topn_node.take().unwrap_or_else(|| TopNNode::new())
    }

    pub fn get_topn_node(&self) -> &TopNNode {
        self.topn_node.as_ref().unwrap_or_else(|| TopNNode::default_instance())
    }

    fn get_topn_node_for_reflect(&self) -> &::protobuf::SingularPtrField<TopNNode> {
        &self.topn_node
    }

    fn mut_topn_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TopNNode> {
        &mut self.topn_node
    }

    // .LimitNode limit_node = 9;

    pub fn clear_limit_node(&mut self) {
        self.limit_node.clear();
    }

    pub fn has_limit_node(&self) -> bool {
        self.limit_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_node(&mut self, v: LimitNode) {
        self.limit_node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_limit_node(&mut self) -> &mut LimitNode {
        if self.limit_node.is_none() {
            self.limit_node.set_default();
        }
        self.limit_node.as_mut().unwrap()
    }

    // Take field
    pub fn take_limit_node(&mut self) -> LimitNode {
        self.limit_node.take().unwrap_or_else(|| LimitNode::new())
    }

    pub fn get_limit_node(&self) -> &LimitNode {
        self.limit_node.as_ref().unwrap_or_else(|| LimitNode::default_instance())
    }

    fn get_limit_node_for_reflect(&self) -> &::protobuf::SingularPtrField<LimitNode> {
        &self.limit_node
    }

    fn mut_limit_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LimitNode> {
        &mut self.limit_node
    }

    // .GetRowNumNode get_row_num_node = 10;

    pub fn clear_get_row_num_node(&mut self) {
        self.get_row_num_node.clear();
    }

    pub fn has_get_row_num_node(&self) -> bool {
        self.get_row_num_node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_row_num_node(&mut self, v: GetRowNumNode) {
        self.get_row_num_node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_row_num_node(&mut self) -> &mut GetRowNumNode {
        if self.get_row_num_node.is_none() {
            self.get_row_num_node.set_default();
        }
        self.get_row_num_node.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_row_num_node(&mut self) -> GetRowNumNode {
        self.get_row_num_node.take().unwrap_or_else(|| GetRowNumNode::new())
    }

    pub fn get_get_row_num_node(&self) -> &GetRowNumNode {
        self.get_row_num_node.as_ref().unwrap_or_else(|| GetRowNumNode::default_instance())
    }

    fn get_get_row_num_node_for_reflect(&self) -> &::protobuf::SingularPtrField<GetRowNumNode> {
        &self.get_row_num_node
    }

    fn mut_get_row_num_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GetRowNumNode> {
        &mut self.get_row_num_node
    }
}

impl ::protobuf::Message for PlanNode {
    fn is_initialized(&self) -> bool {
        for v in &self.children {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.scan_node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filter_node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.project_node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.agg_node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.topn_node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.limit_node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.get_row_num_node {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
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
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filter_node)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.project_node)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agg_node)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.topn_node)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.limit_node)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_row_num_node)?;
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
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.plan_node_type != PlanNodeType::SCAN_NODE {
            my_size += ::protobuf::rt::enum_size(3, self.plan_node_type);
        }
        if let Some(ref v) = self.scan_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.filter_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.project_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.agg_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.topn_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.limit_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.get_row_num_node.as_ref() {
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
        for v in &self.children {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.plan_node_type != PlanNodeType::SCAN_NODE {
            os.write_enum(3, self.plan_node_type.value())?;
        }
        if let Some(ref v) = self.scan_node.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.filter_node.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.project_node.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.agg_node.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.topn_node.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.limit_node.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.get_row_num_node.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlanNode>>(
                    "children",
                    PlanNode::get_children_for_reflect,
                    PlanNode::mut_children_for_reflect,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FilterNode>>(
                    "filter_node",
                    PlanNode::get_filter_node_for_reflect,
                    PlanNode::mut_filter_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProjectNode>>(
                    "project_node",
                    PlanNode::get_project_node_for_reflect,
                    PlanNode::mut_project_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AggregationNode>>(
                    "agg_node",
                    PlanNode::get_agg_node_for_reflect,
                    PlanNode::mut_agg_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TopNNode>>(
                    "topn_node",
                    PlanNode::get_topn_node_for_reflect,
                    PlanNode::mut_topn_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LimitNode>>(
                    "limit_node",
                    PlanNode::get_limit_node_for_reflect,
                    PlanNode::mut_limit_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GetRowNumNode>>(
                    "get_row_num_node",
                    PlanNode::get_get_row_num_node_for_reflect,
                    PlanNode::mut_get_row_num_node_for_reflect,
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
        self.clear_children();
        self.clear_plan_node_type();
        self.clear_scan_node();
        self.clear_filter_node();
        self.clear_project_node();
        self.clear_agg_node();
        self.clear_topn_node();
        self.clear_limit_node();
        self.clear_get_row_num_node();
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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PlanNodeType {
    SCAN_NODE = 0,
    FILTER_NODE = 1,
    PROJECT_NODE = 2,
    AGGREGATE_NODE = 3,
    TOPN_NODE = 4,
    LIMIT_NODE = 5,
    GET_ROW_NUM_NODE = 6,
}

impl ::protobuf::ProtobufEnum for PlanNodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PlanNodeType> {
        match value {
            0 => ::std::option::Option::Some(PlanNodeType::SCAN_NODE),
            1 => ::std::option::Option::Some(PlanNodeType::FILTER_NODE),
            2 => ::std::option::Option::Some(PlanNodeType::PROJECT_NODE),
            3 => ::std::option::Option::Some(PlanNodeType::AGGREGATE_NODE),
            4 => ::std::option::Option::Some(PlanNodeType::TOPN_NODE),
            5 => ::std::option::Option::Some(PlanNodeType::LIMIT_NODE),
            6 => ::std::option::Option::Some(PlanNodeType::GET_ROW_NUM_NODE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PlanNodeType] = &[
            PlanNodeType::SCAN_NODE,
            PlanNodeType::FILTER_NODE,
            PlanNodeType::PROJECT_NODE,
            PlanNodeType::AGGREGATE_NODE,
            PlanNodeType::TOPN_NODE,
            PlanNodeType::LIMIT_NODE,
            PlanNodeType::GET_ROW_NUM_NODE,
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
    \n\x0fzeus_plan.proto\x1a\x0fzeus_expr.proto\"{\n\x08ScanNode\x12\x13\n\
    \x05db_id\x18\x01\x20\x01(\x05R\x04dbId\x12\x19\n\x08table_id\x18\x02\
    \x20\x01(\x05R\x07tableId\x12\x18\n\x07columns\x18\x03\x20\x03(\x05R\x07\
    columns\x12%\n\x07filters\x18\x04\x20\x03(\x0b2\x0b.ExpressionR\x07filte\
    rs\"9\n\nFilterNode\x12+\n\nconditions\x18\x01\x20\x03(\x0b2\x0b.Express\
    ionR\nconditions\"\x8f\x01\n\x0bProjectNode\x12.\n\x05items\x18\x01\x20\
    \x03(\x0b2\x18.ProjectNode.ProjectItemR\x05items\x1aP\n\x0bProjectItem\
    \x12+\n\nexpression\x18\x01\x20\x01(\x0b2\x0b.ExpressionR\nexpression\
    \x12\x14\n\x05alias\x18\x02\x20\x01(\tR\x05alias\"a\n\x0fAggregationNode\
    \x12&\n\x08group_by\x18\x01\x20\x03(\x0b2\x0b.ExpressionR\x07groupBy\x12\
    &\n\x08agg_func\x18\x02\x20\x03(\x0b2\x0b.ExpressionR\x07aggFunc\"\x92\
    \x01\n\x08TopNNode\x12/\n\tsort_item\x18\x01\x20\x03(\x0b2\x12.TopNNode.\
    SortItemR\x08sortItem\x12\x14\n\x05limit\x18\x02\x20\x01(\x05R\x05limit\
    \x1a?\n\x08SortItem\x12\x1f\n\x04expr\x18\x01\x20\x01(\x0b2\x0b.Expressi\
    onR\x04expr\x12\x12\n\x04desc\x18\x02\x20\x01(\x08R\x04desc\"!\n\tLimitN\
    ode\x12\x14\n\x05limit\x18\x01\x20\x01(\x05R\x05limit\"U\n\rGetRowNumNod\
    e\x12\x13\n\x05db_id\x18\x01\x20\x01(\x05R\x04dbId\x12\x19\n\x08table_id\
    \x18\x02\x20\x01(\x05R\x07tableId\x12\x14\n\x05alias\x18\x03\x20\x01(\tR\
    \x05alias\"\xbf\x03\n\x08PlanNode\x12\x17\n\x07node_id\x18\x01\x20\x01(\
    \x05R\x06nodeId\x12%\n\x08children\x18\x02\x20\x03(\x0b2\t.PlanNodeR\x08\
    children\x123\n\x0eplan_node_type\x18\x03\x20\x01(\x0e2\r.PlanNodeTypeR\
    \x0cplanNodeType\x12&\n\tscan_node\x18\x04\x20\x01(\x0b2\t.ScanNodeR\x08\
    scanNode\x12,\n\x0bfilter_node\x18\x05\x20\x01(\x0b2\x0b.FilterNodeR\nfi\
    lterNode\x12/\n\x0cproject_node\x18\x06\x20\x01(\x0b2\x0c.ProjectNodeR\
    \x0bprojectNode\x12+\n\x08agg_node\x18\x07\x20\x01(\x0b2\x10.Aggregation\
    NodeR\x07aggNode\x12&\n\ttopn_node\x18\x08\x20\x01(\x0b2\t.TopNNodeR\x08\
    topnNode\x12)\n\nlimit_node\x18\t\x20\x01(\x0b2\n.LimitNodeR\tlimitNode\
    \x127\n\x10get_row_num_node\x18\n\x20\x01(\x0b2\x0e.GetRowNumNodeR\rgetR\
    owNumNode*\x89\x01\n\x0cPlanNodeType\x12\r\n\tSCAN_NODE\x10\0\x12\x0f\n\
    \x0bFILTER_NODE\x10\x01\x12\x10\n\x0cPROJECT_NODE\x10\x02\x12\x12\n\x0eA\
    GGREGATE_NODE\x10\x03\x12\r\n\tTOPN_NODE\x10\x04\x12\x0e\n\nLIMIT_NODE\
    \x10\x05\x12\x14\n\x10GET_ROW_NUM_NODE\x10\x06B\x16\n\x12io.github.zeus.\
    rpcP\x01J\xb8\x15\n\x06\x12\x04\0\0I\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\
    \n\t\n\x02\x03\0\x12\x03\x02\x07\x18\n\x08\n\x01\x08\x12\x03\x04\0+\n\
    \x0b\n\x04\x08\xe7\x07\0\x12\x03\x04\0+\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x04\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x04\x07\x13\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x04\x07\x13\n\x0c\n\x05\x08\
    \xe7\x07\0\x07\x12\x03\x04\x16*\n\x08\n\x01\x08\x12\x03\x05\0\"\n\x0b\n\
    \x04\x08\xe7\x07\x01\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\
    \x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x05\x07\x1a\n\
    \x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\x05\x08\
    \xe7\x07\x01\x03\x12\x03\x05\x1d!\n\n\n\x02\x05\0\x12\x04\x07\0\x0f\x01\
    \n\n\n\x03\x05\0\x01\x12\x03\x07\x05\x11\n\x0b\n\x04\x05\0\x02\0\x12\x03\
    \x08\x04\x12\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x08\x04\r\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03\x08\x10\x11\n\x0b\n\x04\x05\0\x02\x01\x12\x03\t\
    \x04\x14\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\t\x04\x0f\n\x0c\n\x05\x05\
    \0\x02\x01\x02\x12\x03\t\x12\x13\n\x0b\n\x04\x05\0\x02\x02\x12\x03\n\x04\
    \x15\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\n\x04\x10\n\x0c\n\x05\x05\0\
    \x02\x02\x02\x12\x03\n\x13\x14\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x0b\x04\
    \x17\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x0b\x04\x12\n\x0c\n\x05\x05\0\
    \x02\x03\x02\x12\x03\x0b\x15\x16\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x0c\
    \x04\x12\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x0c\x04\r\n\x0c\n\x05\x05\
    \0\x02\x04\x02\x12\x03\x0c\x10\x11\n\x0b\n\x04\x05\0\x02\x05\x12\x03\r\
    \x04\x13\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\r\x04\x0e\n\x0c\n\x05\x05\
    \0\x02\x05\x02\x12\x03\r\x11\x12\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x0e\
    \x04\x19\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\x0e\x04\x14\n\x0c\n\x05\
    \x05\0\x02\x06\x02\x12\x03\x0e\x17\x18\n\n\n\x02\x04\0\x12\x04\x11\0\x16\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x11\x08\x10\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x12\x04\x14\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x12\x04\x11\x12\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\x12\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x12\n\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x12\x12\x13\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x13\x04\x17\n\r\n\x05\x04\0\x02\x01\x04\x12\
    \x04\x13\x04\x12\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x13\x04\t\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x13\n\x12\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03\x13\x15\x16\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x14\x04\x1f\n\
    \x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x14\x04\x0c\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\x14\r\x12\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x14\x13\
    \x1a\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x14\x1d\x1e\n\x0b\n\x04\x04\0\
    \x02\x03\x12\x03\x15\x04$\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x15\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x15\r\x17\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\x15\x18\x1f\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\
    \x15\"#\n\n\n\x02\x04\x01\x12\x04\x18\0\x1a\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x18\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x19\x04'\n\x0c\n\
    \x05\x04\x01\x02\0\x04\x12\x03\x19\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x06\
    \x12\x03\x19\r\x17\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x19\x18\"\n\x0c\
    \n\x05\x04\x01\x02\0\x03\x12\x03\x19%&\n\n\n\x02\x04\x02\x12\x04\x1c\0\"\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x1c\x08\x13\n\x0c\n\x04\x04\x02\x03\0\
    \x12\x04\x1d\x04\x20\x05\n\x0c\n\x05\x04\x02\x03\0\x01\x12\x03\x1d\x0c\
    \x17\n\r\n\x06\x04\x02\x03\0\x02\0\x12\x03\x1e\x07!\n\x0f\n\x07\x04\x02\
    \x03\0\x02\0\x04\x12\x04\x1e\x07\x1d\x19\n\x0e\n\x07\x04\x02\x03\0\x02\0\
    \x06\x12\x03\x1e\x07\x11\n\x0e\n\x07\x04\x02\x03\0\x02\0\x01\x12\x03\x1e\
    \x12\x1c\n\x0e\n\x07\x04\x02\x03\0\x02\0\x03\x12\x03\x1e\x1f\x20\n\r\n\
    \x06\x04\x02\x03\0\x02\x01\x12\x03\x1f\x07\x18\n\x0f\n\x07\x04\x02\x03\0\
    \x02\x01\x04\x12\x04\x1f\x07\x1e!\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x05\
    \x12\x03\x1f\x07\r\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x03\x1f\x0e\
    \x13\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03\x1f\x16\x17\n\x0b\n\
    \x04\x04\x02\x02\0\x12\x03!\x04#\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03!\
    \x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03!\r\x18\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03!\x19\x1e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03!!\
    \"\n\n\n\x02\x04\x03\x12\x04$\0'\x01\n\n\n\x03\x04\x03\x01\x12\x03$\x08\
    \x17\n\x0b\n\x04\x04\x03\x02\0\x12\x03%\x04%\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03%\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03%\r\x17\n\x0c\
    \n\x05\x04\x03\x02\0\x01\x12\x03%\x18\x20\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03%#$\n\x0b\n\x04\x04\x03\x02\x01\x12\x03&\x04%\n\x0c\n\x05\x04\
    \x03\x02\x01\x04\x12\x03&\x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\
    \x03&\r\x17\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03&\x18\x20\n\x0c\n\x05\
    \x04\x03\x02\x01\x03\x12\x03&#$\n\n\n\x02\x04\x04\x12\x04)\01\x01\n\n\n\
    \x03\x04\x04\x01\x12\x03)\x08\x10\n\x0c\n\x04\x04\x04\x03\0\x12\x04*\x04\
    -\x05\n\x0c\n\x05\x04\x04\x03\0\x01\x12\x03*\x0c\x14\n\r\n\x06\x04\x04\
    \x03\0\x02\0\x12\x03+\x08\x1c\n\x0f\n\x07\x04\x04\x03\0\x02\0\x04\x12\
    \x04+\x08*\x16\n\x0e\n\x07\x04\x04\x03\0\x02\0\x06\x12\x03+\x08\x12\n\
    \x0e\n\x07\x04\x04\x03\0\x02\0\x01\x12\x03+\x13\x17\n\x0e\n\x07\x04\x04\
    \x03\0\x02\0\x03\x12\x03+\x1a\x1b\n\r\n\x06\x04\x04\x03\0\x02\x01\x12\
    \x03,\x08\x16\n\x0f\n\x07\x04\x04\x03\0\x02\x01\x04\x12\x04,\x08+\x1c\n\
    \x0e\n\x07\x04\x04\x03\0\x02\x01\x05\x12\x03,\x08\x0c\n\x0e\n\x07\x04\
    \x04\x03\0\x02\x01\x01\x12\x03,\r\x11\n\x0e\n\x07\x04\x04\x03\0\x02\x01\
    \x03\x12\x03,\x14\x15\n\x0b\n\x04\x04\x04\x02\0\x12\x03/\x04$\n\x0c\n\
    \x05\x04\x04\x02\0\x04\x12\x03/\x04\x0c\n\x0c\n\x05\x04\x04\x02\0\x06\
    \x12\x03/\r\x15\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03/\x16\x1f\n\x0c\n\
    \x05\x04\x04\x02\0\x03\x12\x03/\"#\n\x0b\n\x04\x04\x04\x02\x01\x12\x030\
    \x04\x14\n\r\n\x05\x04\x04\x02\x01\x04\x12\x040\x04/$\n\x0c\n\x05\x04\
    \x04\x02\x01\x05\x12\x030\x04\t\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x030\
    \n\x0f\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x030\x12\x13\n\n\n\x02\x04\
    \x05\x12\x043\05\x01\n\n\n\x03\x04\x05\x01\x12\x033\x08\x11\n\x0b\n\x04\
    \x04\x05\x02\0\x12\x034\x04\x14\n\r\n\x05\x04\x05\x02\0\x04\x12\x044\x04\
    3\x13\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x034\x04\t\n\x0c\n\x05\x04\x05\
    \x02\0\x01\x12\x034\n\x0f\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x034\x12\x13\
    \n\n\n\x02\x04\x06\x12\x047\0;\x01\n\n\n\x03\x04\x06\x01\x12\x037\x08\
    \x15\n\x0b\n\x04\x04\x06\x02\0\x12\x038\x04\x14\n\r\n\x05\x04\x06\x02\0\
    \x04\x12\x048\x047\x17\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x038\x04\t\n\
    \x0c\n\x05\x04\x06\x02\0\x01\x12\x038\n\x0f\n\x0c\n\x05\x04\x06\x02\0\
    \x03\x12\x038\x12\x13\n\x0b\n\x04\x04\x06\x02\x01\x12\x039\x04\x17\n\r\n\
    \x05\x04\x06\x02\x01\x04\x12\x049\x048\x14\n\x0c\n\x05\x04\x06\x02\x01\
    \x05\x12\x039\x04\t\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x039\n\x12\n\x0c\
    \n\x05\x04\x06\x02\x01\x03\x12\x039\x15\x16\n\x0b\n\x04\x04\x06\x02\x02\
    \x12\x03:\x04\x15\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04:\x049\x17\n\x0c\
    \n\x05\x04\x06\x02\x02\x05\x12\x03:\x04\n\n\x0c\n\x05\x04\x06\x02\x02\
    \x01\x12\x03:\x0b\x10\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03:\x13\x14\n\
    \n\n\x02\x04\x07\x12\x04=\0I\x01\n\n\n\x03\x04\x07\x01\x12\x03=\x08\x10\
    \n\x0b\n\x04\x04\x07\x02\0\x12\x03>\x04\x16\n\r\n\x05\x04\x07\x02\0\x04\
    \x12\x04>\x04=\x12\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03>\x04\t\n\x0c\n\
    \x05\x04\x07\x02\0\x01\x12\x03>\n\x11\n\x0c\n\x05\x04\x07\x02\0\x03\x12\
    \x03>\x14\x15\n\x0b\n\x04\x04\x07\x02\x01\x12\x03?\x04#\n\x0c\n\x05\x04\
    \x07\x02\x01\x04\x12\x03?\x04\x0c\n\x0c\n\x05\x04\x07\x02\x01\x06\x12\
    \x03?\r\x15\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03?\x16\x1e\n\x0c\n\x05\
    \x04\x07\x02\x01\x03\x12\x03?!\"\n\x0b\n\x04\x04\x07\x02\x02\x12\x03@\
    \x04$\n\r\n\x05\x04\x07\x02\x02\x04\x12\x04@\x04?#\n\x0c\n\x05\x04\x07\
    \x02\x02\x06\x12\x03@\x04\x10\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03@\
    \x11\x1f\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03@\"#\n\x0b\n\x04\x04\x07\
    \x02\x03\x12\x03B\x04\x1b\n\r\n\x05\x04\x07\x02\x03\x04\x12\x04B\x04@$\n\
    \x0c\n\x05\x04\x07\x02\x03\x06\x12\x03B\x04\x0c\n\x0c\n\x05\x04\x07\x02\
    \x03\x01\x12\x03B\r\x16\n\x0c\n\x05\x04\x07\x02\x03\x03\x12\x03B\x19\x1a\
    \n\x0b\n\x04\x04\x07\x02\x04\x12\x03C\x04\x1f\n\r\n\x05\x04\x07\x02\x04\
    \x04\x12\x04C\x04B\x1b\n\x0c\n\x05\x04\x07\x02\x04\x06\x12\x03C\x04\x0e\
    \n\x0c\n\x05\x04\x07\x02\x04\x01\x12\x03C\x0f\x1a\n\x0c\n\x05\x04\x07\
    \x02\x04\x03\x12\x03C\x1d\x1e\n\x0b\n\x04\x04\x07\x02\x05\x12\x03D\x04!\
    \n\r\n\x05\x04\x07\x02\x05\x04\x12\x04D\x04C\x1f\n\x0c\n\x05\x04\x07\x02\
    \x05\x06\x12\x03D\x04\x0f\n\x0c\n\x05\x04\x07\x02\x05\x01\x12\x03D\x10\
    \x1c\n\x0c\n\x05\x04\x07\x02\x05\x03\x12\x03D\x1f\x20\n\x0b\n\x04\x04\
    \x07\x02\x06\x12\x03E\x04!\n\r\n\x05\x04\x07\x02\x06\x04\x12\x04E\x04D!\
    \n\x0c\n\x05\x04\x07\x02\x06\x06\x12\x03E\x04\x13\n\x0c\n\x05\x04\x07\
    \x02\x06\x01\x12\x03E\x14\x1c\n\x0c\n\x05\x04\x07\x02\x06\x03\x12\x03E\
    \x1f\x20\n\x0b\n\x04\x04\x07\x02\x07\x12\x03F\x04\x1b\n\r\n\x05\x04\x07\
    \x02\x07\x04\x12\x04F\x04E!\n\x0c\n\x05\x04\x07\x02\x07\x06\x12\x03F\x04\
    \x0c\n\x0c\n\x05\x04\x07\x02\x07\x01\x12\x03F\r\x16\n\x0c\n\x05\x04\x07\
    \x02\x07\x03\x12\x03F\x19\x1a\n\x0b\n\x04\x04\x07\x02\x08\x12\x03G\x04\
    \x1d\n\r\n\x05\x04\x07\x02\x08\x04\x12\x04G\x04F\x1b\n\x0c\n\x05\x04\x07\
    \x02\x08\x06\x12\x03G\x04\r\n\x0c\n\x05\x04\x07\x02\x08\x01\x12\x03G\x0e\
    \x18\n\x0c\n\x05\x04\x07\x02\x08\x03\x12\x03G\x1b\x1c\n\x0b\n\x04\x04\
    \x07\x02\t\x12\x03H\x04(\n\r\n\x05\x04\x07\x02\t\x04\x12\x04H\x04G\x1d\n\
    \x0c\n\x05\x04\x07\x02\t\x06\x12\x03H\x04\x11\n\x0c\n\x05\x04\x07\x02\t\
    \x01\x12\x03H\x12\"\n\x0c\n\x05\x04\x07\x02\t\x03\x12\x03H%'b\x06proto3\
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
