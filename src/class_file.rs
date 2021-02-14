use std::rc::Rc;
use std::convert::TryInto;
use std::any::Any;
use std::fs;
use crate::constants::*;

// bytes definition for jvm_spec naming
pub type u1 = u8;
pub type u2 = u16;
pub type u4 = u32;

pub fn read(file: &str) -> ClassFile {
    let mut buffer = Buffer::from(file);
    let classfile = ClassFile::from(&mut buffer);
    println!("{}", classfile.major_version);
    classfile
}

pub struct Buffer {
    bytes: Vec<u1>,
    pos: usize
}

impl Buffer {
    fn from(file: &str) -> Self {
        Buffer {
            bytes: fs::read(file).unwrap(),
            pos: 0
        }
    }
}

impl Buffer {
    fn u1(&mut self) -> u1 {
        let value = self.bytes[self.pos];
        self.pos += 1;
        value
    }

    fn u2(&mut self) -> u2 {
        let value = u2::from_be_bytes([
            self.bytes[self.pos+0], self.bytes[self.pos+1]
        ]);
        self.pos += 2;
        value
    }

    fn u4(&mut self) -> u4 {
        let value = u4::from_be_bytes([
            self.bytes[self.pos+0], self.bytes[self.pos+1],
            self.bytes[self.pos+2], self.bytes[self.pos+3]
        ]);
        self.pos += 4;
        value
    }

    fn bytes(&mut self, length: usize) -> Vec<u1> {
        let mut vec = Vec::with_capacity(length);
        for i in 0..length {
            vec.push(self.bytes[self.pos + i])
        }
        self.pos += length;
        vec
    }
}

trait Resolve {
    fn resolve(&mut self, constant_pool: &Vec<ConstantPoolInfo>);
}

/*
ClassFile {
	u4				magic;
	u2 				minor_version;
	u2 				major_version;
	u2 				constant_pool_count;
	cp_info 		constant_pool[constant_pool_count-1];
	u2 				access_flags;
	u2 				this_class;
	u2 				super_class;
	u2 				interfaces_count;
	u2 				interfaces[interfaces_count];
	u2 				fields_count;
	field_info 		fields[fields_count];
	u2 				methods_count;
	method_info 	methods[methods_count];
	u2 				attributes_count;
	attribute_info 	attributes[attributes_count];
}
*/
pub struct ClassFile {
    pub(crate) magic: u4,
    pub(crate) minor_version: u2,
    pub(crate) major_version: u2,
    pub(crate) constant_pool_count: u2,
    pub(crate) constant_pool: Vec<ConstantPoolInfo>,
    pub(crate) access_flags: u2,
    pub(crate) this_class: u2,
    pub(crate) super_class: u2,
    pub(crate) interfaces_count: u2,
    pub(crate) interfaces: Vec<u2>,
    pub(crate) fields_count: u2,
    pub(crate) fields: Vec<FieldInfo>,
    pub(crate) methods_count: u2,
    pub(crate) methods: Vec<MethodInfo>,
    pub(crate) attributes_count: u2,
    pub(crate) attributes: Vec<AttributeInfo>
}

impl ClassFile {

    fn from(buffer: &mut Buffer) -> ClassFile {
        let magic = buffer.u4();
        let minor_version = buffer.u2();
        let major_version = buffer.u2();
        let constant_pool_count = buffer.u2();
        let constant_pool = ConstantPoolInfo::with_capacity(buffer, constant_pool_count);
        let access_flags = buffer.u2();
        let this_class = buffer.u2();
        let super_class = buffer.u2();
        let interfaces_count = buffer.u2();
        let interfaces = (0..interfaces_count).map(|_| buffer.u2()).collect();
        let fields_count = buffer.u2();
        let fields = FieldInfo::with_capacity(buffer, &constant_pool, fields_count);
        let methods_count = buffer.u2();
        let methods = MethodInfo::with_capacity(buffer, &constant_pool, methods_count);
        let attributes_count = buffer.u2();
        let attributes = AttributeInfo::with_capacity(buffer, &constant_pool, attributes_count);
        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes
        }
    }
}

/*
cp_info {
    u1 tag;
    u1 info[];
}
*/
pub enum ConstantPoolInfo {
    Unknown, // not used as 1st one
    Class(ConstantClassInfo),
    FieldRef(ConstantFieldRefInfo),
    MethodRef(ConstantMethodRefInfo),
    InterfaceMethodRef(ConstantInterfaceMethodrefInfo),
    String(ConstantStringInfo),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    NameAndType(ConstantNameAndTypeInfo),
    Utf8(ConstantUtf8Info),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo)
}

