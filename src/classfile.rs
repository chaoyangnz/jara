use std::rc::Rc;
use std::convert::TryInto;

pub type u1 = u8;
pub type u2 = u16;
pub type u4 = u32;

pub struct ByteCodeVisitor {
    bytecode: Vec<u1>,
    pos: usize
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
}

pub struct ClassFile {
    magic: u4,
    minor_version: u2,
    major_version: u2,
    constant_pool_count: u2,
    constant_pool: Vec<Box<ConstantPoolInfo>>,
    access_flags: u2,
    this_class: u2,
    super_class: u2,
    interface_count: u2,
    interfaces: Vec<u2>,
    fields_count: u2,
    fields: Vec<FieldInfo>,
    method_count: u2,
    methods: Vec<MethodInfo>,
    attribute_count: u2,
    attributes: Vec<Box<AttributeInfo>>
}

impl ClassFile {
    pub fn new(bytecode: &mut ByteCodeVisitor) -> ClassFile {
        let magic = bytecode.u4();
        let minor_version = bytecode.u2();
        let major_version = bytecode.u2();
        let constant_pool_count = bytecode.u2();
        let mut constant_pool = Vec::<Box<ConstantPoolInfo>>::with_capacity(constant_pool_count as usize);
        let constant_tag = bytecode.u1();
        match constant_tag {
            JVM_TAG_Class=>
                constant_pool.push(Box::from(
                ConstantClassInfo { tag: constant_tag, name_index: bytecode.u2() }
                )),
            JVM_TAG_Fieldref =>
                constant_pool.push(Box::from(
                    ConstantFieldrefInfo {
                        tag: constant_tag,
                        class_index: bytecode.u2(),
                        name_and_type_index: bytecode.u2()
                    }
                )),
            // TODO handle constant pool
        }

        let access_flags = bytecode.u2();
        let this_class = bytecode.u2();
        let super_class = bytecode.u2();
        let interface_count = bytecode.u2();
        let interfaces = Vec::with_capacity(interface_count as usize);
        let fields_count = bytecode.u2();
        let fields = Vec::with_capacity(fields_count as usize);
        let method_count = bytecode.u2();
        let methods = Vec::with_capacity(method_count as usize);
        let attribute_count = bytecode.u2();
        let attributes = Vec::with_capacity(attribute_count as usize);
        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interface_count,
            interfaces,
            fields_count,
            fields,
            method_count,
            methods,
            attribute_count,
            attributes
        }
    }
}

pub trait ConstantPoolInfo {}

pub struct ConstantClassInfo {
    tag: u1,
    name_index: u2
}

impl ConstantPoolInfo for ConstantClassInfo {}

pub struct ConstantFieldrefInfo {
    tag: u1,
    class_index: u2,
    name_and_type_index: u2
}

impl ConstantPoolInfo for ConstantFieldrefInfo {}

pub struct ConstantMethodrefInfo {
    tag: u1,
    class_index: u2,
    name_and_type_index: u2
}

impl ConstantPoolInfo for ConstantMethodrefInfo {}

pub struct ConstantInterfaceMethodrefInfo  {
    tag              : u1,
    class_index       : u2,
    name_and_type_index : u2
}

impl ConstantPoolInfo for ConstantInterfaceMethodrefInfo {}

pub struct ConstantStringInfo {
    tag: u1,
    string_index: u2
}

impl ConstantPoolInfo for ConstantStringInfo {}

pub struct ConstantIntegerInfo {
    tag: u1,
    bytes: u4
}

impl ConstantPoolInfo for ConstantIntegerInfo {}

pub struct ConstantFloatInfo {
    tag: u1,
    bytes: u4
}

impl ConstantPoolInfo for ConstantFloatInfo {}

pub struct ConstantLongInfo {
    tag: u1,
    high_bytes: u4,
    low_bytes: u4
}

impl ConstantPoolInfo for ConstantLongInfo {}

pub struct ConstantDoubleInfo{
    tag: u1,
    high_bytes: u4,
    low_bytes: u4
}

impl ConstantPoolInfo for ConstantDoubleInfo {}

pub struct ConstantNameAndTypeInfo {
    tag: u1,
    name_index: u2,
    descriptor_index: u2
}

impl ConstantPoolInfo for ConstantNameAndTypeInfo {}

pub struct ConstantUtf8Info {
    tag: u1,
    length: u2,
    bytes: Vec<u1>
}

impl ConstantPoolInfo for ConstantUtf8Info {}

pub struct ConstantMethodHandleInfo {
    tag: u1,
    reference_kind: u1,
    reference_index: u2
}

impl ConstantPoolInfo for ConstantMethodHandleInfo {}

pub struct ConstantMethodTypeInfo {
    tag: u1,
    descriptor_index: u2
}

impl ConstantPoolInfo for ConstantMethodTypeInfo {}

pub struct ConstantInvokeDynamicInfo {
    tag: u1,
    bootstrap_method_attr_index: u2,
    name_and_type_index: u2
}

impl ConstantPoolInfo for ConstantInvokeDynamicInfo {}

pub struct FieldInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attribute_count: u2,
    attributes: Vec<Box<AttributeInfo>>
}

pub struct MethodInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attribute_count: u2,
    attributes: Vec<Box<AttributeInfo>>
}

pub trait AttributeInfo {}

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
    attributes: Vec<Box<AttributeInfo>>
}

impl AttributeInfo for CodeAttribute {}

pub struct ExceptionTableEntry {
    start_pc: u2,
    end_pc: u2,
    handle_pc: u2,
    catch_type: u2
}

pub struct LineNumberTableAttribute {
    attribute_name_index: u2,
    attribute_length: u4,
    line_number_table_length: u2,
    line_number_table: Vec<LineNumberTableEntry>
}

impl AttributeInfo for LineNumberTableAttribute {}

pub struct LineNumberTableEntry {
    start_pc: u2,
    line_number: u2
}

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

impl AttributeInfo for LocalVariableTableAttribute {}

pub struct SourceFileAttribute {
    attribute_name_index: u2,
    attribute_length: u4,
    source_file_index: u2
}

impl AttributeInfo for SourceFileAttribute {}

pub struct RuntimeVisibleAnnotationAttribute {
    attribute_name_index: u2,
    attribute_length: u4,
    num_annotations: u2,
    annotations: Vec<Annotation>
}

impl AttributeInfo for RuntimeVisibleAnnotationAttribute {}

pub struct Annotation {
    type_index: u2,
    num_element_value_pairs: u2,
    element_value_pairs: Vec<AnnotationElementValuePair>
}

pub struct AnnotationElementValuePair {
    element_name_index: u2,
    value: ElementValue
}

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