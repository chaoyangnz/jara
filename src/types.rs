use std::rc::Rc;
use std::ptr::null;
use crate::constants::{FieldAccessFlag, JVM_SIGNATURE_CLASS, JVM_SIGNATURE_ENDCLASS, MethodAccessFlag};
use crate::values::Value;
use crate::class_file::*;
use crate::constants::*;
use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
pub(crate) enum Type {
    Byte,
    Short,
    Char,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Class(Class)
}

impl Type {
    fn name(&self) -> String {
        match self {
            Type::Byte => JVM_SIGNATURE_BYTE.to_string(),
            Type::Short => JVM_SIGNATURE_SHORT.to_string(),
            Type::Char => JVM_SIGNATURE_CHAR.to_string(),
            Type::Int => JVM_SIGNATURE_INT.to_string(),
            Type::Long => JVM_SIGNATURE_LONG.to_string(),
            Type::Float => JVM_SIGNATURE_FLOAT.to_string(),
            Type::Double => JVM_SIGNATURE_DOUBLE.to_string(),
            Type::Boolean => JVM_SIGNATURE_BOOLEAN.to_string(),
            Type::Class(class) => class.name.to_string()
        }
    }

    fn descriptor(&self) -> String {
        match self {
            Type::Byte => self.name(),
            Type::Short => self.name(),
            Type::Char => self.name(),
            Type::Int => self.name(),
            Type::Long => self.name(),
            Type::Float => self.name(),
            Type::Double => self.name(),
            Type::Boolean => self.name(),
            Type::Class(class) =>  [JVM_SIGNATURE_CLASS, class.name.as_str(), JVM_SIGNATURE_ENDCLASS].concat()
        }
    }
}

pub struct Class {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) name: String,
    pub(crate) access_flags: u16,
    pub(crate) super_class_name: String,
    pub(crate) interface_names: Vec<String>,

    pub(crate) fields:       Vec<Field>,
    pub(crate) methods:      Vec<Method>,

    pub(crate) instance_vars_count: i32,
    pub(crate) instance_var_fields: Vec<Rc<Field>>,
    pub(crate) static_vars_count:   i32,
    pub(crate) static_var_fields:   Vec<Rc<Field>>,

    pub(crate) static_vars:        Vec<Value>,

    pub(crate) source_file: String,

    // ---- these fields are only for array class -------
    pub(crate) component_type: Rc<Type>, // any type
    pub(crate) element_type:   Rc<Type>, // must be not array type
    pub(crate) dimensions:    i32,

    // status flags
    pub(crate) defined: bool, // once read from classfile
    pub(crate) linked:  bool, // once resolve Ref symbols
    pub(crate) initialized: i32, // once call <clinit>

    // after linked
    pub(crate) super_class: Rc<Class>, // to be resolved
    pub(crate) interfaces: Vec<Rc<Class>>, // to be resolved

    // TODO

}

fn uninitialized_class() -> Rc<Class> {
    unsafe { Rc::<Class>::from_raw(null()) }
}
fn uninitialized_method() -> Rc<Method> {
    unsafe { Rc::<Method>::from_raw(null()) }
}