impl ConstantPoolInfo {
    fn with_capacity(buffer: &mut Buffer, constant_pool_count: u2) -> Vec<Self> {
        let mut constant_pool = Vec::<ConstantPoolInfo>::with_capacity(constant_pool_count as usize);
        constant_pool.push(ConstantPoolInfo::Unknown);
        (1..constant_pool_count).for_each(|_| constant_pool.push(ConstantPoolInfo::from(buffer)));
        constant_pool
    }

    fn from(buffer: &mut Buffer) -> Self {
        let tag = buffer.u1();
        match tag {
            JVM_TAG_CLASS =>
                ConstantPoolInfo::Class(
                    ConstantClassInfo { tag, name_index: buffer.u2() }
                ),
            JVM_TAG_FIELDREF =>
                ConstantPoolInfo::FieldRef(
                    ConstantFieldRefInfo {
                        tag,
                        class_index: buffer.u2(),
                        name_and_type_index: buffer.u2()
                    }
                ),
            JVM_TAG_METHODREF =>
                ConstantPoolInfo::MethodRef(
                    ConstantMethodRefInfo {
                        tag,
                        class_index: buffer.u2(),
                        name_and_type_index: buffer.u2()
                    }
                ),
            JVM_TAG_INTERFACE_METHODREF =>
                ConstantPoolInfo::InterfaceMethodRef(
                    ConstantInterfaceMethodrefInfo {
                        tag,
                        class_index: buffer.u2(),
                        name_and_type_index: buffer.u2()
                    }
                ),
            JVM_TAG_STRING =>
                ConstantPoolInfo::String(
                    ConstantStringInfo {
                        tag,
                        string_index: buffer.u2()
                    }
                ),
            JVM_TAG_INTEGER =>
                ConstantPoolInfo::Integer(
                    ConstantIntegerInfo {
                        tag,
                        bytes: buffer.u4()
                    }
                ),
            JVM_TAG_FLOAT =>
                ConstantPoolInfo::Float(
                    ConstantFloatInfo {
                        tag,
                        bytes: buffer.u4()
                    }
                ),
            JVM_TAG_LONG =>
                ConstantPoolInfo::Long(
                    ConstantLongInfo {
                        tag,
                        high_bytes: buffer.u4(),
                        low_bytes: buffer.u4()
                    }
                ),
            JVM_TAG_DOUBLE =>
                ConstantPoolInfo::Double(
                    ConstantDoubleInfo {
                        tag,
                        high_bytes: buffer.u4(),
                        low_bytes: buffer.u4()
                    }
                ),
            JVM_TAG_NAME_AND_TYPE =>
                ConstantPoolInfo::NameAndType(
                    ConstantNameAndTypeInfo {
                        tag,
                        name_index: buffer.u2(),
                        descriptor_index: buffer.u2()
                    }
                ),
            JVM_TAG_UTF8 => {
                let length = buffer.u2();
                ConstantPoolInfo::Utf8(
                    ConstantUtf8Info {
                        tag,
                        length,
                        bytes: buffer.bytes(length as usize)
                    }
                )
            }
            JVM_TAG_METHOD_HANDLE =>
                ConstantPoolInfo::MethodHandle(
                    ConstantMethodHandleInfo {
                        tag,
                        reference_kind: buffer.u1(),
                        reference_index: buffer.u2()
                    }
                ),
            JVM_TAG_METHOD_TYPE =>
                ConstantPoolInfo::MethodType(
                    ConstantMethodTypeInfo {
                        tag,
                        descriptor_index: buffer.u2()
                    }
                ),
            JVM_TAG_INVOKE_DYNAMIC =>
                ConstantPoolInfo::InvokeDynamic(
                    ConstantInvokeDynamicInfo {
                        tag,
                        bootstrap_method_attr_index: buffer.u2(),
                        name_and_type_index: buffer.u2()
                    }
                ),
            _ => panic!("Not a supported constant tag {}", tag)
        }
    }

}

/*
CONSTANT_Class_info {
    u1 tag;
    u2 name_index;
}
*/
pub struct ConstantClassInfo {
    tag: u1,
    pub(crate) name_index: u2
}

/*
CONSTANT_Fieldref_info {
    u1 tag;
    u2 class_index;
    u2 name_and_type_index;
}
*/
pub struct ConstantFieldRefInfo {
    tag: u1,
    pub(crate) class_index: u2,
    pub(crate) name_and_type_index: u2,
}

