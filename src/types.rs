use std::rc::Rc;
use std::ptr::null;
use crate::constants::{FieldAccessFlag, JVM_SIGNATURE_CLASS, JVM_SIGNATURE_ENDCLASS, MethodAccessFlag};
use crate::values::Value;
use crate::class_file::*;
use crate::constants::*;


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
    pub(crate) defined: bool,
    pub(crate) linked:  bool,
    pub(crate) initialized: i32,

    // after linked
    pub(crate) super_class: Rc<Class>, // to be resolved
    pub(crate) interfaces: Vec<Rc<Class>>, // to be resolved

    // TODO

}

fn uninitialized_class() -> Rc<Class> {
    unsafe { Rc::<Class>::from_raw(null()) }
}

impl Class {
    pub(crate) fn from(classfile: &ClassFile) -> Self {
        let cp =  &classfile.constant_pool;
        let constant_pool_len = classfile.constant_pool.len();
        let constant_pool= (0..constant_pool_len).map(|i| Constant::from(i, cp)).collect();
        let mut field_slot = 0;
        let fields = classfile.fields.iter().map(|field_info| {
            let slot = field_slot;
            field_slot += 1;
            Field {
                access_flags: field_info.access_flags,
                name: Constant::resolve_utf8(field_info.name_index, cp),
                descriptor: Constant::resolve_utf8(field_info.descriptor_index, cp),
                class: uninitialized_class(),
                slot
            }
        }).collect();

        let mut method_slot = 0;
        let methods = classfile.methods.iter().map(|method_info| {
            let slot = method_slot;
            method_slot += 1;
            let mut code_attribute_option = Option::None;
            method_info.attributes.iter().for_each(|attribute| {
                match attribute {
                    AttributeInfo::Code(code_attribute) => {
                        code_attribute_option = Option::Some(code_attribute);
                    },
                    _ => {}
                }
            });
            let code_attribute = code_attribute_option.unwrap();
            Method {
                access_flags: method_info.access_flags,
                name: Constant::resolve_utf8(method_info.name_index, cp),
                descriptor: Constant::resolve_utf8(method_info.descriptor_index, cp),
                class: uninitialized_class(),
                max_stack: code_attribute.max_stack as u32,
                max_locals: code_attribute.max_locals as u32,
                code: code_attribute.code.to_vec(),
                exceptions: vec![],
                local_vars: vec![],
                line_numbers: vec![],
                parameter_descriptors: vec![],
                return_descriptor: vec![]
            }
        }).collect();
        Class {
            constant_pool,
            name: Constant::resolve_class(classfile.this_class, cp),
            access_flags: 0,
            super_class_name: Constant::resolve_class(classfile.super_class, cp),
            interface_names: classfile.interfaces.iter().map(|interface| Constant::resolve_class(*interface, cp)).collect(),
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
    fn from(index: usize, constant_pool: &Vec<ConstantPoolInfo>) -> Self {
        match &constant_pool[index] {
            ConstantPoolInfo::Unknown => Constant::Unknown,
            ConstantPoolInfo::Utf8(utf8_info) =>
                Constant::Utf8(Constant::resolve_utf8(index as u16, constant_pool)),
            ConstantPoolInfo::String(string_info) =>
                Constant::String(Constant::resolve_utf8(string_info.string_index, constant_pool)),
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
                    name: Constant::resolve_utf8(name_and_type_info.name_index, constant_pool),
                    descriptor: Constant::resolve_utf8(name_and_type_info.descriptor_index, constant_pool)
                },
            ConstantPoolInfo::Class(class_info) =>
                Constant::Class(Constant::resolve_utf8(class_info.name_index, constant_pool)),
            ConstantPoolInfo::FieldRef(field_ref) => {
                let (name, descriptor) = Constant::resolve_name_and_type(field_ref.name_and_type_index, constant_pool);
                Constant::FieldRef(name, descriptor)
            }
            ConstantPoolInfo::MethodRef(method_ref) => {
                let (name, descriptor) = Constant::resolve_name_and_type(method_ref.name_and_type_index, constant_pool);
                Constant::MethodRef(name, descriptor)
            }
            ConstantPoolInfo::MethodType(method_type) =>
                Constant::MethodType(Constant::resolve_utf8(method_type.descriptor_index, constant_pool)),
            // ConstantPoolInfo::InterfaceMethodRef(_) => {}
            // ConstantPoolInfo::MethodHandle(_) => {}
            // ConstantPoolInfo::InvokeDynamic(_) => {}
            _ => {
                println!("ignore constant");
                Constant::Unknown
            }
        }
    }

    fn resolve_name_and_type(name_and_type_index: u16, constant_pool: &Vec<ConstantPoolInfo>) -> (String, String) {
        match &constant_pool[name_and_type_index as usize] {
            ConstantPoolInfo::NameAndType(name_and_type_info) =>
                (Constant::resolve_utf8(name_and_type_info.name_index, constant_pool), Constant::resolve_utf8(name_and_type_info.descriptor_index, constant_pool)),
            _ => panic!("name_and_type constant is not indexed as a NameAndType")
        }
    }

    fn resolve_utf8(utf8_index: u16, constant_pool: &Vec<ConstantPoolInfo>) -> String {
        match &constant_pool[utf8_index as usize] {
            ConstantPoolInfo::Utf8(utf8_info) => String::from_utf8(utf8_info.bytes.clone()).unwrap(),
            _ => panic!("utf8 constant is not indexed as a utf8")
        }
    }

    fn resolve_class(class_index: u16, constant_pool: &Vec<ConstantPoolInfo>) -> String {
        match &constant_pool[class_index as usize] {
            ConstantPoolInfo::Class(class_info) => Constant::resolve_utf8(class_info.name_index, constant_pool),
            _ => panic!("class constant is not indexed as a Class")
        }
    }
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