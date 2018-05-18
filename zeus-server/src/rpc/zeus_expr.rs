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
pub struct LiteralExpression {
    // message fields
    pub field_type: super::zeus_meta::ColumnType,
    pub value: ::protobuf::SingularPtrField<super::zeus_meta::ColumnValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LiteralExpression {}

impl LiteralExpression {
    pub fn new() -> LiteralExpression {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LiteralExpression {
        static mut instance: ::protobuf::lazy::Lazy<LiteralExpression> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LiteralExpression,
        };
        unsafe {
            instance.get(LiteralExpression::new)
        }
    }

    // .ColumnType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = super::zeus_meta::ColumnType::BOOL;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: super::zeus_meta::ColumnType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> super::zeus_meta::ColumnType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &super::zeus_meta::ColumnType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut super::zeus_meta::ColumnType {
        &mut self.field_type
    }

    // .ColumnValue value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: super::zeus_meta::ColumnValue) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut super::zeus_meta::ColumnValue {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> super::zeus_meta::ColumnValue {
        self.value.take().unwrap_or_else(|| super::zeus_meta::ColumnValue::new())
    }

    pub fn get_value(&self) -> &super::zeus_meta::ColumnValue {
        self.value.as_ref().unwrap_or_else(|| super::zeus_meta::ColumnValue::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<super::zeus_meta::ColumnValue> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::zeus_meta::ColumnValue> {
        &mut self.value
    }
}

impl ::protobuf::Message for LiteralExpression {
    fn is_initialized(&self) -> bool {
        for v in &self.value {
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
                    self.field_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
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
        if self.field_type != super::zeus_meta::ColumnType::BOOL {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != super::zeus_meta::ColumnType::BOOL {
            os.write_enum(1, self.field_type.value())?;
        }
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for LiteralExpression {
    fn new() -> LiteralExpression {
        LiteralExpression::new()
    }

    fn descriptor_static(_: ::std::option::Option<LiteralExpression>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::zeus_meta::ColumnType>>(
                    "type",
                    LiteralExpression::get_field_type_for_reflect,
                    LiteralExpression::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::zeus_meta::ColumnValue>>(
                    "value",
                    LiteralExpression::get_value_for_reflect,
                    LiteralExpression::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LiteralExpression>(
                    "LiteralExpression",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LiteralExpression {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LiteralExpression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LiteralExpression {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ColumnRef {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnRef {}

impl ColumnRef {
    pub fn new() -> ColumnRef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnRef {
        static mut instance: ::protobuf::lazy::Lazy<ColumnRef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnRef,
        };
        unsafe {
            instance.get(ColumnRef::new)
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
}

impl ::protobuf::Message for ColumnRef {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for ColumnRef {
    fn new() -> ColumnRef {
        ColumnRef::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnRef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ColumnRef::get_name_for_reflect,
                    ColumnRef::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnRef>(
                    "ColumnRef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnRef {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColumnRef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnRef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScalarFunction {
    // message fields
    pub func_id: ScalarFuncId,
    pub children: ::protobuf::RepeatedField<Expression>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScalarFunction {}

impl ScalarFunction {
    pub fn new() -> ScalarFunction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScalarFunction {
        static mut instance: ::protobuf::lazy::Lazy<ScalarFunction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScalarFunction,
        };
        unsafe {
            instance.get(ScalarFunction::new)
        }
    }

    // .ScalarFuncId func_id = 1;

    pub fn clear_func_id(&mut self) {
        self.func_id = ScalarFuncId::ADD_INT4_INT4;
    }

    // Param is passed by value, moved
    pub fn set_func_id(&mut self, v: ScalarFuncId) {
        self.func_id = v;
    }

    pub fn get_func_id(&self) -> ScalarFuncId {
        self.func_id
    }

    fn get_func_id_for_reflect(&self) -> &ScalarFuncId {
        &self.func_id
    }

    fn mut_func_id_for_reflect(&mut self) -> &mut ScalarFuncId {
        &mut self.func_id
    }

    // repeated .Expression children = 2;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Expression>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Expression> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Expression> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Expression] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<Expression> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Expression> {
        &mut self.children
    }
}

impl ::protobuf::Message for ScalarFunction {
    fn is_initialized(&self) -> bool {
        for v in &self.children {
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
                    self.func_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
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
        if self.func_id != ScalarFuncId::ADD_INT4_INT4 {
            my_size += ::protobuf::rt::enum_size(1, self.func_id);
        }
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.func_id != ScalarFuncId::ADD_INT4_INT4 {
            os.write_enum(1, self.func_id.value())?;
        }
        for v in &self.children {
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

impl ::protobuf::MessageStatic for ScalarFunction {
    fn new() -> ScalarFunction {
        ScalarFunction::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScalarFunction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ScalarFuncId>>(
                    "func_id",
                    ScalarFunction::get_func_id_for_reflect,
                    ScalarFunction::mut_func_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Expression>>(
                    "children",
                    ScalarFunction::get_children_for_reflect,
                    ScalarFunction::mut_children_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScalarFunction>(
                    "ScalarFunction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScalarFunction {
    fn clear(&mut self) {
        self.clear_func_id();
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScalarFunction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScalarFunction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AggFunction {
    // message fields
    pub func_id: AggFuncId,
    pub children: ::protobuf::RepeatedField<Expression>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AggFunction {}

impl AggFunction {
    pub fn new() -> AggFunction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AggFunction {
        static mut instance: ::protobuf::lazy::Lazy<AggFunction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AggFunction,
        };
        unsafe {
            instance.get(AggFunction::new)
        }
    }

    // .AggFuncId func_id = 1;

    pub fn clear_func_id(&mut self) {
        self.func_id = AggFuncId::SUM;
    }

    // Param is passed by value, moved
    pub fn set_func_id(&mut self, v: AggFuncId) {
        self.func_id = v;
    }

    pub fn get_func_id(&self) -> AggFuncId {
        self.func_id
    }

    fn get_func_id_for_reflect(&self) -> &AggFuncId {
        &self.func_id
    }

    fn mut_func_id_for_reflect(&mut self) -> &mut AggFuncId {
        &mut self.func_id
    }

    // repeated .Expression children = 2;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Expression>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Expression> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Expression> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Expression] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<Expression> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Expression> {
        &mut self.children
    }
}

impl ::protobuf::Message for AggFunction {
    fn is_initialized(&self) -> bool {
        for v in &self.children {
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
                    self.func_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
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
        if self.func_id != AggFuncId::SUM {
            my_size += ::protobuf::rt::enum_size(1, self.func_id);
        }
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.func_id != AggFuncId::SUM {
            os.write_enum(1, self.func_id.value())?;
        }
        for v in &self.children {
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

impl ::protobuf::MessageStatic for AggFunction {
    fn new() -> AggFunction {
        AggFunction::new()
    }

    fn descriptor_static(_: ::std::option::Option<AggFunction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AggFuncId>>(
                    "func_id",
                    AggFunction::get_func_id_for_reflect,
                    AggFunction::mut_func_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Expression>>(
                    "children",
                    AggFunction::get_children_for_reflect,
                    AggFunction::mut_children_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AggFunction>(
                    "AggFunction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AggFunction {
    fn clear(&mut self) {
        self.clear_func_id();
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AggFunction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AggFunction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Expression {
    // message fields
    pub expression_type: ExpressionType,
    pub literal: ::protobuf::SingularPtrField<LiteralExpression>,
    pub column: ::protobuf::SingularPtrField<ColumnRef>,
    pub scalar_func: ::protobuf::SingularPtrField<ScalarFunction>,
    pub agg_func: ::protobuf::SingularPtrField<AggFunction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Expression {}

impl Expression {
    pub fn new() -> Expression {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Expression {
        static mut instance: ::protobuf::lazy::Lazy<Expression> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Expression,
        };
        unsafe {
            instance.get(Expression::new)
        }
    }

    // .ExpressionType expression_type = 1;

    pub fn clear_expression_type(&mut self) {
        self.expression_type = ExpressionType::LITERAL;
    }

    // Param is passed by value, moved
    pub fn set_expression_type(&mut self, v: ExpressionType) {
        self.expression_type = v;
    }

    pub fn get_expression_type(&self) -> ExpressionType {
        self.expression_type
    }

    fn get_expression_type_for_reflect(&self) -> &ExpressionType {
        &self.expression_type
    }

    fn mut_expression_type_for_reflect(&mut self) -> &mut ExpressionType {
        &mut self.expression_type
    }

    // .LiteralExpression literal = 2;

    pub fn clear_literal(&mut self) {
        self.literal.clear();
    }

    pub fn has_literal(&self) -> bool {
        self.literal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_literal(&mut self, v: LiteralExpression) {
        self.literal = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_literal(&mut self) -> &mut LiteralExpression {
        if self.literal.is_none() {
            self.literal.set_default();
        }
        self.literal.as_mut().unwrap()
    }

    // Take field
    pub fn take_literal(&mut self) -> LiteralExpression {
        self.literal.take().unwrap_or_else(|| LiteralExpression::new())
    }

    pub fn get_literal(&self) -> &LiteralExpression {
        self.literal.as_ref().unwrap_or_else(|| LiteralExpression::default_instance())
    }

    fn get_literal_for_reflect(&self) -> &::protobuf::SingularPtrField<LiteralExpression> {
        &self.literal
    }

    fn mut_literal_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LiteralExpression> {
        &mut self.literal
    }

    // .ColumnRef column = 3;

    pub fn clear_column(&mut self) {
        self.column.clear();
    }

    pub fn has_column(&self) -> bool {
        self.column.is_some()
    }

    // Param is passed by value, moved
    pub fn set_column(&mut self, v: ColumnRef) {
        self.column = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_column(&mut self) -> &mut ColumnRef {
        if self.column.is_none() {
            self.column.set_default();
        }
        self.column.as_mut().unwrap()
    }

    // Take field
    pub fn take_column(&mut self) -> ColumnRef {
        self.column.take().unwrap_or_else(|| ColumnRef::new())
    }

    pub fn get_column(&self) -> &ColumnRef {
        self.column.as_ref().unwrap_or_else(|| ColumnRef::default_instance())
    }

    fn get_column_for_reflect(&self) -> &::protobuf::SingularPtrField<ColumnRef> {
        &self.column
    }

    fn mut_column_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ColumnRef> {
        &mut self.column
    }

    // .ScalarFunction scalar_func = 4;

    pub fn clear_scalar_func(&mut self) {
        self.scalar_func.clear();
    }

    pub fn has_scalar_func(&self) -> bool {
        self.scalar_func.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scalar_func(&mut self, v: ScalarFunction) {
        self.scalar_func = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scalar_func(&mut self) -> &mut ScalarFunction {
        if self.scalar_func.is_none() {
            self.scalar_func.set_default();
        }
        self.scalar_func.as_mut().unwrap()
    }

    // Take field
    pub fn take_scalar_func(&mut self) -> ScalarFunction {
        self.scalar_func.take().unwrap_or_else(|| ScalarFunction::new())
    }

    pub fn get_scalar_func(&self) -> &ScalarFunction {
        self.scalar_func.as_ref().unwrap_or_else(|| ScalarFunction::default_instance())
    }

    fn get_scalar_func_for_reflect(&self) -> &::protobuf::SingularPtrField<ScalarFunction> {
        &self.scalar_func
    }

    fn mut_scalar_func_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ScalarFunction> {
        &mut self.scalar_func
    }

    // .AggFunction agg_func = 5;

    pub fn clear_agg_func(&mut self) {
        self.agg_func.clear();
    }

    pub fn has_agg_func(&self) -> bool {
        self.agg_func.is_some()
    }

    // Param is passed by value, moved
    pub fn set_agg_func(&mut self, v: AggFunction) {
        self.agg_func = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agg_func(&mut self) -> &mut AggFunction {
        if self.agg_func.is_none() {
            self.agg_func.set_default();
        }
        self.agg_func.as_mut().unwrap()
    }

    // Take field
    pub fn take_agg_func(&mut self) -> AggFunction {
        self.agg_func.take().unwrap_or_else(|| AggFunction::new())
    }

    pub fn get_agg_func(&self) -> &AggFunction {
        self.agg_func.as_ref().unwrap_or_else(|| AggFunction::default_instance())
    }

    fn get_agg_func_for_reflect(&self) -> &::protobuf::SingularPtrField<AggFunction> {
        &self.agg_func
    }

    fn mut_agg_func_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AggFunction> {
        &mut self.agg_func
    }
}

impl ::protobuf::Message for Expression {
    fn is_initialized(&self) -> bool {
        for v in &self.literal {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.column {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.scalar_func {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.expression_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.literal)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.column)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scalar_func)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.agg_func)?;
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
        if self.expression_type != ExpressionType::LITERAL {
            my_size += ::protobuf::rt::enum_size(1, self.expression_type);
        }
        if let Some(ref v) = self.literal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.column.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.scalar_func.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.agg_func.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.expression_type != ExpressionType::LITERAL {
            os.write_enum(1, self.expression_type.value())?;
        }
        if let Some(ref v) = self.literal.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.column.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.scalar_func.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.agg_func.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Expression {
    fn new() -> Expression {
        Expression::new()
    }

    fn descriptor_static(_: ::std::option::Option<Expression>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ExpressionType>>(
                    "expression_type",
                    Expression::get_expression_type_for_reflect,
                    Expression::mut_expression_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LiteralExpression>>(
                    "literal",
                    Expression::get_literal_for_reflect,
                    Expression::mut_literal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ColumnRef>>(
                    "column",
                    Expression::get_column_for_reflect,
                    Expression::mut_column_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScalarFunction>>(
                    "scalar_func",
                    Expression::get_scalar_func_for_reflect,
                    Expression::mut_scalar_func_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AggFunction>>(
                    "agg_func",
                    Expression::get_agg_func_for_reflect,
                    Expression::mut_agg_func_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Expression>(
                    "Expression",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Expression {
    fn clear(&mut self) {
        self.clear_expression_type();
        self.clear_literal();
        self.clear_column();
        self.clear_scalar_func();
        self.clear_agg_func();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Expression {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExpressionType {
    LITERAL = 0,
    COLUMN_REF = 1,
    SCALAR_FUNCTION = 2,
    AGG_FUNCTION = 3,
}

impl ::protobuf::ProtobufEnum for ExpressionType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExpressionType> {
        match value {
            0 => ::std::option::Option::Some(ExpressionType::LITERAL),
            1 => ::std::option::Option::Some(ExpressionType::COLUMN_REF),
            2 => ::std::option::Option::Some(ExpressionType::SCALAR_FUNCTION),
            3 => ::std::option::Option::Some(ExpressionType::AGG_FUNCTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExpressionType] = &[
            ExpressionType::LITERAL,
            ExpressionType::COLUMN_REF,
            ExpressionType::SCALAR_FUNCTION,
            ExpressionType::AGG_FUNCTION,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ExpressionType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ExpressionType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ExpressionType {
}

impl ::std::default::Default for ExpressionType {
    fn default() -> Self {
        ExpressionType::LITERAL
    }
}

impl ::protobuf::reflect::ProtobufValue for ExpressionType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ScalarFuncId {
    ADD_INT4_INT4 = 0,
    AND = 1,
}

impl ::protobuf::ProtobufEnum for ScalarFuncId {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ScalarFuncId> {
        match value {
            0 => ::std::option::Option::Some(ScalarFuncId::ADD_INT4_INT4),
            1 => ::std::option::Option::Some(ScalarFuncId::AND),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ScalarFuncId] = &[
            ScalarFuncId::ADD_INT4_INT4,
            ScalarFuncId::AND,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ScalarFuncId>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ScalarFuncId", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ScalarFuncId {
}

impl ::std::default::Default for ScalarFuncId {
    fn default() -> Self {
        ScalarFuncId::ADD_INT4_INT4
    }
}

impl ::protobuf::reflect::ProtobufValue for ScalarFuncId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AggFuncId {
    SUM = 0,
}

impl ::protobuf::ProtobufEnum for AggFuncId {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AggFuncId> {
        match value {
            0 => ::std::option::Option::Some(AggFuncId::SUM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AggFuncId] = &[
            AggFuncId::SUM,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AggFuncId>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AggFuncId", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AggFuncId {
}

impl ::std::default::Default for AggFuncId {
    fn default() -> Self {
        AggFuncId::SUM
    }
}

impl ::protobuf::reflect::ProtobufValue for AggFuncId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fzeus_expr.proto\x1a\x0fzeus_meta.proto\"X\n\x11LiteralExpression\
    \x12\x1f\n\x04type\x18\x01\x20\x01(\x0e2\x0b.ColumnTypeR\x04type\x12\"\n\
    \x05value\x18\x02\x20\x01(\x0b2\x0c.ColumnValueR\x05value\"\x1f\n\tColum\
    nRef\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"a\n\x0eScalarFunctio\
    n\x12&\n\x07func_id\x18\x01\x20\x01(\x0e2\r.ScalarFuncIdR\x06funcId\x12'\
    \n\x08children\x18\x02\x20\x03(\x0b2\x0b.ExpressionR\x08children\"[\n\
    \x0bAggFunction\x12#\n\x07func_id\x18\x01\x20\x01(\x0e2\n.AggFuncIdR\x06\
    funcId\x12'\n\x08children\x18\x02\x20\x03(\x0b2\x0b.ExpressionR\x08child\
    ren\"\xf3\x01\n\nExpression\x128\n\x0fexpression_type\x18\x01\x20\x01(\
    \x0e2\x0f.ExpressionTypeR\x0eexpressionType\x12,\n\x07literal\x18\x02\
    \x20\x01(\x0b2\x12.LiteralExpressionR\x07literal\x12\"\n\x06column\x18\
    \x03\x20\x01(\x0b2\n.ColumnRefR\x06column\x120\n\x0bscalar_func\x18\x04\
    \x20\x01(\x0b2\x0f.ScalarFunctionR\nscalarFunc\x12'\n\x08agg_func\x18\
    \x05\x20\x01(\x0b2\x0c.AggFunctionR\x07aggFunc*T\n\x0eExpressionType\x12\
    \x0b\n\x07LITERAL\x10\0\x12\x0e\n\nCOLUMN_REF\x10\x01\x12\x13\n\x0fSCALA\
    R_FUNCTION\x10\x02\x12\x10\n\x0cAGG_FUNCTION\x10\x03**\n\x0cScalarFuncId\
    \x12\x11\n\rADD_INT4_INT4\x10\0\x12\x07\n\x03AND\x10\x01*\x14\n\tAggFunc\
    Id\x12\x07\n\x03SUM\x10\0B\x16\n\x12io.github.zeus.rpcP\x01J\xfb\x0b\n\
    \x06\x12\x04\0\04\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\
    \x12\x03\x02\x07\x18\n\x08\n\x01\x08\x12\x03\x04\0+\n\x0b\n\x04\x08\xe7\
    \x07\0\x12\x03\x04\0+\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x04\x07\x13\
    \n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x04\x07\x13\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\x04\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\
    \x03\x04\x16*\n\x08\n\x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\
    \x01\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x05\x07\x1a\
    \n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\
    \x07\x01\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x01\x03\
    \x12\x03\x05\x1d!\n\n\n\x02\x05\0\x12\x04\x08\0\r\x01\n\n\n\x03\x05\0\
    \x01\x12\x03\x08\x05\x13\n\x0b\n\x04\x05\0\x02\0\x12\x03\t\x04\x10\n\x0c\
    \n\x05\x05\0\x02\0\x01\x12\x03\t\x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03\t\x0e\x0f\n\x0b\n\x04\x05\0\x02\x01\x12\x03\n\x04\x13\n\x0c\n\x05\
    \x05\0\x02\x01\x01\x12\x03\n\x04\x0e\n\x0c\n\x05\x05\0\x02\x01\x02\x12\
    \x03\n\x11\x12\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x0b\x04\x18\n\x0c\n\x05\
    \x05\0\x02\x02\x01\x12\x03\x0b\x04\x13\n\x0c\n\x05\x05\0\x02\x02\x02\x12\
    \x03\x0b\x16\x17\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x0c\x04\x15\n\x0c\n\
    \x05\x05\0\x02\x03\x01\x12\x03\x0c\x04\x10\n\x0c\n\x05\x05\0\x02\x03\x02\
    \x12\x03\x0c\x13\x14\n\n\n\x02\x05\x01\x12\x04\x0f\0\x14\x01\n\n\n\x03\
    \x05\x01\x01\x12\x03\x0f\x05\x11\n\x0b\n\x04\x05\x01\x02\0\x12\x03\x10\
    \x04\x16\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x10\x04\x11\n\x0c\n\x05\
    \x05\x01\x02\0\x02\x12\x03\x10\x14\x15\n\x20\n\x04\x05\x01\x02\x01\x12\
    \x03\x13\x04\x0c\x1a\x13\x20Logical\x20operators\n\n\x0c\n\x05\x05\x01\
    \x02\x01\x01\x12\x03\x13\x04\x07\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03\
    \x13\n\x0b\n\n\n\x02\x05\x02\x12\x04\x16\0\x18\x01\n\n\n\x03\x05\x02\x01\
    \x12\x03\x16\x05\x0e\n\x0b\n\x04\x05\x02\x02\0\x12\x03\x17\x04\x0c\n\x0c\
    \n\x05\x05\x02\x02\0\x01\x12\x03\x17\x04\x07\n\x0c\n\x05\x05\x02\x02\0\
    \x02\x12\x03\x17\n\x0b\n\n\n\x02\x04\0\x12\x04\x1a\0\x1d\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x1a\x08\x19\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1b\x04\
    \x18\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x1b\x04\x1a\x1b\n\x0c\n\x05\x04\0\
    \x02\0\x06\x12\x03\x1b\x04\x0e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1b\
    \x0f\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1b\x16\x17\n\x0b\n\x04\x04\
    \0\x02\x01\x12\x03\x1c\x04\x1a\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x1c\
    \x04\x1b\x18\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x1c\x04\x0f\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x1c\x10\x15\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x1c\x18\x19\n\n\n\x02\x04\x01\x12\x04\x1f\0!\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\x1f\x08\x11\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x20\x04\
    \x14\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x20\x04\x1f\x13\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\x20\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x20\x0b\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x20\x12\x13\n\n\n\x02\
    \x04\x02\x12\x04#\0&\x01\n\n\n\x03\x04\x02\x01\x12\x03#\x08\x16\n\x0b\n\
    \x04\x04\x02\x02\0\x12\x03$\x04\x1d\n\r\n\x05\x04\x02\x02\0\x04\x12\x04$\
    \x04#\x18\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03$\x04\x10\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03$\x11\x18\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03$\
    \x1b\x1c\n\x0b\n\x04\x04\x02\x02\x01\x12\x03%\x04%\n\x0c\n\x05\x04\x02\
    \x02\x01\x04\x12\x03%\x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03%\r\
    \x17\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03%\x18\x20\n\x0c\n\x05\x04\
    \x02\x02\x01\x03\x12\x03%#$\n\n\n\x02\x04\x03\x12\x04(\0+\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03(\x08\x13\n\x0b\n\x04\x04\x03\x02\0\x12\x03)\x04\x1a\
    \n\r\n\x05\x04\x03\x02\0\x04\x12\x04)\x04(\x15\n\x0c\n\x05\x04\x03\x02\0\
    \x06\x12\x03)\x04\r\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03)\x0e\x15\n\x0c\
    \n\x05\x04\x03\x02\0\x03\x12\x03)\x18\x19\n\x0b\n\x04\x04\x03\x02\x01\
    \x12\x03*\x04%\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03*\x04\x0c\n\x0c\n\
    \x05\x04\x03\x02\x01\x06\x12\x03*\r\x17\n\x0c\n\x05\x04\x03\x02\x01\x01\
    \x12\x03*\x18\x20\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03*#$\n\n\n\x02\
    \x04\x04\x12\x04-\04\x01\n\n\n\x03\x04\x04\x01\x12\x03-\x08\x12\n\x0b\n\
    \x04\x04\x04\x02\0\x12\x03.\x04'\n\r\n\x05\x04\x04\x02\0\x04\x12\x04.\
    \x04-\x14\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03.\x04\x12\n\x0c\n\x05\x04\
    \x04\x02\0\x01\x12\x03.\x13\"\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03.%&\n\
    \x0b\n\x04\x04\x04\x02\x01\x12\x030\x04\"\n\r\n\x05\x04\x04\x02\x01\x04\
    \x12\x040\x04.'\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x030\x04\x15\n\x0c\n\
    \x05\x04\x04\x02\x01\x01\x12\x030\x16\x1d\n\x0c\n\x05\x04\x04\x02\x01\
    \x03\x12\x030\x20!\n\x0b\n\x04\x04\x04\x02\x02\x12\x031\x04\x19\n\r\n\
    \x05\x04\x04\x02\x02\x04\x12\x041\x040\"\n\x0c\n\x05\x04\x04\x02\x02\x06\
    \x12\x031\x04\r\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x031\x0e\x14\n\x0c\n\
    \x05\x04\x04\x02\x02\x03\x12\x031\x17\x18\n\x0b\n\x04\x04\x04\x02\x03\
    \x12\x032\x04#\n\r\n\x05\x04\x04\x02\x03\x04\x12\x042\x041\x19\n\x0c\n\
    \x05\x04\x04\x02\x03\x06\x12\x032\x04\x12\n\x0c\n\x05\x04\x04\x02\x03\
    \x01\x12\x032\x13\x1e\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x032!\"\n\x0b\
    \n\x04\x04\x04\x02\x04\x12\x033\x04\x1d\n\r\n\x05\x04\x04\x02\x04\x04\
    \x12\x043\x042#\n\x0c\n\x05\x04\x04\x02\x04\x06\x12\x033\x04\x0f\n\x0c\n\
    \x05\x04\x04\x02\x04\x01\x12\x033\x10\x18\n\x0c\n\x05\x04\x04\x02\x04\
    \x03\x12\x033\x1b\x1cb\x06proto3\
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