/*
CONSTANT_Methodref_info {
    u1 tag;
    u2 class_index;
    u2 name_and_type_index;
}
*/
pub struct ConstantMethodRefInfo {
    tag: u1,
    pub(crate) class_index: u2,
    pub(crate) name_and_type_index: u2,
}

/*
CONSTANT_InterfaceMethodref_info {
    u1 tag;
    u2 class_index;
    u2 name_and_type_index;
}
*/
pub struct ConstantInterfaceMethodrefInfo  {
    tag              : u1,
    pub(crate) class_index       : u2,
    pub(crate) name_and_type_index : u2
}

/*
CONSTANT_String_info {
    u1 tag;
    u2 string_index;
}
*/
pub struct ConstantStringInfo {
    tag: u1,
    pub(crate) string_index: u2
}

/*
CONSTANT_Integer_info {
    u1 tag;
    u4 bytes;
}
*/
pub struct ConstantIntegerInfo {
    tag: u1,
    pub(crate) bytes: u4
}

/*
CONSTANT_Float_info {
    u1 tag;
    u4 bytes;
}
*/
pub struct ConstantFloatInfo {
    tag: u1,
    pub(crate) bytes: u4
}

/*
CONSTANT_Long_info {
    u1 tag;
    u4 high_bytes;
    u4 low_bytes;
}
*/
pub struct ConstantLongInfo {
    tag: u1,
    pub(crate) high_bytes: u4,
    pub(crate) low_bytes: u4
}

/*
CONSTANT_Double_info {
    u1 tag;
    u4 high_bytes;
    u4 low_bytes;
}
*/
pub struct ConstantDoubleInfo{
    tag: u1,
    pub(crate) high_bytes: u4,
    pub(crate) low_bytes: u4
}

/*
CONSTANT_NameAndType_info {
    u1 tag;
    u2 name_index;
    u2 descriptor_index;
}
*/
pub struct ConstantNameAndTypeInfo {
    tag: u1,
    pub(crate) name_index: u2,
    pub(crate) descriptor_index: u2
}

/*
CONSTANT_Utf8_info {
    u1 tag;
    u2 length;
    u1 bytes[length];
}
*/
pub struct ConstantUtf8Info {
    tag: u1,
    pub(crate) length: u2,
    pub(crate) bytes: Vec<u1>
}

impl ConstantUtf8Info {
    // TODO refine clone
    pub fn value(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.bytes.clone()) }
    }
}

/*
CONSTANT_MethodHandle_info {
    u1 tag;
    u1 reference_kind;
    u2 reference_index;
}
*/
pub struct ConstantMethodHandleInfo {
    tag: u1,
    pub(crate) reference_kind: u1,
    pub(crate) reference_index: u2
}

/*
CONSTANT_MethodType_info {
    u1 tag;
    u2 descriptor_index;
}
*/
pub struct ConstantMethodTypeInfo {
    tag: u1,
    pub(crate) descriptor_index: u2
}

/*
CONSTANT_InvokeDynamic_info {
    u1 tag;
    u2 bootstrap_method_attr_index;
    u2 name_and_type_index;
}
*/
pub struct ConstantInvokeDynamicInfo {
    tag: u1,
    pub(crate) bootstrap_method_attr_index: u2,
    pub(crate) name_and_type_index: u2
}

