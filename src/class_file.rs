use std::rc::Rc;
use std::convert::TryInto;
use std::any::Any;
use std::fs;

pub type u1 = u8;
pub type u2 = u16;
pub type u4 = u32;

pub fn read(file: &str) -> ClassFile {
    let mut bytecode = ByteCodeVisitor::from(file);
    let classfile = ClassFile::from(&mut bytecode);
    println!("{}", classfile.major_version);
    classfile
}

pub struct ByteCodeVisitor {
    bytecode: Vec<u1>,
    pos: usize
}

impl ByteCodeVisitor {
    fn from(file: &str) -> Self {
        ByteCodeVisitor {
            bytecode: fs::read(file).unwrap(),
            pos: 0
        }
    }
}

impl ByteCodeVisitor {
    fn u1(&mut self) -> u1 {
        let value = self.bytecode[self.pos];
        self.pos += 1;
        value
    }

    fn u2(&mut self) -> u2 {
        let value = u2::from_be_bytes([
            self.bytecode[self.pos+0], self.bytecode[self.pos+1]
        ]);
        self.pos += 2;
        value
    }

    fn u4(&mut self) -> u4 {
        let value = u4::from_be_bytes([
            self.bytecode[self.pos+0], self.bytecode[self.pos+1],
            self.bytecode[self.pos+2], self.bytecode[self.pos+3]
        ]);
        self.pos += 4;
        value
    }

    fn bytes(&self, length: usize) -> Vec<u1> {
        let mut vec = Vec::with_capacity(length);
        for i in 0..length {
            vec.push(self.bytecode[self.pos + i])
        }
        vec
    }
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
    magic: u4,
    minor_version: u2,
    major_version: u2,
    constant_pool_count: u2,
    constant_pool: Vec<ConstantPoolInfo>,
    access_flags: u2,
    this_class: u2,
    super_class: u2,
    interfaces_count: u2,
    interfaces: Vec<u2>,
    fields_count: u2,
    fields: Vec<FieldInfo>,
    methods_count: u2,
    methods: Vec<MethodInfo>,
    attributes_count: u2,
    attributes: Vec<AttributeInfo>
}

impl ClassFile {

