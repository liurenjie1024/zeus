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

    // .ColumnValue value = 1;

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
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.value.as_ref() {
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
        self.func_id = ScalarFuncId::AND;
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
        if self.func_id != ScalarFuncId::AND {
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
        if self.func_id != ScalarFuncId::AND {
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
        self.func_id = AggFuncId::COUNT;
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
        if self.func_id != AggFuncId::COUNT {
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
        if self.func_id != AggFuncId::COUNT {
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
    pub alias: ::std::string::String,
    pub field_type: super::zeus_meta::ColumnType,
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

    // string alias = 6;

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

    // .ColumnType field_type = 7;

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
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alias)?;
                },
                7 => {
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
        if !self.alias.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.alias);
        }
        if self.field_type != super::zeus_meta::ColumnType::BOOL {
            my_size += ::protobuf::rt::enum_size(7, self.field_type);
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
        if !self.alias.is_empty() {
            os.write_string(6, &self.alias)?;
        }
        if self.field_type != super::zeus_meta::ColumnType::BOOL {
            os.write_enum(7, self.field_type.value())?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    Expression::get_alias_for_reflect,
                    Expression::mut_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::zeus_meta::ColumnType>>(
                    "field_type",
                    Expression::get_field_type_for_reflect,
                    Expression::mut_field_type_for_reflect,
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
        self.clear_alias();
        self.clear_field_type();
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
    AND = 0,
    OR = 1,
    NOT = 2,
    LIKE = 3,
    GT_INT32 = 24,
    GT_INT64 = 25,
    GT_FLOAT4 = 26,
    GT_FLOAT8 = 27,
    GT_STR = 28,
    GE_INT32 = 32,
    GE_INT64 = 33,
    GE_FLOAT4 = 34,
    GE_FLOAT8 = 35,
    GE_STR = 36,
    LT_INT32 = 40,
    LT_INT64 = 41,
    LT_FLOAT4 = 42,
    LT_FLOAT8 = 43,
    LT_STR = 44,
    LE_INT32 = 48,
    LE_INT64 = 49,
    LE_FLOAT4 = 50,
    LE_FLOAT8 = 51,
    LE_STR = 52,
    EQ_INT32 = 56,
    EQ_INT64 = 57,
    EQ_FLOAT4 = 58,
    EQ_FLOAT8 = 59,
    EQ_STR = 60,
    NE_INT32 = 64,
    NE_INT64 = 65,
    NE_FLOAT4 = 66,
    NE_FLOAT8 = 67,
    NE_STR = 68,
    ADD_INT32 = 79,
    ADD_INT64 = 80,
    ADD_FLOAT4 = 81,
    ADD_FLOAT8 = 82,
    MINUS_INT32 = 83,
    MINUS_INT64 = 84,
    MINUS_FLOAT4 = 85,
    MINUS_FLOAT8 = 86,
    MUL_INT32 = 87,
    MUL_INT64 = 88,
    MUL_FLOAT4 = 89,
    MUL_FLOAT8 = 90,
    DIV_INT32 = 91,
    DIV_INT64 = 92,
    DIV_FLOAT4 = 93,
    DIV_FLOAT8 = 94,
}

impl ::protobuf::ProtobufEnum for ScalarFuncId {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ScalarFuncId> {
        match value {
            0 => ::std::option::Option::Some(ScalarFuncId::AND),
            1 => ::std::option::Option::Some(ScalarFuncId::OR),
            2 => ::std::option::Option::Some(ScalarFuncId::NOT),
            3 => ::std::option::Option::Some(ScalarFuncId::LIKE),
            24 => ::std::option::Option::Some(ScalarFuncId::GT_INT32),
            25 => ::std::option::Option::Some(ScalarFuncId::GT_INT64),
            26 => ::std::option::Option::Some(ScalarFuncId::GT_FLOAT4),
            27 => ::std::option::Option::Some(ScalarFuncId::GT_FLOAT8),
            28 => ::std::option::Option::Some(ScalarFuncId::GT_STR),
            32 => ::std::option::Option::Some(ScalarFuncId::GE_INT32),
            33 => ::std::option::Option::Some(ScalarFuncId::GE_INT64),
            34 => ::std::option::Option::Some(ScalarFuncId::GE_FLOAT4),
            35 => ::std::option::Option::Some(ScalarFuncId::GE_FLOAT8),
            36 => ::std::option::Option::Some(ScalarFuncId::GE_STR),
            40 => ::std::option::Option::Some(ScalarFuncId::LT_INT32),
            41 => ::std::option::Option::Some(ScalarFuncId::LT_INT64),
            42 => ::std::option::Option::Some(ScalarFuncId::LT_FLOAT4),
            43 => ::std::option::Option::Some(ScalarFuncId::LT_FLOAT8),
            44 => ::std::option::Option::Some(ScalarFuncId::LT_STR),
            48 => ::std::option::Option::Some(ScalarFuncId::LE_INT32),
            49 => ::std::option::Option::Some(ScalarFuncId::LE_INT64),
            50 => ::std::option::Option::Some(ScalarFuncId::LE_FLOAT4),
            51 => ::std::option::Option::Some(ScalarFuncId::LE_FLOAT8),
            52 => ::std::option::Option::Some(ScalarFuncId::LE_STR),
            56 => ::std::option::Option::Some(ScalarFuncId::EQ_INT32),
            57 => ::std::option::Option::Some(ScalarFuncId::EQ_INT64),
            58 => ::std::option::Option::Some(ScalarFuncId::EQ_FLOAT4),
            59 => ::std::option::Option::Some(ScalarFuncId::EQ_FLOAT8),
            60 => ::std::option::Option::Some(ScalarFuncId::EQ_STR),
            64 => ::std::option::Option::Some(ScalarFuncId::NE_INT32),
            65 => ::std::option::Option::Some(ScalarFuncId::NE_INT64),
            66 => ::std::option::Option::Some(ScalarFuncId::NE_FLOAT4),
            67 => ::std::option::Option::Some(ScalarFuncId::NE_FLOAT8),
            68 => ::std::option::Option::Some(ScalarFuncId::NE_STR),
            79 => ::std::option::Option::Some(ScalarFuncId::ADD_INT32),
            80 => ::std::option::Option::Some(ScalarFuncId::ADD_INT64),
            81 => ::std::option::Option::Some(ScalarFuncId::ADD_FLOAT4),
            82 => ::std::option::Option::Some(ScalarFuncId::ADD_FLOAT8),
            83 => ::std::option::Option::Some(ScalarFuncId::MINUS_INT32),
            84 => ::std::option::Option::Some(ScalarFuncId::MINUS_INT64),
            85 => ::std::option::Option::Some(ScalarFuncId::MINUS_FLOAT4),
            86 => ::std::option::Option::Some(ScalarFuncId::MINUS_FLOAT8),
            87 => ::std::option::Option::Some(ScalarFuncId::MUL_INT32),
            88 => ::std::option::Option::Some(ScalarFuncId::MUL_INT64),
            89 => ::std::option::Option::Some(ScalarFuncId::MUL_FLOAT4),
            90 => ::std::option::Option::Some(ScalarFuncId::MUL_FLOAT8),
            91 => ::std::option::Option::Some(ScalarFuncId::DIV_INT32),
            92 => ::std::option::Option::Some(ScalarFuncId::DIV_INT64),
            93 => ::std::option::Option::Some(ScalarFuncId::DIV_FLOAT4),
            94 => ::std::option::Option::Some(ScalarFuncId::DIV_FLOAT8),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ScalarFuncId] = &[
            ScalarFuncId::AND,
            ScalarFuncId::OR,
            ScalarFuncId::NOT,
            ScalarFuncId::LIKE,
            ScalarFuncId::GT_INT32,
            ScalarFuncId::GT_INT64,
            ScalarFuncId::GT_FLOAT4,
            ScalarFuncId::GT_FLOAT8,
            ScalarFuncId::GT_STR,
            ScalarFuncId::GE_INT32,
            ScalarFuncId::GE_INT64,
            ScalarFuncId::GE_FLOAT4,
            ScalarFuncId::GE_FLOAT8,
            ScalarFuncId::GE_STR,
            ScalarFuncId::LT_INT32,
            ScalarFuncId::LT_INT64,
            ScalarFuncId::LT_FLOAT4,
            ScalarFuncId::LT_FLOAT8,
            ScalarFuncId::LT_STR,
            ScalarFuncId::LE_INT32,
            ScalarFuncId::LE_INT64,
            ScalarFuncId::LE_FLOAT4,
            ScalarFuncId::LE_FLOAT8,
            ScalarFuncId::LE_STR,
            ScalarFuncId::EQ_INT32,
            ScalarFuncId::EQ_INT64,
            ScalarFuncId::EQ_FLOAT4,
            ScalarFuncId::EQ_FLOAT8,
            ScalarFuncId::EQ_STR,
            ScalarFuncId::NE_INT32,
            ScalarFuncId::NE_INT64,
            ScalarFuncId::NE_FLOAT4,
            ScalarFuncId::NE_FLOAT8,
            ScalarFuncId::NE_STR,
            ScalarFuncId::ADD_INT32,
            ScalarFuncId::ADD_INT64,
            ScalarFuncId::ADD_FLOAT4,
            ScalarFuncId::ADD_FLOAT8,
            ScalarFuncId::MINUS_INT32,
            ScalarFuncId::MINUS_INT64,
            ScalarFuncId::MINUS_FLOAT4,
            ScalarFuncId::MINUS_FLOAT8,
            ScalarFuncId::MUL_INT32,
            ScalarFuncId::MUL_INT64,
            ScalarFuncId::MUL_FLOAT4,
            ScalarFuncId::MUL_FLOAT8,
            ScalarFuncId::DIV_INT32,
            ScalarFuncId::DIV_INT64,
            ScalarFuncId::DIV_FLOAT4,
            ScalarFuncId::DIV_FLOAT8,
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
        ScalarFuncId::AND
    }
}

impl ::protobuf::reflect::ProtobufValue for ScalarFuncId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AggFuncId {
    COUNT = 0,
    MAX_INT32 = 1,
    MAX_INT64 = 2,
    MAX_FLOAT4 = 3,
    MAX_FLOAT8 = 4,
    MAX_STR = 5,
    MIN_INT32 = 6,
    MIN_INT64 = 7,
    MIN_FLOAT4 = 8,
    MIN_FLOAT8 = 9,
    MIN_STR = 10,
    SUM_INT32 = 11,
    SUM_INT64 = 12,
    SUM_FLOAT4 = 13,
    SUM_FLOAT8 = 14,
}

impl ::protobuf::ProtobufEnum for AggFuncId {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AggFuncId> {
        match value {
            0 => ::std::option::Option::Some(AggFuncId::COUNT),
            1 => ::std::option::Option::Some(AggFuncId::MAX_INT32),
            2 => ::std::option::Option::Some(AggFuncId::MAX_INT64),
            3 => ::std::option::Option::Some(AggFuncId::MAX_FLOAT4),
            4 => ::std::option::Option::Some(AggFuncId::MAX_FLOAT8),
            5 => ::std::option::Option::Some(AggFuncId::MAX_STR),
            6 => ::std::option::Option::Some(AggFuncId::MIN_INT32),
            7 => ::std::option::Option::Some(AggFuncId::MIN_INT64),
            8 => ::std::option::Option::Some(AggFuncId::MIN_FLOAT4),
            9 => ::std::option::Option::Some(AggFuncId::MIN_FLOAT8),
            10 => ::std::option::Option::Some(AggFuncId::MIN_STR),
            11 => ::std::option::Option::Some(AggFuncId::SUM_INT32),
            12 => ::std::option::Option::Some(AggFuncId::SUM_INT64),
            13 => ::std::option::Option::Some(AggFuncId::SUM_FLOAT4),
            14 => ::std::option::Option::Some(AggFuncId::SUM_FLOAT8),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AggFuncId] = &[
            AggFuncId::COUNT,
            AggFuncId::MAX_INT32,
            AggFuncId::MAX_INT64,
            AggFuncId::MAX_FLOAT4,
            AggFuncId::MAX_FLOAT8,
            AggFuncId::MAX_STR,
            AggFuncId::MIN_INT32,
            AggFuncId::MIN_INT64,
            AggFuncId::MIN_FLOAT4,
            AggFuncId::MIN_FLOAT8,
            AggFuncId::MIN_STR,
            AggFuncId::SUM_INT32,
            AggFuncId::SUM_INT64,
            AggFuncId::SUM_FLOAT4,
            AggFuncId::SUM_FLOAT8,
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
        AggFuncId::COUNT
    }
}

impl ::protobuf::reflect::ProtobufValue for AggFuncId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fzeus_expr.proto\x1a\x0fzeus_meta.proto\"7\n\x11LiteralExpression\
    \x12\"\n\x05value\x18\x01\x20\x01(\x0b2\x0c.ColumnValueR\x05value\"\x1f\
    \n\tColumnRef\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"a\n\x0eScal\
    arFunction\x12&\n\x07func_id\x18\x01\x20\x01(\x0e2\r.ScalarFuncIdR\x06fu\
    ncId\x12'\n\x08children\x18\x02\x20\x03(\x0b2\x0b.ExpressionR\x08childre\
    n\"[\n\x0bAggFunction\x12#\n\x07func_id\x18\x01\x20\x01(\x0e2\n.AggFuncI\
    dR\x06funcId\x12'\n\x08children\x18\x02\x20\x03(\x0b2\x0b.ExpressionR\
    \x08children\"\xb5\x02\n\nExpression\x128\n\x0fexpression_type\x18\x01\
    \x20\x01(\x0e2\x0f.ExpressionTypeR\x0eexpressionType\x12,\n\x07literal\
    \x18\x02\x20\x01(\x0b2\x12.LiteralExpressionR\x07literal\x12\"\n\x06colu\
    mn\x18\x03\x20\x01(\x0b2\n.ColumnRefR\x06column\x120\n\x0bscalar_func\
    \x18\x04\x20\x01(\x0b2\x0f.ScalarFunctionR\nscalarFunc\x12'\n\x08agg_fun\
    c\x18\x05\x20\x01(\x0b2\x0c.AggFunctionR\x07aggFunc\x12\x14\n\x05alias\
    \x18\x06\x20\x01(\tR\x05alias\x12*\n\nfield_type\x18\x07\x20\x01(\x0e2\
    \x0b.ColumnTypeR\tfieldType*T\n\x0eExpressionType\x12\x0b\n\x07LITERAL\
    \x10\0\x12\x0e\n\nCOLUMN_REF\x10\x01\x12\x13\n\x0fSCALAR_FUNCTION\x10\
    \x02\x12\x10\n\x0cAGG_FUNCTION\x10\x03*\xd6\x05\n\x0cScalarFuncId\x12\
    \x07\n\x03AND\x10\0\x12\x06\n\x02OR\x10\x01\x12\x07\n\x03NOT\x10\x02\x12\
    \x08\n\x04LIKE\x10\x03\x12\x0c\n\x08GT_INT32\x10\x18\x12\x0c\n\x08GT_INT\
    64\x10\x19\x12\r\n\tGT_FLOAT4\x10\x1a\x12\r\n\tGT_FLOAT8\x10\x1b\x12\n\n\
    \x06GT_STR\x10\x1c\x12\x0c\n\x08GE_INT32\x10\x20\x12\x0c\n\x08GE_INT64\
    \x10!\x12\r\n\tGE_FLOAT4\x10\"\x12\r\n\tGE_FLOAT8\x10#\x12\n\n\x06GE_STR\
    \x10$\x12\x0c\n\x08LT_INT32\x10(\x12\x0c\n\x08LT_INT64\x10)\x12\r\n\tLT_\
    FLOAT4\x10*\x12\r\n\tLT_FLOAT8\x10+\x12\n\n\x06LT_STR\x10,\x12\x0c\n\x08\
    LE_INT32\x100\x12\x0c\n\x08LE_INT64\x101\x12\r\n\tLE_FLOAT4\x102\x12\r\n\
    \tLE_FLOAT8\x103\x12\n\n\x06LE_STR\x104\x12\x0c\n\x08EQ_INT32\x108\x12\
    \x0c\n\x08EQ_INT64\x109\x12\r\n\tEQ_FLOAT4\x10:\x12\r\n\tEQ_FLOAT8\x10;\
    \x12\n\n\x06EQ_STR\x10<\x12\x0c\n\x08NE_INT32\x10@\x12\x0c\n\x08NE_INT64\
    \x10A\x12\r\n\tNE_FLOAT4\x10B\x12\r\n\tNE_FLOAT8\x10C\x12\n\n\x06NE_STR\
    \x10D\x12\r\n\tADD_INT32\x10O\x12\r\n\tADD_INT64\x10P\x12\x0e\n\nADD_FLO\
    AT4\x10Q\x12\x0e\n\nADD_FLOAT8\x10R\x12\x0f\n\x0bMINUS_INT32\x10S\x12\
    \x0f\n\x0bMINUS_INT64\x10T\x12\x10\n\x0cMINUS_FLOAT4\x10U\x12\x10\n\x0cM\
    INUS_FLOAT8\x10V\x12\r\n\tMUL_INT32\x10W\x12\r\n\tMUL_INT64\x10X\x12\x0e\
    \n\nMUL_FLOAT4\x10Y\x12\x0e\n\nMUL_FLOAT8\x10Z\x12\r\n\tDIV_INT32\x10[\
    \x12\r\n\tDIV_INT64\x10\\\x12\x0e\n\nDIV_FLOAT4\x10]\x12\x0e\n\nDIV_FLOA\
    T8\x10^*\xea\x01\n\tAggFuncId\x12\t\n\x05COUNT\x10\0\x12\r\n\tMAX_INT32\
    \x10\x01\x12\r\n\tMAX_INT64\x10\x02\x12\x0e\n\nMAX_FLOAT4\x10\x03\x12\
    \x0e\n\nMAX_FLOAT8\x10\x04\x12\x0b\n\x07MAX_STR\x10\x05\x12\r\n\tMIN_INT\
    32\x10\x06\x12\r\n\tMIN_INT64\x10\x07\x12\x0e\n\nMIN_FLOAT4\x10\x08\x12\
    \x0e\n\nMIN_FLOAT8\x10\t\x12\x0b\n\x07MIN_STR\x10\n\x12\r\n\tSUM_INT32\
    \x10\x0b\x12\r\n\tSUM_INT64\x10\x0c\x12\x0e\n\nSUM_FLOAT4\x10\r\x12\x0e\
    \n\nSUM_FLOAT8\x10\x0eB\x16\n\x12io.github.zeus.rpcP\x01J\xe2\x20\n\x07\
    \x12\x05\0\0\x82\x01\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\
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
    \x12\x03\x0c\x13\x14\n\n\n\x02\x05\x01\x12\x04\x0f\0O\x01\n\n\n\x03\x05\
    \x01\x01\x12\x03\x0f\x05\x11\n\x20\n\x04\x05\x01\x02\0\x12\x03\x11\x04\
    \x0c\x1a\x13\x20Logical\x20operators\n\n\x0c\n\x05\x05\x01\x02\0\x01\x12\
    \x03\x11\x04\x07\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\x11\n\x0b\n\x0b\n\
    \x04\x05\x01\x02\x01\x12\x03\x12\x04\x0c\n\x0c\n\x05\x05\x01\x02\x01\x01\
    \x12\x03\x12\x04\x06\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03\x12\n\x0b\n\
    \x0b\n\x04\x05\x01\x02\x02\x12\x03\x13\x04\x0c\n\x0c\n\x05\x05\x01\x02\
    \x02\x01\x12\x03\x13\x04\x07\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03\x13\
    \n\x0b\n\x0b\n\x04\x05\x01\x02\x03\x12\x03\x14\x04\r\n\x0c\n\x05\x05\x01\
    \x02\x03\x01\x12\x03\x14\x04\x08\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03\
    \x14\x0b\x0c\n\x1f\n\x04\x05\x01\x02\x04\x12\x03\x17\x04\x12\x1a\x12\x20\
    Compare\x20operator\n\n\x0c\n\x05\x05\x01\x02\x04\x01\x12\x03\x17\x04\
    \x0c\n\x0c\n\x05\x05\x01\x02\x04\x02\x12\x03\x17\x0f\x11\n\x0b\n\x04\x05\
    \x01\x02\x05\x12\x03\x18\x04\x12\n\x0c\n\x05\x05\x01\x02\x05\x01\x12\x03\
    \x18\x04\x0c\n\x0c\n\x05\x05\x01\x02\x05\x02\x12\x03\x18\x0f\x11\n\x0b\n\
    \x04\x05\x01\x02\x06\x12\x03\x19\x04\x13\n\x0c\n\x05\x05\x01\x02\x06\x01\
    \x12\x03\x19\x04\r\n\x0c\n\x05\x05\x01\x02\x06\x02\x12\x03\x19\x10\x12\n\
    \x0b\n\x04\x05\x01\x02\x07\x12\x03\x1a\x04\x13\n\x0c\n\x05\x05\x01\x02\
    \x07\x01\x12\x03\x1a\x04\r\n\x0c\n\x05\x05\x01\x02\x07\x02\x12\x03\x1a\
    \x10\x12\n\x0b\n\x04\x05\x01\x02\x08\x12\x03\x1b\x04\x10\n\x0c\n\x05\x05\
    \x01\x02\x08\x01\x12\x03\x1b\x04\n\n\x0c\n\x05\x05\x01\x02\x08\x02\x12\
    \x03\x1b\r\x0f\n\x0b\n\x04\x05\x01\x02\t\x12\x03\x1d\x04\x12\n\x0c\n\x05\
    \x05\x01\x02\t\x01\x12\x03\x1d\x04\x0c\n\x0c\n\x05\x05\x01\x02\t\x02\x12\
    \x03\x1d\x0f\x11\n\x0b\n\x04\x05\x01\x02\n\x12\x03\x1e\x04\x12\n\x0c\n\
    \x05\x05\x01\x02\n\x01\x12\x03\x1e\x04\x0c\n\x0c\n\x05\x05\x01\x02\n\x02\
    \x12\x03\x1e\x0f\x11\n\x0b\n\x04\x05\x01\x02\x0b\x12\x03\x1f\x04\x13\n\
    \x0c\n\x05\x05\x01\x02\x0b\x01\x12\x03\x1f\x04\r\n\x0c\n\x05\x05\x01\x02\
    \x0b\x02\x12\x03\x1f\x10\x12\n\x0b\n\x04\x05\x01\x02\x0c\x12\x03\x20\x04\
    \x13\n\x0c\n\x05\x05\x01\x02\x0c\x01\x12\x03\x20\x04\r\n\x0c\n\x05\x05\
    \x01\x02\x0c\x02\x12\x03\x20\x10\x12\n\x0b\n\x04\x05\x01\x02\r\x12\x03!\
    \x04\x10\n\x0c\n\x05\x05\x01\x02\r\x01\x12\x03!\x04\n\n\x0c\n\x05\x05\
    \x01\x02\r\x02\x12\x03!\r\x0f\n\x0b\n\x04\x05\x01\x02\x0e\x12\x03#\x04\
    \x12\n\x0c\n\x05\x05\x01\x02\x0e\x01\x12\x03#\x04\x0c\n\x0c\n\x05\x05\
    \x01\x02\x0e\x02\x12\x03#\x0f\x11\n\x0b\n\x04\x05\x01\x02\x0f\x12\x03$\
    \x04\x12\n\x0c\n\x05\x05\x01\x02\x0f\x01\x12\x03$\x04\x0c\n\x0c\n\x05\
    \x05\x01\x02\x0f\x02\x12\x03$\x0f\x11\n\x0b\n\x04\x05\x01\x02\x10\x12\
    \x03%\x04\x13\n\x0c\n\x05\x05\x01\x02\x10\x01\x12\x03%\x04\r\n\x0c\n\x05\
    \x05\x01\x02\x10\x02\x12\x03%\x10\x12\n\x0b\n\x04\x05\x01\x02\x11\x12\
    \x03&\x04\x13\n\x0c\n\x05\x05\x01\x02\x11\x01\x12\x03&\x04\r\n\x0c\n\x05\
    \x05\x01\x02\x11\x02\x12\x03&\x10\x12\n\x0b\n\x04\x05\x01\x02\x12\x12\
    \x03'\x04\x10\n\x0c\n\x05\x05\x01\x02\x12\x01\x12\x03'\x04\n\n\x0c\n\x05\
    \x05\x01\x02\x12\x02\x12\x03'\r\x0f\n\x0b\n\x04\x05\x01\x02\x13\x12\x03)\
    \x04\x12\n\x0c\n\x05\x05\x01\x02\x13\x01\x12\x03)\x04\x0c\n\x0c\n\x05\
    \x05\x01\x02\x13\x02\x12\x03)\x0f\x11\n\x0b\n\x04\x05\x01\x02\x14\x12\
    \x03*\x04\x12\n\x0c\n\x05\x05\x01\x02\x14\x01\x12\x03*\x04\x0c\n\x0c\n\
    \x05\x05\x01\x02\x14\x02\x12\x03*\x0f\x11\n\x0b\n\x04\x05\x01\x02\x15\
    \x12\x03+\x04\x13\n\x0c\n\x05\x05\x01\x02\x15\x01\x12\x03+\x04\r\n\x0c\n\
    \x05\x05\x01\x02\x15\x02\x12\x03+\x10\x12\n\x0b\n\x04\x05\x01\x02\x16\
    \x12\x03,\x04\x13\n\x0c\n\x05\x05\x01\x02\x16\x01\x12\x03,\x04\r\n\x0c\n\
    \x05\x05\x01\x02\x16\x02\x12\x03,\x10\x12\n\x0b\n\x04\x05\x01\x02\x17\
    \x12\x03-\x04\x10\n\x0c\n\x05\x05\x01\x02\x17\x01\x12\x03-\x04\n\n\x0c\n\
    \x05\x05\x01\x02\x17\x02\x12\x03-\r\x0f\n\x0b\n\x04\x05\x01\x02\x18\x12\
    \x03/\x04\x12\n\x0c\n\x05\x05\x01\x02\x18\x01\x12\x03/\x04\x0c\n\x0c\n\
    \x05\x05\x01\x02\x18\x02\x12\x03/\x0f\x11\n\x0b\n\x04\x05\x01\x02\x19\
    \x12\x030\x04\x12\n\x0c\n\x05\x05\x01\x02\x19\x01\x12\x030\x04\x0c\n\x0c\
    \n\x05\x05\x01\x02\x19\x02\x12\x030\x0f\x11\n\x0b\n\x04\x05\x01\x02\x1a\
    \x12\x031\x04\x13\n\x0c\n\x05\x05\x01\x02\x1a\x01\x12\x031\x04\r\n\x0c\n\
    \x05\x05\x01\x02\x1a\x02\x12\x031\x10\x12\n\x0b\n\x04\x05\x01\x02\x1b\
    \x12\x032\x04\x13\n\x0c\n\x05\x05\x01\x02\x1b\x01\x12\x032\x04\r\n\x0c\n\
    \x05\x05\x01\x02\x1b\x02\x12\x032\x10\x12\n\x0b\n\x04\x05\x01\x02\x1c\
    \x12\x033\x04\x10\n\x0c\n\x05\x05\x01\x02\x1c\x01\x12\x033\x04\n\n\x0c\n\
    \x05\x05\x01\x02\x1c\x02\x12\x033\r\x0f\n\x0b\n\x04\x05\x01\x02\x1d\x12\
    \x035\x04\x12\n\x0c\n\x05\x05\x01\x02\x1d\x01\x12\x035\x04\x0c\n\x0c\n\
    \x05\x05\x01\x02\x1d\x02\x12\x035\x0f\x11\n\x0b\n\x04\x05\x01\x02\x1e\
    \x12\x036\x04\x12\n\x0c\n\x05\x05\x01\x02\x1e\x01\x12\x036\x04\x0c\n\x0c\
    \n\x05\x05\x01\x02\x1e\x02\x12\x036\x0f\x11\n\x0b\n\x04\x05\x01\x02\x1f\
    \x12\x037\x04\x13\n\x0c\n\x05\x05\x01\x02\x1f\x01\x12\x037\x04\r\n\x0c\n\
    \x05\x05\x01\x02\x1f\x02\x12\x037\x10\x12\n\x0b\n\x04\x05\x01\x02\x20\
    \x12\x038\x04\x13\n\x0c\n\x05\x05\x01\x02\x20\x01\x12\x038\x04\r\n\x0c\n\
    \x05\x05\x01\x02\x20\x02\x12\x038\x10\x12\n\x0b\n\x04\x05\x01\x02!\x12\
    \x039\x04\x10\n\x0c\n\x05\x05\x01\x02!\x01\x12\x039\x04\n\n\x0c\n\x05\
    \x05\x01\x02!\x02\x12\x039\r\x0f\n\x1d\n\x04\x05\x01\x02\"\x12\x03<\x04\
    \x11\x1a\x10\x20math\x20operators\n\n\x0c\n\x05\x05\x01\x02\"\x01\x12\
    \x03<\x04\r\n\x0c\n\x05\x05\x01\x02\"\x02\x12\x03<\x0e\x10\n\x0b\n\x04\
    \x05\x01\x02#\x12\x03=\x04\x11\n\x0c\n\x05\x05\x01\x02#\x01\x12\x03=\x04\
    \r\n\x0c\n\x05\x05\x01\x02#\x02\x12\x03=\x0e\x10\n\x0b\n\x04\x05\x01\x02\
    $\x12\x03>\x04\x12\n\x0c\n\x05\x05\x01\x02$\x01\x12\x03>\x04\x0e\n\x0c\n\
    \x05\x05\x01\x02$\x02\x12\x03>\x0f\x11\n\x0b\n\x04\x05\x01\x02%\x12\x03?\
    \x04\x12\n\x0c\n\x05\x05\x01\x02%\x01\x12\x03?\x04\x0e\n\x0c\n\x05\x05\
    \x01\x02%\x02\x12\x03?\x0f\x11\n\x0b\n\x04\x05\x01\x02&\x12\x03A\x04\x13\
    \n\x0c\n\x05\x05\x01\x02&\x01\x12\x03A\x04\x0f\n\x0c\n\x05\x05\x01\x02&\
    \x02\x12\x03A\x10\x12\n\x0b\n\x04\x05\x01\x02'\x12\x03B\x04\x13\n\x0c\n\
    \x05\x05\x01\x02'\x01\x12\x03B\x04\x0f\n\x0c\n\x05\x05\x01\x02'\x02\x12\
    \x03B\x10\x12\n\x0b\n\x04\x05\x01\x02(\x12\x03C\x04\x14\n\x0c\n\x05\x05\
    \x01\x02(\x01\x12\x03C\x04\x10\n\x0c\n\x05\x05\x01\x02(\x02\x12\x03C\x11\
    \x13\n\x0b\n\x04\x05\x01\x02)\x12\x03D\x04\x14\n\x0c\n\x05\x05\x01\x02)\
    \x01\x12\x03D\x04\x10\n\x0c\n\x05\x05\x01\x02)\x02\x12\x03D\x11\x13\n\
    \x0b\n\x04\x05\x01\x02*\x12\x03F\x04\x11\n\x0c\n\x05\x05\x01\x02*\x01\
    \x12\x03F\x04\r\n\x0c\n\x05\x05\x01\x02*\x02\x12\x03F\x0e\x10\n\x0b\n\
    \x04\x05\x01\x02+\x12\x03G\x04\x11\n\x0c\n\x05\x05\x01\x02+\x01\x12\x03G\
    \x04\r\n\x0c\n\x05\x05\x01\x02+\x02\x12\x03G\x0e\x10\n\x0b\n\x04\x05\x01\
    \x02,\x12\x03H\x04\x12\n\x0c\n\x05\x05\x01\x02,\x01\x12\x03H\x04\x0e\n\
    \x0c\n\x05\x05\x01\x02,\x02\x12\x03H\x0f\x11\n\x0b\n\x04\x05\x01\x02-\
    \x12\x03I\x04\x12\n\x0c\n\x05\x05\x01\x02-\x01\x12\x03I\x04\x0e\n\x0c\n\
    \x05\x05\x01\x02-\x02\x12\x03I\x0f\x11\n\x0b\n\x04\x05\x01\x02.\x12\x03K\
    \x04\x11\n\x0c\n\x05\x05\x01\x02.\x01\x12\x03K\x04\r\n\x0c\n\x05\x05\x01\
    \x02.\x02\x12\x03K\x0e\x10\n\x0b\n\x04\x05\x01\x02/\x12\x03L\x04\x11\n\
    \x0c\n\x05\x05\x01\x02/\x01\x12\x03L\x04\r\n\x0c\n\x05\x05\x01\x02/\x02\
    \x12\x03L\x0e\x10\n\x0b\n\x04\x05\x01\x020\x12\x03M\x04\x12\n\x0c\n\x05\
    \x05\x01\x020\x01\x12\x03M\x04\x0e\n\x0c\n\x05\x05\x01\x020\x02\x12\x03M\
    \x0f\x11\n\x0b\n\x04\x05\x01\x021\x12\x03N\x04\x12\n\x0c\n\x05\x05\x01\
    \x021\x01\x12\x03N\x04\x0e\n\x0c\n\x05\x05\x01\x021\x02\x12\x03N\x0f\x11\
    \n\n\n\x02\x05\x02\x12\x04Q\0d\x01\n\n\n\x03\x05\x02\x01\x12\x03Q\x05\
    \x0e\n\x0b\n\x04\x05\x02\x02\0\x12\x03R\x04\x0e\n\x0c\n\x05\x05\x02\x02\
    \0\x01\x12\x03R\x04\t\n\x0c\n\x05\x05\x02\x02\0\x02\x12\x03R\x0c\r\n\x0b\
    \n\x04\x05\x02\x02\x01\x12\x03T\x04\x10\n\x0c\n\x05\x05\x02\x02\x01\x01\
    \x12\x03T\x04\r\n\x0c\n\x05\x05\x02\x02\x01\x02\x12\x03T\x0e\x0f\n\x0b\n\
    \x04\x05\x02\x02\x02\x12\x03U\x04\x10\n\x0c\n\x05\x05\x02\x02\x02\x01\
    \x12\x03U\x04\r\n\x0c\n\x05\x05\x02\x02\x02\x02\x12\x03U\x0e\x0f\n\x0b\n\
    \x04\x05\x02\x02\x03\x12\x03V\x04\x11\n\x0c\n\x05\x05\x02\x02\x03\x01\
    \x12\x03V\x04\x0e\n\x0c\n\x05\x05\x02\x02\x03\x02\x12\x03V\x0f\x10\n\x0b\
    \n\x04\x05\x02\x02\x04\x12\x03W\x04\x11\n\x0c\n\x05\x05\x02\x02\x04\x01\
    \x12\x03W\x04\x0e\n\x0c\n\x05\x05\x02\x02\x04\x02\x12\x03W\x0f\x10\n\x0b\
    \n\x04\x05\x02\x02\x05\x12\x03X\x04\x0e\n\x0c\n\x05\x05\x02\x02\x05\x01\
    \x12\x03X\x04\x0b\n\x0c\n\x05\x05\x02\x02\x05\x02\x12\x03X\x0c\r\n\x0b\n\
    \x04\x05\x02\x02\x06\x12\x03Z\x04\x10\n\x0c\n\x05\x05\x02\x02\x06\x01\
    \x12\x03Z\x04\r\n\x0c\n\x05\x05\x02\x02\x06\x02\x12\x03Z\x0e\x0f\n\x0b\n\
    \x04\x05\x02\x02\x07\x12\x03[\x04\x10\n\x0c\n\x05\x05\x02\x02\x07\x01\
    \x12\x03[\x04\r\n\x0c\n\x05\x05\x02\x02\x07\x02\x12\x03[\x0e\x0f\n\x0b\n\
    \x04\x05\x02\x02\x08\x12\x03\\\x04\x11\n\x0c\n\x05\x05\x02\x02\x08\x01\
    \x12\x03\\\x04\x0e\n\x0c\n\x05\x05\x02\x02\x08\x02\x12\x03\\\x0f\x10\n\
    \x0b\n\x04\x05\x02\x02\t\x12\x03]\x04\x11\n\x0c\n\x05\x05\x02\x02\t\x01\
    \x12\x03]\x04\x0e\n\x0c\n\x05\x05\x02\x02\t\x02\x12\x03]\x0f\x10\n\x0b\n\
    \x04\x05\x02\x02\n\x12\x03^\x04\x0f\n\x0c\n\x05\x05\x02\x02\n\x01\x12\
    \x03^\x04\x0b\n\x0c\n\x05\x05\x02\x02\n\x02\x12\x03^\x0c\x0e\n\x0b\n\x04\
    \x05\x02\x02\x0b\x12\x03`\x04\x11\n\x0c\n\x05\x05\x02\x02\x0b\x01\x12\
    \x03`\x04\r\n\x0c\n\x05\x05\x02\x02\x0b\x02\x12\x03`\x0e\x10\n\x0b\n\x04\
    \x05\x02\x02\x0c\x12\x03a\x04\x11\n\x0c\n\x05\x05\x02\x02\x0c\x01\x12\
    \x03a\x04\r\n\x0c\n\x05\x05\x02\x02\x0c\x02\x12\x03a\x0e\x10\n\x0b\n\x04\
    \x05\x02\x02\r\x12\x03b\x04\x12\n\x0c\n\x05\x05\x02\x02\r\x01\x12\x03b\
    \x04\x0e\n\x0c\n\x05\x05\x02\x02\r\x02\x12\x03b\x0f\x11\n\x0b\n\x04\x05\
    \x02\x02\x0e\x12\x03c\x04\x12\n\x0c\n\x05\x05\x02\x02\x0e\x01\x12\x03c\
    \x04\x0e\n\x0c\n\x05\x05\x02\x02\x0e\x02\x12\x03c\x0f\x11\n\n\n\x02\x04\
    \0\x12\x04f\0h\x01\n\n\n\x03\x04\0\x01\x12\x03f\x08\x19\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03g\x04\x1a\n\r\n\x05\x04\0\x02\0\x04\x12\x04g\x04f\x1b\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03g\x04\x0f\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03g\x10\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03g\x18\x19\n\n\n\x02\
    \x04\x01\x12\x04j\0l\x01\n\n\n\x03\x04\x01\x01\x12\x03j\x08\x11\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03k\x04\x14\n\r\n\x05\x04\x01\x02\0\x04\x12\x04k\
    \x04j\x13\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03k\x04\n\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03k\x0b\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03k\
    \x12\x13\n\n\n\x02\x04\x02\x12\x04n\0q\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    n\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03o\x04\x1d\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04o\x04n\x18\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03o\x04\
    \x10\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03o\x11\x18\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03o\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x01\x12\x03p\x04%\n\
    \x0c\n\x05\x04\x02\x02\x01\x04\x12\x03p\x04\x0c\n\x0c\n\x05\x04\x02\x02\
    \x01\x06\x12\x03p\r\x17\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03p\x18\x20\
    \n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03p#$\n\n\n\x02\x04\x03\x12\x04s\0\
    v\x01\n\n\n\x03\x04\x03\x01\x12\x03s\x08\x13\n\x0b\n\x04\x04\x03\x02\0\
    \x12\x03t\x04\x1a\n\r\n\x05\x04\x03\x02\0\x04\x12\x04t\x04s\x15\n\x0c\n\
    \x05\x04\x03\x02\0\x06\x12\x03t\x04\r\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x03t\x0e\x15\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03t\x18\x19\n\x0b\n\x04\
    \x04\x03\x02\x01\x12\x03u\x04%\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03u\
    \x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03u\r\x17\n\x0c\n\x05\x04\
    \x03\x02\x01\x01\x12\x03u\x18\x20\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\
    \x03u#$\n\x0b\n\x02\x04\x04\x12\x05x\0\x82\x01\x01\n\n\n\x03\x04\x04\x01\
    \x12\x03x\x08\x12\n\x0b\n\x04\x04\x04\x02\0\x12\x03y\x04'\n\r\n\x05\x04\
    \x04\x02\0\x04\x12\x04y\x04x\x14\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03y\
    \x04\x12\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03y\x13\"\n\x0c\n\x05\x04\
    \x04\x02\0\x03\x12\x03y%&\n\x0b\n\x04\x04\x04\x02\x01\x12\x03{\x04\"\n\r\
    \n\x05\x04\x04\x02\x01\x04\x12\x04{\x04y'\n\x0c\n\x05\x04\x04\x02\x01\
    \x06\x12\x03{\x04\x15\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03{\x16\x1d\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x03{\x20!\n\x0b\n\x04\x04\x04\x02\x02\
    \x12\x03|\x04\x19\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04|\x04{\"\n\x0c\n\
    \x05\x04\x04\x02\x02\x06\x12\x03|\x04\r\n\x0c\n\x05\x04\x04\x02\x02\x01\
    \x12\x03|\x0e\x14\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03|\x17\x18\n\x0b\
    \n\x04\x04\x04\x02\x03\x12\x03}\x04#\n\r\n\x05\x04\x04\x02\x03\x04\x12\
    \x04}\x04|\x19\n\x0c\n\x05\x04\x04\x02\x03\x06\x12\x03}\x04\x12\n\x0c\n\
    \x05\x04\x04\x02\x03\x01\x12\x03}\x13\x1e\n\x0c\n\x05\x04\x04\x02\x03\
    \x03\x12\x03}!\"\n\x0b\n\x04\x04\x04\x02\x04\x12\x03~\x04\x1d\n\r\n\x05\
    \x04\x04\x02\x04\x04\x12\x04~\x04}#\n\x0c\n\x05\x04\x04\x02\x04\x06\x12\
    \x03~\x04\x0f\n\x0c\n\x05\x04\x04\x02\x04\x01\x12\x03~\x10\x18\n\x0c\n\
    \x05\x04\x04\x02\x04\x03\x12\x03~\x1b\x1c\n\x0c\n\x04\x04\x04\x02\x05\
    \x12\x04\x80\x01\x04\x15\n\x0e\n\x05\x04\x04\x02\x05\x04\x12\x05\x80\x01\
    \x04~\x1d\n\r\n\x05\x04\x04\x02\x05\x05\x12\x04\x80\x01\x04\n\n\r\n\x05\
    \x04\x04\x02\x05\x01\x12\x04\x80\x01\x0b\x10\n\r\n\x05\x04\x04\x02\x05\
    \x03\x12\x04\x80\x01\x13\x14\n\x0c\n\x04\x04\x04\x02\x06\x12\x04\x81\x01\
    \x04\x1e\n\x0f\n\x05\x04\x04\x02\x06\x04\x12\x06\x81\x01\x04\x80\x01\x15\
    \n\r\n\x05\x04\x04\x02\x06\x06\x12\x04\x81\x01\x04\x0e\n\r\n\x05\x04\x04\
    \x02\x06\x01\x12\x04\x81\x01\x0f\x19\n\r\n\x05\x04\x04\x02\x06\x03\x12\
    \x04\x81\x01\x1c\x1db\x06proto3\
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