/*
field_info {
    u2             access_flags;
    u2             name_index;
    u2             descriptor_index;
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/
pub struct FieldInfo {
    pub(crate) access_flags: u2,
    pub(crate) name_index: u2,
    pub(crate) descriptor_index: u2,
    pub(crate) attribute_count: u2,
    pub(crate) attributes: Vec<AttributeInfo>
}

impl FieldInfo {
    fn from(buffer: &mut Buffer, constant_pool: &Vec<ConstantPoolInfo>) -> Self {
        let access_flags = buffer.u2();
        let name_index = buffer.u2();
        let descriptor_index = buffer.u2();
        let attribute_count = buffer.u2();
        let attributes = AttributeInfo::with_capacity(buffer, constant_pool, attribute_count);
        FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attribute_count,
            attributes
        }
    }

    fn with_capacity(buffer: &mut Buffer, constant_pool: &Vec<ConstantPoolInfo>, fields_count: u2) -> Vec<Self> {
        (0..fields_count).map(|_| FieldInfo::from(buffer, &constant_pool)).collect()
    }
}

pub struct MethodInfo {
    pub(crate) access_flags: u2,
    pub(crate) name_index: u2,
    pub(crate) descriptor_index: u2,
    pub(crate) attribute_count: u2,
    pub(crate) attributes: Vec<AttributeInfo>
}

/*
method_info {
    u2             access_flags;
    u2             name_index;
    u2             descriptor_index;
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/
impl MethodInfo {
    fn from(buffer: &mut Buffer, constant_pool: &Vec<ConstantPoolInfo>) -> Self {
        let access_flags = buffer.u2();
        let name_index = buffer.u2();
        let descriptor_index = buffer.u2();
        let attribute_count = buffer.u2();
        let attributes = AttributeInfo::with_capacity(buffer, constant_pool, attribute_count);
        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attribute_count,
            attributes
        }
    }

    fn with_capacity(buffer: &mut Buffer, constant_pool: &Vec<ConstantPoolInfo>, method_count: u2) -> Vec<Self> {
        (0..method_count).map(|_| MethodInfo::from(buffer, &constant_pool)).collect()
    }
}

/*
attribute_info {
    u2 attribute_name_index;
    u4 attribute_length;
    u1 info[attribute_length];
}
*/
pub enum AttributeInfo {
    Unknown,
    Code(CodeAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    SourceFile(SourceFileAttribute),
    RuntimeVisibleAnnotation(RuntimeVisibleAnnotationAttribute)
}

impl AttributeInfo {
    fn from(buffer: &mut Buffer, constant_pool: &Vec<ConstantPoolInfo>) -> Self {
        let attribute_name_index = buffer.u2();
        let attribute_length = buffer.u4();
        let const_pool_info = &constant_pool[attribute_name_index as usize];
        let attribute_name = match const_pool_info {
            ConstantPoolInfo::Utf8(utf8) => utf8.value(),
            _ => {
                println!("not a utf8 attribute name");
                "".to_string()
            }
        };
        match attribute_name.as_str() {
            "Code" => {
                let max_stack= buffer.u2();
                let max_locals = buffer.u2();
                let code_length = buffer.u4();
                let code = buffer.bytes(code_length as usize);
                let exception_table_length = buffer.u2();
                let mut exception_table = Vec::<ExceptionTableEntry>::with_capacity(exception_table_length as usize);
                for _ in 0..exception_table_length {
                    exception_table.push(ExceptionTableEntry {
                        start_pc: buffer.u2(),
                        end_pc: buffer.u2(),
                        handle_pc: buffer.u2(),
                        catch_type: buffer.u2()
                    })
                }
                let attributes_count = buffer.u2();
                let attributes = AttributeInfo::with_capacity(buffer, constant_pool, attributes_count);
                let code = CodeAttribute {
                    attribute_name_index,
                    attribute_length,
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exception_table_length,
                    exception_table,
                    attributes_count,
                    attributes
                };
                println!("code.attributes: {}", code.attributes.len());
                AttributeInfo::Code(
                    code
                )
            }
            "LineNumberTable" => {
                let line_number_table_length = buffer.u2();
                let mut line_number_table = Vec::<LineNumberTableEntry>::with_capacity(line_number_table_length as usize);
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberTableEntry { start_pc: buffer.u2(), line_number: buffer.u2() })
                }
                AttributeInfo::LineNumberTable(
                    LineNumberTableAttribute {
                        attribute_name_index,
                        attribute_length,
                        line_number_table_length,
                        line_number_table
                    }
                )
            }
            "LocalVariableTable" => {
                let local_variable_table_length = buffer.u2();
                let mut local_variable_table = Vec::<LocalVariableTableEntry>::with_capacity(local_variable_table_length as usize);
                for _ in 0..local_variable_table_length {
                    local_variable_table.push(LocalVariableTableEntry {
                        start_pc: buffer.u2(),
                        length: buffer.u2(),
                        name_index: buffer.u2(),
                        descriptor_index: buffer.u2(),
                        index: buffer.u2()
                    })
                }
                AttributeInfo::LocalVariableTable(
                    LocalVariableTableAttribute {
                        attribute_name_index,
                        attribute_length,
                        local_variable_table_length,
                        local_variable_table
                    }
                )
            }
            "SourceFile" => {
                AttributeInfo::SourceFile(
                    SourceFileAttribute {
                        attribute_name_index,
                        attribute_length,
                        source_file_index: buffer.u2()
                    }
                )
            }
            _ => {
                println!("Skip not parsed attribute: {}", attribute_name);
                buffer.bytes(attribute_length as usize); // skip
                AttributeInfo::Unknown
            }
        }
    }

    fn with_capacity(buffer: &mut Buffer, constant_pool: &Vec<ConstantPoolInfo>, attributes_count: u2) -> Vec<Self> {
        (0..attributes_count).map(|_| AttributeInfo::from(buffer, constant_pool)).collect()
    }
}