    fn from(bytecode: &mut ByteCodeVisitor) -> ClassFile {
        let magic = bytecode.u4();
        let minor_version = bytecode.u2();
        let major_version = bytecode.u2();
        let constant_pool_count = bytecode.u2();
        let constant_pool: Vec<ConstantPoolInfo> = (0..constant_pool_count).map(|_| ConstantPoolInfo::from(bytecode)).collect();
        let access_flags = bytecode.u2();
        let this_class = bytecode.u2();
        let super_class = bytecode.u2();
        let interfaces_count = bytecode.u2();
        let interfaces = (0..interfaces_count).map(|_| bytecode.u2()).collect();
        let fields_count = bytecode.u2();
        let fields = FieldInfo::with_capacity(bytecode, &constant_pool, fields_count);
        let methods_count = bytecode.u2();
        let methods = MethodInfo::with_capacity(bytecode, &constant_pool, methods_count);
        let attributes_count = bytecode.u2();
        let attributes = AttributeInfo::with_capacity(bytecode, &constant_pool, attributes_count);
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
    Class(ConstantClassInfo),
    Fieldref(ConstantFieldrefInfo),
    Methodref(ConstantMethodrefInfo),
    InterfaceMethodref(ConstantInterfaceMethodrefInfo),
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
    fn from(bytecode: &mut ByteCodeVisitor) -> Self {
        let tag = bytecode.u1();
        match tag {
            JVM_TAG_Class=>
                ConstantPoolInfo::Class(
                    ConstantClassInfo { tag, name_index: bytecode.u2() }
                ),
            JVM_TAG_Fieldref =>
                ConstantPoolInfo::Fieldref(
                    ConstantFieldrefInfo {
                        tag,
                        class_index: bytecode.u2(),
                        name_and_type_index: bytecode.u2()
                    }
                ),
            JVM_TAG_Methodref =>
                ConstantPoolInfo::Methodref(
                    ConstantMethodrefInfo {
                        tag,
                        class_index: bytecode.u2(),
                        name_and_type_index: bytecode.u2()
                    }
                ),
            JVM_TAG_InterfaceMethodref =>
                ConstantPoolInfo::InterfaceMethodref(
                    ConstantInterfaceMethodrefInfo {
                        tag,
                        class_index: bytecode.u2(),
                        name_and_type_index: bytecode.u2()
                    }
                ),
            JVM_TAG_String =>
                ConstantPoolInfo::String(
                    ConstantStringInfo {
                        tag,
                        string_index: bytecode.u2()
                    }
                ),
            JVM_TAG_Integer =>
                ConstantPoolInfo::Integer(
                    ConstantIntegerInfo {
                        tag,
                        bytes: bytecode.u4()
                    }
                ),
            JVM_TAG_Float =>
                ConstantPoolInfo::Float(
                    ConstantFloatInfo {
                        tag,
                        bytes: bytecode.u4()
                    }
                ),
            JVM_TAG_Long =>
                ConstantPoolInfo::Long(
                    ConstantLongInfo {
                        tag,
                        high_bytes: bytecode.u4(),
                        low_bytes: bytecode.u4()
                    }
                ),
            JVM_TAG_Double =>
                ConstantPoolInfo::Double(
                    ConstantDoubleInfo {
                        tag,
                        high_bytes: bytecode.u4(),
                        low_bytes: bytecode.u4()
                    }
                ),
            JVM_TAG_NameAndType =>
                ConstantPoolInfo::Methodref(
                    ConstantMethodrefInfo {
                        tag,
                        class_index: bytecode.u2(),
                        name_and_type_index: bytecode.u2()
                    }
                ),
            JVM_TAG_Utf8 => {
                let length = bytecode.u2();
                ConstantPoolInfo::Utf8(
                    ConstantUtf8Info {
                        tag,
                        length,
                        bytes: bytecode.bytes(length as usize)
                    }
                )
            },
            JVM_TAG_MethodHandle =>
                ConstantPoolInfo::MethodHandle(
                    ConstantMethodHandleInfo {
                        tag,
                        reference_kind: bytecode.u1(),
                        reference_index: bytecode.u2()
                    }
                ),
            JVM_TAG_MethodType =>
                ConstantPoolInfo::MethodType(
                    ConstantMethodTypeInfo {
                        tag,
                        descriptor_index: bytecode.u2()
                    }
                ),
            JVM_TAG_InvokeDynamic=>
                ConstantPoolInfo::InvokeDynamic(
                    ConstantInvokeDynamicInfo {
                        tag,
                        bootstrap_method_attr_index: bytecode.u2(),
                        name_and_type_index: bytecode.u2()
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
    name_index: u2
}

/*
CONSTANT_Fieldref_info {
    u1 tag;
    u2 class_index;
    u2 name_and_type_index;
}
*/
pub struct ConstantFieldrefInfo {
    tag: u1,
    class_index: u2,
    name_and_type_index: u2
}

/*
CONSTANT_Methodref_info {
    u1 tag;
    u2 class_index;
    u2 name_and_type_index;
}
*/
pub struct ConstantMethodrefInfo {
    tag: u1,
    class_index: u2,
    name_and_type_index: u2
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
    class_index       : u2,
    name_and_type_index : u2
}

/*
CONSTANT_String_info {
    u1 tag;
    u2 string_index;
}
*/
pub struct ConstantStringInfo {
    tag: u1,
    string_index: u2
}

/*
CONSTANT_Integer_info {
    u1 tag;
    u4 bytes;
}
*/
pub struct ConstantIntegerInfo {
    tag: u1,
    bytes: u4
}

/*
CONSTANT_Float_info {
    u1 tag;
    u4 bytes;
}
*/
pub struct ConstantFloatInfo {
    tag: u1,
    bytes: u4
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
    high_bytes: u4,
    low_bytes: u4
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
    high_bytes: u4,
    low_bytes: u4
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
    name_index: u2,
    descriptor_index: u2
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
    length: u2,
    bytes: Vec<u1>
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
    reference_kind: u1,
    reference_index: u2
}

/*
CONSTANT_MethodType_info {
    u1 tag;
    u2 descriptor_index;
}
*/
pub struct ConstantMethodTypeInfo {
    tag: u1,
    descriptor_index: u2
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
    bootstrap_method_attr_index: u2,
    name_and_type_index: u2
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
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attribute_count: u2,
    attributes: Vec<AttributeInfo>
}

impl FieldInfo {
    fn from(bytecode: &mut ByteCodeVisitor, constant_pool: &Vec<ConstantPoolInfo>) -> Self {
        let access_flags = bytecode.u2();
        let name_index = bytecode.u2();
        let descriptor_index = bytecode.u2();
        let attribute_count = bytecode.u2();
        let attributes = AttributeInfo::with_capacity(bytecode, constant_pool, attribute_count);
        FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attribute_count,
            attributes
        }
    }

    fn with_capacity(bytecode: &mut ByteCodeVisitor, constant_pool: &Vec<ConstantPoolInfo>, fields_count: u2) -> Vec<Self> {
        (0..fields_count).map(|_| FieldInfo::from(bytecode, &constant_pool)).collect()
    }
}

pub struct MethodInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attribute_count: u2,
    attributes: Vec<AttributeInfo>
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
    fn from(bytecode: &mut ByteCodeVisitor, constant_pool: &Vec<ConstantPoolInfo>) -> Self {
        let access_flags = bytecode.u2();
        let name_index = bytecode.u2();
        let descriptor_index = bytecode.u2();
        let attribute_count = bytecode.u2();
        let attributes = AttributeInfo::with_capacity(bytecode, constant_pool, attribute_count);
        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attribute_count,
            attributes
        }
    }

    fn with_capacity(bytecode: &mut ByteCodeVisitor, constant_pool: &Vec<ConstantPoolInfo>, method_count: u2) -> Vec<Self> {
        (0..method_count).map(|_| MethodInfo::from(bytecode, &constant_pool)).collect()
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
    LineNumber(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    SourceFile(SourceFileAttribute),
    RuntimeVisibleAnnotation(RuntimeVisibleAnnotationAttribute)
}

impl AttributeInfo {
    fn from(bytecode: &mut ByteCodeVisitor, constant_pool: &Vec<ConstantPoolInfo>) -> Self {
        let attribute_name_index = bytecode.u2();
        let attribute_length = bytecode.u4();
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
                let max_stack= bytecode.u2();
                let max_locals = bytecode.u2();
                let code_length = bytecode.u4();
                let code = bytecode.bytes(code_length as usize);
                let exception_table_length = bytecode.u2();
                let mut exception_table = Vec::<ExceptionTableEntry>::with_capacity(exception_table_length as usize);
                for _ in 0..exception_table_length {
                    exception_table.push(ExceptionTableEntry {
                        start_pc: bytecode.u2(),
                        end_pc: bytecode.u2(),
                        handle_pc: bytecode.u2(),
                        catch_type: bytecode.u2()
                    })
                }
                let attributes_count = bytecode.u2();
                let code_attributes = AttributeInfo::with_capacity(bytecode, constant_pool, attributes_count);
                AttributeInfo::Code(
                    CodeAttribute {
                        attribute_name_index,
                        attribute_length,
                        max_stack,
                        max_locals,
                        code_length,
                        code,
                        exception_table_length,
                        exception_table,
                        attributes_count,
                        attributes: code_attributes
                    }
                )
            }
            _ => {
                println!("Skip not parsed attribute: {}", attribute_name);
                bytecode.bytes(attribute_length as usize); // skip
                AttributeInfo::Unknown
            }
        }
    }

    fn with_capacity(bytecode: &mut ByteCodeVisitor, constant_pool: &Vec<ConstantPoolInfo>, attributes_count: u2) -> Vec<Self> {
        (0..attributes_count).map(|_| AttributeInfo::from(bytecode, constant_pool)).collect()
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
    attribute_name_index: u2,
    attribute_length: u4,
    max_stack: u2,
    max_locals: u2,
    code_length: u4,
    code: Vec<u1>,
    exception_table_length: u2,
    exception_table: Vec<ExceptionTableEntry>,
    attributes_count: u2,
    attributes: Vec<AttributeInfo>
}

pub struct ExceptionTableEntry {
    start_pc: u2,
    end_pc: u2,
    handle_pc: u2,
    catch_type: u2
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
    attribute_name_index: u2,
    attribute_length: u4,
    line_number_table_length: u2,
    line_number_table: Vec<LineNumberTableEntry>
}

pub struct LineNumberTableEntry {
    start_pc: u2,
    line_number: u2
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
    attribute_name_index: u2,
    attribute_length: u4,
    local_variable_table_length: u2,
    local_variable_table: Vec<LocalVariableTableEntry>
}

pub struct LocalVariableTableEntry {
    start_pc: u2,
    length: u2,
    name_index: u2,
    descriptor_index: u2,
    index: u2
}

/*
SourceFile_attribute {
    u2 attribute_name_index;
    u4 attribute_length;
    u2 sourcefile_index;
}
*/
pub struct SourceFileAttribute {
    attribute_name_index: u2,
    attribute_length: u4,
    source_file_index: u2
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
    attribute_name_index: u2,
    attribute_length: u4,
    num_annotations: u2,
    annotations: Vec<Annotation>
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
    type_index: u2,
    num_element_value_pairs: u2,
    element_value_pairs: Vec<AnnotationElementValuePair>
}

pub struct AnnotationElementValuePair {
    element_name_index: u2,
    value: ElementValue
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
    const_value_index: u2,
    enum_const_value: EnumConstValue,
    class_info_index: u2,
    annotation_value: Annotation,
    array_value: ElementValueArray
}

pub struct EnumConstValue {
    type_name_index: u2,
    const_name_index: u2
}

pub struct ElementValueArray {
    num_values: u2,
    values: Vec<Rc<ElementValue>>
}