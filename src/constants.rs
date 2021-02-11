pub const JVM_TAG_Class              : u8 = 7;
pub const JVM_TAG_Fieldref           : u8 = 9;
pub const JVM_TAG_Methodref          : u8 = 10;
pub const JVM_TAG_InterfaceMethodref : u8 = 11;
pub const JVM_TAG_String             : u8 = 8;
pub const JVM_TAG_Integer            : u8 = 3;
pub const JVM_TAG_Float              : u8 = 4;
pub const JVM_TAG_Long               : u8 = 5;
pub const JVM_TAG_Double             : u8 = 6;
pub const JVM_TAG_NameAndType        : u8 = 12;
pub const JVM_TAG_Utf8               : u8 = 1;
pub const JVM_TAG_MethodHandle       : u8 = 15;
pub const JVM_TAG_MethodType         : u8 = 16;
pub const JVM_TAG_InvokeDynamic      : u8 = 18;


pub const JVM_SIGNATURE_ARRAY    : &str =  "[";
pub const JVM_SIGNATURE_BYTE     : &str =  "B";
pub const JVM_SIGNATURE_CHAR     : &str =  "C";
pub const JVM_SIGNATURE_CLASS    : &str =  "L";
pub const JVM_SIGNATURE_ENDCLASS : &str =  ";";
pub const JVM_SIGNATURE_ENUM     : &str =  "E";
pub const JVM_SIGNATURE_FLOAT    : &str =  "F";
pub const JVM_SIGNATURE_DOUBLE   : &str =  "D";
pub const JVM_SIGNATURE_FUNC     : &str =  "(";
pub const JVM_SIGNATURE_ENDFUNC  : &str =  ")";
pub const JVM_SIGNATURE_INT      : &str =  "I";
pub const JVM_SIGNATURE_LONG     : &str =  "J";
pub const JVM_SIGNATURE_SHORT    : &str =  "S";
pub const JVM_SIGNATURE_VOID     : &str =  "V";
pub const JVM_SIGNATURE_BOOLEAN  : &str =  "Z";


pub type FieldAccessFlag = u16;
pub type MethodAccessFlag = u16;