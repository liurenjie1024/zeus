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
pub struct SegmentIndex {
    // message fields
    pub format_version: i32,
    pub magic_number: i32,
    pub block_node: ::protobuf::RepeatedField<BlockNode>,
    pub block_index: ::protobuf::RepeatedField<BlockIndex>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SegmentIndex {}

impl SegmentIndex {
    pub fn new() -> SegmentIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SegmentIndex {
        static mut instance: ::protobuf::lazy::Lazy<SegmentIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SegmentIndex,
        };
        unsafe {
            instance.get(SegmentIndex::new)
        }
    }

    // int32 format_version = 1;

    pub fn clear_format_version(&mut self) {
        self.format_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_format_version(&mut self, v: i32) {
        self.format_version = v;
    }

    pub fn get_format_version(&self) -> i32 {
        self.format_version
    }

    fn get_format_version_for_reflect(&self) -> &i32 {
        &self.format_version
    }

    fn mut_format_version_for_reflect(&mut self) -> &mut i32 {
        &mut self.format_version
    }

    // int32 magic_number = 2;

    pub fn clear_magic_number(&mut self) {
        self.magic_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_magic_number(&mut self, v: i32) {
        self.magic_number = v;
    }

    pub fn get_magic_number(&self) -> i32 {
        self.magic_number
    }

    fn get_magic_number_for_reflect(&self) -> &i32 {
        &self.magic_number
    }

    fn mut_magic_number_for_reflect(&mut self) -> &mut i32 {
        &mut self.magic_number
    }

    // repeated .BlockNode block_node = 3;

    pub fn clear_block_node(&mut self) {
        self.block_node.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_node(&mut self, v: ::protobuf::RepeatedField<BlockNode>) {
        self.block_node = v;
    }

    // Mutable pointer to the field.
    pub fn mut_block_node(&mut self) -> &mut ::protobuf::RepeatedField<BlockNode> {
        &mut self.block_node
    }

    // Take field
    pub fn take_block_node(&mut self) -> ::protobuf::RepeatedField<BlockNode> {
        ::std::mem::replace(&mut self.block_node, ::protobuf::RepeatedField::new())
    }

    pub fn get_block_node(&self) -> &[BlockNode] {
        &self.block_node
    }

    fn get_block_node_for_reflect(&self) -> &::protobuf::RepeatedField<BlockNode> {
        &self.block_node
    }

    fn mut_block_node_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BlockNode> {
        &mut self.block_node
    }

    // repeated .BlockIndex block_index = 4;

    pub fn clear_block_index(&mut self) {
        self.block_index.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_index(&mut self, v: ::protobuf::RepeatedField<BlockIndex>) {
        self.block_index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_block_index(&mut self) -> &mut ::protobuf::RepeatedField<BlockIndex> {
        &mut self.block_index
    }

    // Take field
    pub fn take_block_index(&mut self) -> ::protobuf::RepeatedField<BlockIndex> {
        ::std::mem::replace(&mut self.block_index, ::protobuf::RepeatedField::new())
    }

    pub fn get_block_index(&self) -> &[BlockIndex] {
        &self.block_index
    }

    fn get_block_index_for_reflect(&self) -> &::protobuf::RepeatedField<BlockIndex> {
        &self.block_index
    }

    fn mut_block_index_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BlockIndex> {
        &mut self.block_index
    }
}

impl ::protobuf::Message for SegmentIndex {
    fn is_initialized(&self) -> bool {
        for v in &self.block_node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.block_index {
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
                    self.format_version = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.magic_number = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.block_node)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.block_index)?;
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
        if self.format_version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.format_version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.magic_number != 0 {
            my_size += ::protobuf::rt::value_size(2, self.magic_number, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.block_node {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.block_index {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.format_version != 0 {
            os.write_int32(1, self.format_version)?;
        }
        if self.magic_number != 0 {
            os.write_int32(2, self.magic_number)?;
        }
        for v in &self.block_node {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.block_index {
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

impl ::protobuf::MessageStatic for SegmentIndex {
    fn new() -> SegmentIndex {
        SegmentIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<SegmentIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "format_version",
                    SegmentIndex::get_format_version_for_reflect,
                    SegmentIndex::mut_format_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "magic_number",
                    SegmentIndex::get_magic_number_for_reflect,
                    SegmentIndex::mut_magic_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockNode>>(
                    "block_node",
                    SegmentIndex::get_block_node_for_reflect,
                    SegmentIndex::mut_block_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockIndex>>(
                    "block_index",
                    SegmentIndex::get_block_index_for_reflect,
                    SegmentIndex::mut_block_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SegmentIndex>(
                    "SegmentIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SegmentIndex {
    fn clear(&mut self) {
        self.clear_format_version();
        self.clear_magic_number();
        self.clear_block_node();
        self.clear_block_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SegmentIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SegmentIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ColumnNode {
    // message fields
    pub start: i64,
    pub end: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnNode {}

impl ColumnNode {
    pub fn new() -> ColumnNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnNode {
        static mut instance: ::protobuf::lazy::Lazy<ColumnNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnNode,
        };
        unsafe {
            instance.get(ColumnNode::new)
        }
    }

    // int64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = 0;
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: i64) {
        self.start = v;
    }

    pub fn get_start(&self) -> i64 {
        self.start
    }

    fn get_start_for_reflect(&self) -> &i64 {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut i64 {
        &mut self.start
    }

    // int64 end = 2;

    pub fn clear_end(&mut self) {
        self.end = 0;
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: i64) {
        self.end = v;
    }

    pub fn get_end(&self) -> i64 {
        self.end
    }

    fn get_end_for_reflect(&self) -> &i64 {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut i64 {
        &mut self.end
    }
}

impl ::protobuf::Message for ColumnNode {
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
                    let tmp = is.read_int64()?;
                    self.start = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.end = tmp;
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
        if self.start != 0 {
            my_size += ::protobuf::rt::value_size(1, self.start, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.end != 0 {
            my_size += ::protobuf::rt::value_size(2, self.end, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.start != 0 {
            os.write_int64(1, self.start)?;
        }
        if self.end != 0 {
            os.write_int64(2, self.end)?;
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

impl ::protobuf::MessageStatic for ColumnNode {
    fn new() -> ColumnNode {
        ColumnNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start",
                    ColumnNode::get_start_for_reflect,
                    ColumnNode::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "end",
                    ColumnNode::get_end_for_reflect,
                    ColumnNode::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnNode>(
                    "ColumnNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnNode {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColumnNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ColumnRangeIndex {
    // message fields
    pub min_number_value: i64,
    pub max_number_value: i64,
    pub min_string_value: ::std::string::String,
    pub max_string_value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnRangeIndex {}

impl ColumnRangeIndex {
    pub fn new() -> ColumnRangeIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnRangeIndex {
        static mut instance: ::protobuf::lazy::Lazy<ColumnRangeIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnRangeIndex,
        };
        unsafe {
            instance.get(ColumnRangeIndex::new)
        }
    }

    // int64 min_number_value = 1;

    pub fn clear_min_number_value(&mut self) {
        self.min_number_value = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_number_value(&mut self, v: i64) {
        self.min_number_value = v;
    }

    pub fn get_min_number_value(&self) -> i64 {
        self.min_number_value
    }

    fn get_min_number_value_for_reflect(&self) -> &i64 {
        &self.min_number_value
    }

    fn mut_min_number_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.min_number_value
    }

    // int64 max_number_value = 2;

    pub fn clear_max_number_value(&mut self) {
        self.max_number_value = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_number_value(&mut self, v: i64) {
        self.max_number_value = v;
    }

    pub fn get_max_number_value(&self) -> i64 {
        self.max_number_value
    }

    fn get_max_number_value_for_reflect(&self) -> &i64 {
        &self.max_number_value
    }

    fn mut_max_number_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.max_number_value
    }

    // string min_string_value = 3;

    pub fn clear_min_string_value(&mut self) {
        self.min_string_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_min_string_value(&mut self, v: ::std::string::String) {
        self.min_string_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_min_string_value(&mut self) -> &mut ::std::string::String {
        &mut self.min_string_value
    }

    // Take field
    pub fn take_min_string_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.min_string_value, ::std::string::String::new())
    }

    pub fn get_min_string_value(&self) -> &str {
        &self.min_string_value
    }

    fn get_min_string_value_for_reflect(&self) -> &::std::string::String {
        &self.min_string_value
    }

    fn mut_min_string_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.min_string_value
    }

    // string max_string_value = 4;

    pub fn clear_max_string_value(&mut self) {
        self.max_string_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_max_string_value(&mut self, v: ::std::string::String) {
        self.max_string_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_string_value(&mut self) -> &mut ::std::string::String {
        &mut self.max_string_value
    }

    // Take field
    pub fn take_max_string_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.max_string_value, ::std::string::String::new())
    }

    pub fn get_max_string_value(&self) -> &str {
        &self.max_string_value
    }

    fn get_max_string_value_for_reflect(&self) -> &::std::string::String {
        &self.max_string_value
    }

    fn mut_max_string_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.max_string_value
    }
}

impl ::protobuf::Message for ColumnRangeIndex {
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
                    let tmp = is.read_int64()?;
                    self.min_number_value = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.max_number_value = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.min_string_value)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.max_string_value)?;
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
        if self.min_number_value != 0 {
            my_size += ::protobuf::rt::value_size(1, self.min_number_value, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_number_value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.max_number_value, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.min_string_value.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.min_string_value);
        }
        if !self.max_string_value.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.max_string_value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.min_number_value != 0 {
            os.write_int64(1, self.min_number_value)?;
        }
        if self.max_number_value != 0 {
            os.write_int64(2, self.max_number_value)?;
        }
        if !self.min_string_value.is_empty() {
            os.write_string(3, &self.min_string_value)?;
        }
        if !self.max_string_value.is_empty() {
            os.write_string(4, &self.max_string_value)?;
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

impl ::protobuf::MessageStatic for ColumnRangeIndex {
    fn new() -> ColumnRangeIndex {
        ColumnRangeIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnRangeIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "min_number_value",
                    ColumnRangeIndex::get_min_number_value_for_reflect,
                    ColumnRangeIndex::mut_min_number_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max_number_value",
                    ColumnRangeIndex::get_max_number_value_for_reflect,
                    ColumnRangeIndex::mut_max_number_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "min_string_value",
                    ColumnRangeIndex::get_min_string_value_for_reflect,
                    ColumnRangeIndex::mut_min_string_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "max_string_value",
                    ColumnRangeIndex::get_max_string_value_for_reflect,
                    ColumnRangeIndex::mut_max_string_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnRangeIndex>(
                    "ColumnRangeIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnRangeIndex {
    fn clear(&mut self) {
        self.clear_min_number_value();
        self.clear_max_number_value();
        self.clear_min_string_value();
        self.clear_max_string_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColumnRangeIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnRangeIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockNode {
    // message fields
    pub start: i64,
    pub end: i64,
    pub column_node: ::std::collections::HashMap<i32, ColumnNode>,
    pub block_column_size: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockNode {}

impl BlockNode {
    pub fn new() -> BlockNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockNode {
        static mut instance: ::protobuf::lazy::Lazy<BlockNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockNode,
        };
        unsafe {
            instance.get(BlockNode::new)
        }
    }

    // int64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = 0;
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: i64) {
        self.start = v;
    }

    pub fn get_start(&self) -> i64 {
        self.start
    }

    fn get_start_for_reflect(&self) -> &i64 {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut i64 {
        &mut self.start
    }

    // int64 end = 2;

    pub fn clear_end(&mut self) {
        self.end = 0;
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: i64) {
        self.end = v;
    }

    pub fn get_end(&self) -> i64 {
        self.end
    }

    fn get_end_for_reflect(&self) -> &i64 {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut i64 {
        &mut self.end
    }

    // repeated .BlockNode.ColumnNodeEntry column_node = 3;

    pub fn clear_column_node(&mut self) {
        self.column_node.clear();
    }

    // Param is passed by value, moved
    pub fn set_column_node(&mut self, v: ::std::collections::HashMap<i32, ColumnNode>) {
        self.column_node = v;
    }

    // Mutable pointer to the field.
    pub fn mut_column_node(&mut self) -> &mut ::std::collections::HashMap<i32, ColumnNode> {
        &mut self.column_node
    }

    // Take field
    pub fn take_column_node(&mut self) -> ::std::collections::HashMap<i32, ColumnNode> {
        ::std::mem::replace(&mut self.column_node, ::std::collections::HashMap::new())
    }

    pub fn get_column_node(&self) -> &::std::collections::HashMap<i32, ColumnNode> {
        &self.column_node
    }

    fn get_column_node_for_reflect(&self) -> &::std::collections::HashMap<i32, ColumnNode> {
        &self.column_node
    }

    fn mut_column_node_for_reflect(&mut self) -> &mut ::std::collections::HashMap<i32, ColumnNode> {
        &mut self.column_node
    }

    // int32 block_column_size = 4;

    pub fn clear_block_column_size(&mut self) {
        self.block_column_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_column_size(&mut self, v: i32) {
        self.block_column_size = v;
    }

    pub fn get_block_column_size(&self) -> i32 {
        self.block_column_size
    }

    fn get_block_column_size_for_reflect(&self) -> &i32 {
        &self.block_column_size
    }

    fn mut_block_column_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.block_column_size
    }
}

impl ::protobuf::Message for BlockNode {
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
                    let tmp = is.read_int64()?;
                    self.start = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.end = tmp;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnNode>>(wire_type, is, &mut self.column_node)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.block_column_size = tmp;
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
        if self.start != 0 {
            my_size += ::protobuf::rt::value_size(1, self.start, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.end != 0 {
            my_size += ::protobuf::rt::value_size(2, self.end, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnNode>>(3, &self.column_node);
        if self.block_column_size != 0 {
            my_size += ::protobuf::rt::value_size(4, self.block_column_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.start != 0 {
            os.write_int64(1, self.start)?;
        }
        if self.end != 0 {
            os.write_int64(2, self.end)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnNode>>(3, &self.column_node, os)?;
        if self.block_column_size != 0 {
            os.write_int32(4, self.block_column_size)?;
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

impl ::protobuf::MessageStatic for BlockNode {
    fn new() -> BlockNode {
        BlockNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start",
                    BlockNode::get_start_for_reflect,
                    BlockNode::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "end",
                    BlockNode::get_end_for_reflect,
                    BlockNode::mut_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnNode>>(
                    "column_node",
                    BlockNode::get_column_node_for_reflect,
                    BlockNode::mut_column_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "block_column_size",
                    BlockNode::get_block_column_size_for_reflect,
                    BlockNode::mut_block_column_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockNode>(
                    "BlockNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockNode {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.clear_column_node();
        self.clear_block_column_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockIndex {
    // message fields
    pub column_range_index: ::std::collections::HashMap<i32, ColumnRangeIndex>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockIndex {}

impl BlockIndex {
    pub fn new() -> BlockIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockIndex {
        static mut instance: ::protobuf::lazy::Lazy<BlockIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockIndex,
        };
        unsafe {
            instance.get(BlockIndex::new)
        }
    }

    // repeated .BlockIndex.ColumnRangeIndexEntry column_range_index = 1;

    pub fn clear_column_range_index(&mut self) {
        self.column_range_index.clear();
    }

    // Param is passed by value, moved
    pub fn set_column_range_index(&mut self, v: ::std::collections::HashMap<i32, ColumnRangeIndex>) {
        self.column_range_index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_column_range_index(&mut self) -> &mut ::std::collections::HashMap<i32, ColumnRangeIndex> {
        &mut self.column_range_index
    }

    // Take field
    pub fn take_column_range_index(&mut self) -> ::std::collections::HashMap<i32, ColumnRangeIndex> {
        ::std::mem::replace(&mut self.column_range_index, ::std::collections::HashMap::new())
    }

    pub fn get_column_range_index(&self) -> &::std::collections::HashMap<i32, ColumnRangeIndex> {
        &self.column_range_index
    }

    fn get_column_range_index_for_reflect(&self) -> &::std::collections::HashMap<i32, ColumnRangeIndex> {
        &self.column_range_index
    }

    fn mut_column_range_index_for_reflect(&mut self) -> &mut ::std::collections::HashMap<i32, ColumnRangeIndex> {
        &mut self.column_range_index
    }
}

impl ::protobuf::Message for BlockIndex {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnRangeIndex>>(wire_type, is, &mut self.column_range_index)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnRangeIndex>>(1, &self.column_range_index);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnRangeIndex>>(1, &self.column_range_index, os)?;
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

impl ::protobuf::MessageStatic for BlockIndex {
    fn new() -> BlockIndex {
        BlockIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnRangeIndex>>(
                    "column_range_index",
                    BlockIndex::get_column_range_index_for_reflect,
                    BlockIndex::mut_column_range_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockIndex>(
                    "BlockIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockIndex {
    fn clear(&mut self) {
        self.clear_column_range_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"protobuf/zeus_blizard_format.proto\"\xb1\x01\n\x0cSegmentIndex\x12%\
    \n\x0eformat_version\x18\x01\x20\x01(\x05R\rformatVersion\x12!\n\x0cmagi\
    c_number\x18\x02\x20\x01(\x05R\x0bmagicNumber\x12)\n\nblock_node\x18\x03\
    \x20\x03(\x0b2\n.BlockNodeR\tblockNode\x12,\n\x0bblock_index\x18\x04\x20\
    \x03(\x0b2\x0b.BlockIndexR\nblockIndex\"4\n\nColumnNode\x12\x14\n\x05sta\
    rt\x18\x01\x20\x01(\x03R\x05start\x12\x10\n\x03end\x18\x02\x20\x01(\x03R\
    \x03end\"\xba\x01\n\x10ColumnRangeIndex\x12(\n\x10min_number_value\x18\
    \x01\x20\x01(\x03R\x0eminNumberValue\x12(\n\x10max_number_value\x18\x02\
    \x20\x01(\x03R\x0emaxNumberValue\x12(\n\x10min_string_value\x18\x03\x20\
    \x01(\tR\x0eminStringValue\x12(\n\x10max_string_value\x18\x04\x20\x01(\t\
    R\x0emaxStringValue\"\xe8\x01\n\tBlockNode\x12\x14\n\x05start\x18\x01\
    \x20\x01(\x03R\x05start\x12\x10\n\x03end\x18\x02\x20\x01(\x03R\x03end\
    \x12;\n\x0bcolumn_node\x18\x03\x20\x03(\x0b2\x1a.BlockNode.ColumnNodeEnt\
    ryR\ncolumnNode\x12*\n\x11block_column_size\x18\x04\x20\x01(\x05R\x0fblo\
    ckColumnSize\x1aJ\n\x0fColumnNodeEntry\x12\x10\n\x03key\x18\x01\x20\x01(\
    \x05R\x03key\x12!\n\x05value\x18\x02\x20\x01(\x0b2\x0b.ColumnNodeR\x05va\
    lue:\x028\x01\"\xb5\x01\n\nBlockIndex\x12O\n\x12column_range_index\x18\
    \x01\x20\x03(\x0b2!.BlockIndex.ColumnRangeIndexEntryR\x10columnRangeInde\
    x\x1aV\n\x15ColumnRangeIndexEntry\x12\x10\n\x03key\x18\x01\x20\x01(\x05R\
    \x03key\x12'\n\x05value\x18\x02\x20\x01(\x0b2\x11.ColumnRangeIndexR\x05v\
    alue:\x028\x01B!\n\x1dio.github.zeus.format.blizardP\x01J\xe0\n\n\x06\
    \x12\x04\0\0!\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x08\x12\
    \x03\x02\04\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x02\04\n\x0c\n\x05\x08\xe7\
    \x07\0\x02\x12\x03\x02\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x02\
    \x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x02\x07\x13\n\x0c\n\
    \x05\x08\xe7\x07\0\x07\x12\x03\x02\x143\n\x08\n\x01\x08\x12\x03\x03\0\
    \x20\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x03\0\x20\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x03\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x03\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x03\x07\x1a\n\
    \x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\x03\x1b\x1f\n\n\n\x02\x04\0\x12\
    \x04\x05\0\n\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x14\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\x06\x04\x1d\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x06\x04\
    \x05\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x04\t\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x06\n\x18\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x1b\
    \x1c\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x04\x1b\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x04\x07\x04\x06\x1d\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x07\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\n\x16\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x07\x19\x1a\n\x0b\n\x04\x04\0\x02\x02\x12\x03\
    \x08\x04&\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x08\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\x02\x06\x12\x03\x08\r\x16\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\x08\x17!\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x08$%\n\x0b\n\x04\
    \x04\0\x02\x03\x12\x03\t\x04(\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\t\
    \x04\x0c\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\t\r\x17\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\t\x18#\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\t&'\n\
    \n\n\x02\x04\x01\x12\x04\x0c\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0c\
    \x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\r\x04\x14\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\r\x04\x0c\x14\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\r\
    \x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\n\x0f\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\r\x12\x13\n\x18\n\x04\x04\x01\x02\x01\x12\x03\x0e\x04\
    \x12\"\x0b\x20exclusive\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x0e\x04\
    \r\x14\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0e\x04\t\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\x0e\n\r\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x0e\x10\x11\n\n\n\x02\x04\x02\x12\x04\x11\0\x16\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x11\x08\x18\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x12\x04\x1f\n\
    \r\n\x05\x04\x02\x02\0\x04\x12\x04\x12\x04\x11\x1a\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\x12\x04\t\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x12\n\
    \x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x12\x1d\x1e\n\x0b\n\x04\x04\
    \x02\x02\x01\x12\x03\x13\x04\x1f\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\
    \x13\x04\x12\x1f\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x13\x04\t\n\x0c\
    \n\x05\x04\x02\x02\x01\x01\x12\x03\x13\n\x1a\n\x0c\n\x05\x04\x02\x02\x01\
    \x03\x12\x03\x13\x1d\x1e\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x14\x04\x20\
    \n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x14\x04\x13\x1f\n\x0c\n\x05\x04\
    \x02\x02\x02\x05\x12\x03\x14\x04\n\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\
    \x03\x14\x0b\x1b\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x14\x1e\x1f\n\
    \x0b\n\x04\x04\x02\x02\x03\x12\x03\x15\x04\x20\n\r\n\x05\x04\x02\x02\x03\
    \x04\x12\x04\x15\x04\x14\x20\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x15\
    \x04\n\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03\x15\x0b\x1b\n\x0c\n\x05\
    \x04\x02\x02\x03\x03\x12\x03\x15\x1e\x1f\n\n\n\x02\x04\x03\x12\x04\x18\0\
    \x1d\x01\n\n\n\x03\x04\x03\x01\x12\x03\x18\x08\x11\n\x0b\n\x04\x04\x03\
    \x02\0\x12\x03\x19\x04\x14\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x19\x04\
    \x18\x13\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x19\x04\t\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x03\x19\n\x0f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\
    \x19\x12\x13\n\x18\n\x04\x04\x03\x02\x01\x12\x03\x1a\x04\x12\"\x0b\x20ex\
    clusive\n\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\x1a\x04\x19\x14\n\x0c\n\
    \x05\x04\x03\x02\x01\x05\x12\x03\x1a\x04\t\n\x0c\n\x05\x04\x03\x02\x01\
    \x01\x12\x03\x1a\n\r\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x1a\x10\x11\
    \n\x0b\n\x04\x04\x03\x02\x02\x12\x03\x1b\x04+\n\r\n\x05\x04\x03\x02\x02\
    \x04\x12\x04\x1b\x04\x1a\x12\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x03\x1b\
    \x04\x1a\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\x1b\x1b&\n\x0c\n\x05\
    \x04\x03\x02\x02\x03\x12\x03\x1b)*\n\x0b\n\x04\x04\x03\x02\x03\x12\x03\
    \x1c\x04\x20\n\r\n\x05\x04\x03\x02\x03\x04\x12\x04\x1c\x04\x1b+\n\x0c\n\
    \x05\x04\x03\x02\x03\x05\x12\x03\x1c\x04\t\n\x0c\n\x05\x04\x03\x02\x03\
    \x01\x12\x03\x1c\n\x1b\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x03\x1c\x1e\
    \x1f\n\n\n\x02\x04\x04\x12\x04\x1f\0!\x01\n\n\n\x03\x04\x04\x01\x12\x03\
    \x1f\x08\x12\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x20\x048\n\r\n\x05\x04\
    \x04\x02\0\x04\x12\x04\x20\x04\x1f\x14\n\x0c\n\x05\x04\x04\x02\0\x06\x12\
    \x03\x20\x04\x20\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x20!3\n\x0c\n\x05\
    \x04\x04\x02\0\x03\x12\x03\x2067b\x06proto3\
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
