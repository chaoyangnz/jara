use crate::constants::{JVM_SIGNATURE_BYTE, JVM_SIGNATURE_SHORT, JVM_SIGNATURE_CHAR, JVM_SIGNATURE_INT, JVM_SIGNATURE_LONG, JVM_SIGNATURE_FLOAT, JVM_SIGNATURE_DOUBLE, JVM_SIGNATURE_BOOLEAN};

pub trait Type {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
}

pub enum ByteType {
    BYTE_TYPE
}

impl Type for ByteType {
    fn name(&self) -> String {
        JVM_SIGNATURE_BYTE.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_BYTE.to_string()
    }
}

pub enum ShortType {
    SHORT_TYPE
}

impl Type for ShortType {
    fn name(&self) -> String {
        JVM_SIGNATURE_SHORT.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_SHORT.to_string()
    }
}

pub enum CharType {
    CHAR_TYPE
}

impl Type for CharType {
    fn name(&self) -> String {
        JVM_SIGNATURE_CHAR.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_CHAR.to_string()
    }
}

pub enum IntType {
    INT_TYPE
}

impl Type for IntType {
    fn name(&self) -> String {
        JVM_SIGNATURE_INT.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_INT.to_string()
    }
}

pub enum LongType {
    LONG_TYPE
}

impl Type for LongType {
    fn name(&self) -> String {
        JVM_SIGNATURE_LONG.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_LONG.to_string()
    }
}

pub enum FloatType {
    FLOAT_TYPE
}

impl Type for FloatType {
    fn name(&self) -> String {
        JVM_SIGNATURE_FLOAT.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_FLOAT.to_string()
    }
}

pub enum DoubleType {
    DOUBLE_TYPE
}

impl Type for DoubleType {
    fn name(&self) -> String {
        JVM_SIGNATURE_DOUBLE.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_DOUBLE.to_string()
    }
}

pub enum BooleanType {
    BOOLEAN_TYPE
}

impl Type for BooleanType {
    fn name(&self) -> String {
        JVM_SIGNATURE_BOOLEAN.to_string()
    }

    fn descriptor(&self) -> String {
        JVM_SIGNATURE_BOOLEAN.to_string()
    }
}