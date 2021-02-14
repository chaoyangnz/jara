use std::rc::Rc;
use crate::constants::{FieldAccessFlag, JVM_SIGNATURE_CLASS, JVM_SIGNATURE_ENDCLASS, MethodAccessFlag};
use crate::types::Type;
use crate::value::Value;

pub struct Class {
    name: String,
    access_flags: u16,
    super_class_name: String,
    interface_names: Vec<String>,
    super_class: Rc<Class>,
    interfaces: Vec<Rc<Class>>,
    constant_pool: Vec<Box<Constant>>,
    fields:       Vec<Field>,
    methods:      Vec<Method>,

    instance_vars_count: i32,
    instance_var_fields: Vec<Rc<Field>>,
    static_vars_count:   i32,
    static_var_fields:   Vec<Rc<Field>>,

    static_vars:        Vec<Box<Value>>,

    source_file: String,

    // ---- these fields are only for array class -------
    component_type: Rc<Type>, // any type
    element_type:   Rc<Type>, // must be not array type
    dimensions:    i32,

    defined: bool,
    linked:  bool,

    initialized: i32,

    // TODO

}

impl Type for Class {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn descriptor(&self) -> String {
        [JVM_SIGNATURE_CLASS, self.name.as_str(), JVM_SIGNATURE_ENDCLASS].concat()
    }
}

pub struct Field {
    access_flags: FieldAccessFlag,
    name: String,
    descriptor: String,
    class: Rc<Class>,
    /**
    index of instanceFields or staticFields
    for instance fields, it is the global index considering superclass hierarchy
    */
    slot: i32
}

pub struct Method {
    access_flags: MethodAccessFlag,
    name:        String,
    descriptor:  String,
    class:       Rc<Class>,

    max_stack:    u32,
    max_locals:   u32,
    code:        Vec<u8>,             //u4 code_length
    exceptions:  Vec<ExceptionHandler>, //u2 exception_table_length
    local_vars:   Vec<LocalVariable>,
    line_numbers: Vec<LineNumber>,

    parameter_descriptors: Vec<String>,
    return_descriptor:     Vec<String>
}

pub trait Constant {
    fn resolve(&self);
}

pub struct ExceptionHandler {
    start_pc:   i32,
    end_pc:     i32,
    handler_pc: i32,
    catch_type: i32 // index of constant pool: ClassRef
}

pub struct LocalVariable {
    method:     Rc<Method>,
    start_pc:    u16,
    length:     u16,
    index:      u16,
    name:       String,
    descriptor: String
}

pub struct LineNumber {
    start_pc:    i32,
    line_number: i32
}