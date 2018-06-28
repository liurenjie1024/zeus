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
pub struct ColumnHandle {
    // message fields
    pub start: i64,
    pub end: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnHandle {}

impl ColumnHandle {
    pub fn new() -> ColumnHandle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnHandle {
        static mut instance: ::protobuf::lazy::Lazy<ColumnHandle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnHandle,
        };
        unsafe {
            instance.get(ColumnHandle::new)
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

impl ::protobuf::Message for ColumnHandle {
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

impl ::protobuf::MessageStatic for ColumnHandle {
    fn new() -> ColumnHandle {
        ColumnHandle::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnHandle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start",
                    ColumnHandle::get_start_for_reflect,
                    ColumnHandle::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "end",
                    ColumnHandle::get_end_for_reflect,
                    ColumnHandle::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnHandle>(
                    "ColumnHandle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnHandle {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColumnHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnHandle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockHandle {
    // message fields
    pub start: i64,
    pub end: i64,
    pub columns: ::std::collections::HashMap<i32, ColumnHandle>,
    pub block_column_size: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockHandle {}

impl BlockHandle {
    pub fn new() -> BlockHandle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockHandle {
        static mut instance: ::protobuf::lazy::Lazy<BlockHandle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockHandle,
        };
        unsafe {
            instance.get(BlockHandle::new)
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

    // repeated .BlockHandle.ColumnsEntry columns = 3;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::std::collections::HashMap<i32, ColumnHandle>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::std::collections::HashMap<i32, ColumnHandle> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::std::collections::HashMap<i32, ColumnHandle> {
        ::std::mem::replace(&mut self.columns, ::std::collections::HashMap::new())
    }

    pub fn get_columns(&self) -> &::std::collections::HashMap<i32, ColumnHandle> {
        &self.columns
    }

    fn get_columns_for_reflect(&self) -> &::std::collections::HashMap<i32, ColumnHandle> {
        &self.columns
    }

    fn mut_columns_for_reflect(&mut self) -> &mut ::std::collections::HashMap<i32, ColumnHandle> {
        &mut self.columns
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

impl ::protobuf::Message for BlockHandle {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnHandle>>(wire_type, is, &mut self.columns)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnHandle>>(3, &self.columns);
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
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnHandle>>(3, &self.columns, os)?;
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

impl ::protobuf::MessageStatic for BlockHandle {
    fn new() -> BlockHandle {
        BlockHandle::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockHandle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start",
                    BlockHandle::get_start_for_reflect,
                    BlockHandle::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "end",
                    BlockHandle::get_end_for_reflect,
                    BlockHandle::mut_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeMessage<ColumnHandle>>(
                    "columns",
                    BlockHandle::get_columns_for_reflect,
                    BlockHandle::mut_columns_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "block_column_size",
                    BlockHandle::get_block_column_size_for_reflect,
                    BlockHandle::mut_block_column_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockHandle>(
                    "BlockHandle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockHandle {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.clear_columns();
        self.clear_block_column_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHandle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockHandles {
    // message fields
    pub handles: ::protobuf::RepeatedField<BlockHandle>,
    pub max_block_column_size: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockHandles {}

impl BlockHandles {
    pub fn new() -> BlockHandles {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockHandles {
        static mut instance: ::protobuf::lazy::Lazy<BlockHandles> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockHandles,
        };
        unsafe {
            instance.get(BlockHandles::new)
        }
    }

    // repeated .BlockHandle handles = 1;

    pub fn clear_handles(&mut self) {
        self.handles.clear();
    }

    // Param is passed by value, moved
    pub fn set_handles(&mut self, v: ::protobuf::RepeatedField<BlockHandle>) {
        self.handles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_handles(&mut self) -> &mut ::protobuf::RepeatedField<BlockHandle> {
        &mut self.handles
    }

    // Take field
    pub fn take_handles(&mut self) -> ::protobuf::RepeatedField<BlockHandle> {
        ::std::mem::replace(&mut self.handles, ::protobuf::RepeatedField::new())
    }

    pub fn get_handles(&self) -> &[BlockHandle] {
        &self.handles
    }

    fn get_handles_for_reflect(&self) -> &::protobuf::RepeatedField<BlockHandle> {
        &self.handles
    }

    fn mut_handles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BlockHandle> {
        &mut self.handles
    }

    // int32 max_block_column_size = 2;

    pub fn clear_max_block_column_size(&mut self) {
        self.max_block_column_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_block_column_size(&mut self, v: i32) {
        self.max_block_column_size = v;
    }

    pub fn get_max_block_column_size(&self) -> i32 {
        self.max_block_column_size
    }

    fn get_max_block_column_size_for_reflect(&self) -> &i32 {
        &self.max_block_column_size
    }

    fn mut_max_block_column_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.max_block_column_size
    }
}

impl ::protobuf::Message for BlockHandles {
    fn is_initialized(&self) -> bool {
        for v in &self.handles {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.handles)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_block_column_size = tmp;
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
        for value in &self.handles {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.max_block_column_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.max_block_column_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.handles {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.max_block_column_size != 0 {
            os.write_int32(2, self.max_block_column_size)?;
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

impl ::protobuf::MessageStatic for BlockHandles {
    fn new() -> BlockHandles {
        BlockHandles::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockHandles>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockHandle>>(
                    "handles",
                    BlockHandles::get_handles_for_reflect,
                    BlockHandles::mut_handles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_block_column_size",
                    BlockHandles::get_max_block_column_size_for_reflect,
                    BlockHandles::mut_max_block_column_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockHandles>(
                    "BlockHandles",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockHandles {
    fn clear(&mut self) {
        self.clear_handles();
        self.clear_max_block_column_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHandles {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHandles {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18zeus_simple_format.proto\"6\n\x0cColumnHandle\x12\x14\n\x05start\
    \x18\x01\x20\x01(\x03R\x05start\x12\x10\n\x03end\x18\x02\x20\x01(\x03R\
    \x03end\"\xe1\x01\n\x0bBlockHandle\x12\x14\n\x05start\x18\x01\x20\x01(\
    \x03R\x05start\x12\x10\n\x03end\x18\x02\x20\x01(\x03R\x03end\x123\n\x07c\
    olumns\x18\x03\x20\x03(\x0b2\x19.BlockHandle.ColumnsEntryR\x07columns\
    \x12*\n\x11block_column_size\x18\x04\x20\x01(\x05R\x0fblockColumnSize\
    \x1aI\n\x0cColumnsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\x05R\x03key\
    \x12#\n\x05value\x18\x02\x20\x01(\x0b2\r.ColumnHandleR\x05value:\x028\
    \x01\"i\n\x0cBlockHandles\x12&\n\x07handles\x18\x01\x20\x03(\x0b2\x0c.Bl\
    ockHandleR\x07handles\x121\n\x15max_block_column_size\x18\x02\x20\x01(\
    \x05R\x12maxBlockColumnSizeB\x20\n\x1cio.github.zeus.format.simpleP\x01J\
    \xc7\x06\n\x06\x12\x04\0\0\x14\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\
    \n\x01\x08\x12\x03\x02\03\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x02\03\n\x0c\
    \n\x05\x08\xe7\x07\0\x02\x12\x03\x02\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\
    \0\x12\x03\x02\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x02\
    \x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x02\x142\n\x08\n\x01\x08\
    \x12\x03\x03\0\x20\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x03\0\x20\n\x0c\n\
    \x05\x08\xe7\x07\x01\x02\x12\x03\x03\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\
    \x02\0\x12\x03\x03\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\
    \x03\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\x03\x1b\x1f\n\n\n\
    \x02\x04\0\x12\x04\x05\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x14\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x04\x14\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04\x06\x04\x05\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x04\t\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\n\x0f\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x06\x12\x13\n\x18\n\x04\x04\0\x02\x01\x12\x03\x07\x04\x12\"\x0b\
    \x20exclusive\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x07\x04\x06\x14\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\x04\t\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x07\n\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07\x10\x11\n\
    \n\n\x02\x04\x01\x12\x04\n\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\
    \x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x04\x14\n\r\n\x05\x04\x01\x02\
    \0\x04\x12\x04\x0b\x04\n\x15\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0b\
    \x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\n\x0f\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x0b\x12\x13\n\x18\n\x04\x04\x01\x02\x01\x12\x03\
    \x0c\x04\x12\"\x0b\x20exclusive\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\
    \x0c\x04\x0b\x14\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0c\x04\t\n\x0c\
    \n\x05\x04\x01\x02\x01\x01\x12\x03\x0c\n\r\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x0c\x10\x11\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\r\x04)\n\r\
    \n\x05\x04\x01\x02\x02\x04\x12\x04\r\x04\x0c\x12\n\x0c\n\x05\x04\x01\x02\
    \x02\x06\x12\x03\r\x04\x1c\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\r\x1d\
    $\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\r'(\n\x0b\n\x04\x04\x01\x02\
    \x03\x12\x03\x0e\x04\x20\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\x0e\x04\r\
    )\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0e\x04\t\n\x0c\n\x05\x04\x01\
    \x02\x03\x01\x12\x03\x0e\n\x1b\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\
    \x0e\x1e\x1f\n\n\n\x02\x04\x02\x12\x04\x11\0\x14\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x11\x08\x14\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x12\x04%\n\
    \x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\x02\x02\
    \0\x06\x12\x03\x12\r\x18\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x12\x19\
    \x20\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x12#$\n\x0b\n\x04\x04\x02\x02\
    \x01\x12\x03\x13\x04$\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x13\x04\x12%\
    \n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x13\x04\t\n\x0c\n\x05\x04\x02\
    \x02\x01\x01\x12\x03\x13\n\x1f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\
    \x13\"#b\x06proto3\
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