impl Class {
    pub(crate) fn from(classfile: &ClassFile) -> Self {
        let cp =  &classfile.constant_pool;
        let constant_pool_len = classfile.constant_pool_count as usize;
        let constant_pool= (0..constant_pool_len).map(|i| Constant::from(i, cp)).collect();
        let fields = classfile.fields.iter().enumerate().map(|(i, field_info)|
            Field {
                access_flags: field_info.access_flags,
                name: cp.resolve_utf8(field_info.name_index),
                descriptor: cp.resolve_utf8(field_info.descriptor_index),
                class: uninitialized_class(),
                slot: i as u32
            }
        ).collect();

        let methods = classfile.methods.iter().enumerate().map(|(i, method_info)| {
            let code_attribute = method_info.find_code_attribute();
            let exceptions = code_attribute.exception_table.iter().map(|exception_entry|
                ExceptionHandler {
                    start_pc: exception_entry.start_pc,
                    end_pc: exception_entry.end_pc,
                    handler_pc: exception_entry.handle_pc,
                    catch_type: cp.resolve_class(exception_entry.catch_type)
                }
            ).collect();
            let local_variable_table_attribute = code_attribute.find_local_variable_table_attribute();
            let local_vars = local_variable_table_attribute.local_variable_table.iter().map(|local_variable_entry|
                LocalVariable {
                    method: uninitialized_method(),
                    start_pc: local_variable_entry.start_pc,
                    length: local_variable_entry.length,
                    index: local_variable_entry.index,
                    name: cp.resolve_utf8(local_variable_entry.name_index),
                    descriptor: cp.resolve_utf8(local_variable_entry.descriptor_index)
                }
            ).collect();
            let line_number_table_attribute = code_attribute.find_line_number_table_attribute();
            let line_numbers = line_number_table_attribute.line_number_table.iter().map(|line_number_entry|
                LineNumber {
                    start_pc: line_number_entry.start_pc,
                    line_number: line_number_entry.line_number
                }
            ).collect();

            let descriptor = cp.resolve_utf8(method_info.descriptor_index);
            //(vv,)v
            assert!(descriptor.starts_with("("));
            let params_and_return: Vec<&str> = descriptor.split(')').collect();
            assert_eq!(params_and_return.len(), 2);
            let mut params = params_and_return[0].to_string();
            params.remove(0);
            let parameter_descriptors = vec![params]; // TODO
            let return_descriptor = params_and_return[1].to_string();

            Method {
                access_flags: method_info.access_flags,
                name: cp.resolve_utf8(method_info.name_index),
                descriptor,
                class: uninitialized_class(),
                max_stack: code_attribute.max_stack as u32,
                max_locals: code_attribute.max_locals as u32,
                code: code_attribute.code.to_vec(),
                exceptions,
                local_vars,
                line_numbers,
                parameter_descriptors,
                return_descriptor
            }
        }).collect();
        Class {
            constant_pool,
            name: cp.resolve_class(classfile.this_class),
            access_flags: 0,
            super_class_name: cp.resolve_class(classfile.super_class),
            interface_names: classfile.interfaces.iter().map(|interface| cp.resolve_class(*interface)).collect(),
            super_class: uninitialized_class(),
            interfaces: vec![],
            fields,
            methods,
            instance_vars_count: 0,
            instance_var_fields: vec![],
            static_vars_count: 0,
            static_var_fields: vec![],
            static_vars: vec![],
            source_file: "".to_string(),
            component_type: Rc::new(Type::Byte),
            element_type: Rc::new(Type::Byte),
            dimensions: 0,
            defined: false,
            linked: false,
            initialized: 0
        }
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
    slot: u32
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
    return_descriptor:     String
}

#[derive(EnumAsInner)]
pub enum Constant {
    Unknown,
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Utf8(String),
    String(String),
    NameAndType{name: String, descriptor: String},
    Class(String),//Rc<Class>),
    FieldRef(String, String),//{class: Rc<Class>, filed: Rc<Field>},
    MethodRef(String, String),//{class: Rc<Class>, method: Rc<Method>}
    MethodType(String)
}

impl Constant {
    fn from(index: usize, constant_pool: &ConstantPool) -> Self {
        match &constant_pool.0[index] {
            ConstantPoolInfo::Unknown => Constant::Unknown,
            ConstantPoolInfo::Utf8(utf8_info) =>
                Constant::Utf8(constant_pool.resolve_utf8(index as u16)),
            ConstantPoolInfo::String(string_info) =>
                Constant::String(constant_pool.resolve_utf8(string_info.string_index)),
            ConstantPoolInfo::Integer(integer_info) =>
                Constant::Integer(i32::from_be_bytes(integer_info.bytes.to_be_bytes())),
            ConstantPoolInfo::Float(float_info) =>
                Constant::Float(f32::from_be_bytes(float_info.bytes.to_be_bytes())),
            ConstantPoolInfo::Long(long_info) =>
                Constant::Long(i64::from_be_bytes(((long_info.high_bytes as u64) << 32 | long_info.low_bytes as u64).to_be_bytes())),
            ConstantPoolInfo::Double(double_info) =>
                Constant::Double(f64::from_be_bytes(((double_info.high_bytes as u64) << 32 | double_info.low_bytes as u64).to_be_bytes())),
            ConstantPoolInfo::NameAndType(name_and_type_info) =>
                Constant::NameAndType {
                    name: constant_pool.resolve_utf8(name_and_type_info.name_index),
                    descriptor: constant_pool.resolve_utf8(name_and_type_info.descriptor_index)
                },
            ConstantPoolInfo::Class(class_info) =>
                Constant::Class(constant_pool.resolve_utf8(class_info.name_index)),
            ConstantPoolInfo::FieldRef(field_ref) => {
                let (name, descriptor) = constant_pool.resolve_name_and_type(field_ref.name_and_type_index);
                Constant::FieldRef(name, descriptor)
            }
            ConstantPoolInfo::MethodRef(method_ref) => {
                let (name, descriptor) = constant_pool.resolve_name_and_type(method_ref.name_and_type_index);
                Constant::MethodRef(name, descriptor)
            }
            ConstantPoolInfo::MethodType(method_type) =>
                Constant::MethodType(constant_pool.resolve_utf8(method_type.descriptor_index)),
            // ConstantPoolInfo::InterfaceMethodRef(_) => {}
            // ConstantPoolInfo::MethodHandle(_) => {}
            // ConstantPoolInfo::InvokeDynamic(_) => {}
            _ => {
                println!("ignore constant");
                Constant::Unknown
            }
        }
    }


}

pub struct ExceptionHandler {
    start_pc:   u16,
    end_pc:     u16,
    handler_pc: u16,
    catch_type: String // index of constant pool: ClassRef
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
    start_pc:    u16,
    line_number: u16
}