/*
Code_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 max_stack;
    u2 max_locals;
    u4 code_length;
    u1 code[code_length];
    u2 exception_table_length;
    {   u2 start_pc;
        u2 end_pc;
        u2 handler_pc;
        u2 catch_type;
    } exception_table[exception_table_length];
    u2 attributes_count;
    attribute_info attributes[attributes_count];
}
*/
pub struct  CodeAttribute {
    pub(crate) attribute_name_index: u2,
    pub(crate) attribute_length: u4,
    pub(crate) max_stack: u2,
    pub(crate) max_locals: u2,
    pub(crate) code_length: u4,
    pub(crate) code: Vec<u1>,
    pub(crate) exception_table_length: u2,
    pub(crate) exception_table: Vec<ExceptionTableEntry>,
    pub(crate) attributes_count: u2,
    pub(crate) attributes: Vec<AttributeInfo>
}

pub struct ExceptionTableEntry {
    pub(crate) start_pc: u2,
    pub(crate) end_pc: u2,
    pub(crate) handle_pc: u2,
    pub(crate) catch_type: u2
}

/*
LineNumberTable_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 line_number_table_length;
    {   u2 start_pc;
        u2 line_number;
    } line_number_table[line_number_table_length];
}
*/
pub struct LineNumberTableAttribute {
    pub(crate) attribute_name_index: u2,
    pub(crate) attribute_length: u4,
    pub(crate) line_number_table_length: u2,
    pub(crate) line_number_table: Vec<LineNumberTableEntry>
}

pub struct LineNumberTableEntry {
    pub(crate) start_pc: u2,
    pub(crate) line_number: u2
}

/*
LocalVariableTable_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 local_variable_table_length;
    {   u2 start_pc;
        u2 length;
        u2 name_index;
        u2 descriptor_index;
        u2 index;
    } local_variable_table[local_variable_table_length];
}
*/
pub struct LocalVariableTableAttribute {
    pub(crate) attribute_name_index: u2,
    pub(crate) attribute_length: u4,
    pub(crate) local_variable_table_length: u2,
    pub(crate) local_variable_table: Vec<LocalVariableTableEntry>
}

pub struct LocalVariableTableEntry {
    pub(crate) start_pc: u2,
    pub(crate) length: u2,
    pub(crate) name_index: u2,
    pub(crate) descriptor_index: u2,
    pub(crate) index: u2
}

/*
SourceFile_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 sourcefile_index;
}
*/
pub struct SourceFileAttribute {
    pub(crate) attribute_name_index: u2,
    pub(crate) attribute_length: u4,
    pub(crate) source_file_index: u2
}

/*
RuntimeVisibleAnnotations_attribute {
    u2         attribute_name_index;
    u4         attribute_length;
    u2         num_annotations;
    annotation annotations[num_annotations];
}
*/
pub struct RuntimeVisibleAnnotationAttribute {
    pub(crate) attribute_name_index: u2,
    pub(crate) attribute_length: u4,
    pub(crate) num_annotations: u2,
    pub(crate) annotations: Vec<Annotation>
}

/*
annotation {
    u2 type_index;
    u2 num_element_value_pairs;
    {   u2            element_name_index;
        element_value value;
    } element_value_pairs[num_element_value_pairs];
}
*/
pub struct Annotation {
    pub(crate) type_index: u2,
    pub(crate) num_element_value_pairs: u2,
    pub(crate) element_value_pairs: Vec<AnnotationElementValuePair>,
}

pub struct AnnotationElementValuePair {
    pub(crate) element_name_index: u2,
    pub(crate) value: ElementValue
}

/*
element_value {
    u1 tag;
    union {
        u2 const_value_index;

        {   u2 type_name_index;
            u2 const_name_index;
        } enum_const_value;

        u2 class_info_index;

        annotation annotation_value;

        {   u2            num_values;
            element_value values[num_values];
        } array_value;
    } value;
}
*/
pub struct ElementValue {
    tag: u1,
    pub(crate) const_value_index: u2,
    pub(crate) enum_const_value: EnumConstValue,
    pub(crate) class_info_index: u2,
    pub(crate) annotation_value: Annotation,
    pub(crate) array_value: ElementValueArray
}

pub struct EnumConstValue {
    pub(crate) type_name_index: u2,
    pub(crate) const_name_index: u2
}

pub struct ElementValueArray {
    pub(crate) num_values: u2,
    pub(crate) values: Vec<Rc<ElementValue>>
}