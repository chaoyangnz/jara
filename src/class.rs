use std::rc::Rc;
use crate::constants::{FieldAccessFlag, JVM_SIGNATURE_CLASS, JVM_SIGNATURE_ENDCLASS, MethodAccessFlag};
use crate::types::Type;
use crate::value::Value;

pub struct Class {
    name: String,
    accessFlags: u16,
    superClassName: String,
    interfaceNames: Vec<String>,
    superClass: Rc<Class>,
    interfaces: Vec<Rc<Class>>,
    constantPool: Vec<Box<Constant>>,
    fields:       Vec<Field>,
    methods:      Vec<Method>,

    instanceVarsCount: i32,
    instanceVarFields: Vec<Rc<Field>>,
    staticVarsCount:   i32,
    staticVarFields:   Vec<Rc<Field>>,

    staticVars:        Vec<Box<Value>>,

    sourceFile: String,

    // ---- these fields are only for array class -------
    componentType: Rc<Type>, // any type
    elementType:   Rc<Type>, // must be not array type
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
    accessFlags: FieldAccessFlag,
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
    accessFlags: MethodAccessFlag,
    name:        String,
    descriptor:  String,
    class:       Rc<Class>,

    maxStack:    u32,
    maxLocals:   u32,
    code:        Vec<u8>,             //u4 code_length
    exceptions:  Vec<ExceptionHandler>, //u2 exception_table_length
    localVars:   Vec<LocalVariable>,
    lineNumbers: Vec<LineNumber>,

    parameterDescriptors: Vec<String>,
    returnDescriptor:     Vec<String>
}

pub trait Constant {
    fn resolve(&self);
}

pub struct ExceptionHandler {
    startPc:   i32,
    endPc:     i32,
    handlerPc: i32,
    catchType: i32 // index of constant pool: ClassRef
}

pub struct LocalVariable {
    method:     Rc<Method>,
    startPc:    u16,
    length:     u16,
    index:      u16,
    name:       String,
    descriptor: String
}

pub struct LineNumber {
    startPc:    i32,
    lineNumber: i32
}