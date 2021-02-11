use crate::types::ByteType::BYTE_TYPE;
use crate::types::{Type, ByteType, ShortType, CharType, IntType, LongType, BooleanType, DoubleType, FloatType};
use crate::types::ShortType::SHORT_TYPE;
use crate::types::CharType::CHAR_TYPE;
use crate::types::IntType::INT_TYPE;
use crate::types::LongType::LONG_TYPE;
use crate::types::BooleanType::BOOLEAN_TYPE;
use crate::types::DoubleType::DOUBLE_TYPE;
use crate::types::FloatType::FLOAT_TYPE;

pub trait Value {
    fn getType(&self) -> &Type;
}

pub type Byte = i8;

impl Value for Byte {
    fn getType(&self) -> &dyn Type {
        &BYTE_TYPE
    }
}

pub type Short = i16;

impl Value for Short {
    fn getType(&self) -> &dyn Type {
        &SHORT_TYPE
    }
}

pub type Char = u16;

impl Value for Char {
    fn getType(&self) -> &dyn Type {
        &CHAR_TYPE
    }
}

pub type Int = i32;

impl Value for Int {
    fn getType(&self) -> &dyn Type {
        &INT_TYPE
    }
}

pub type Long = i64;

impl Value for Long {
    fn getType(&self) -> &dyn Type {
        &LONG_TYPE
    }
}

pub type Float = f32;

impl Value for Float {
    fn getType(&self) -> &dyn Type {
        &FLOAT_TYPE
    }
}

pub type Double = f64;

impl Value for Double {
    fn getType(&self) -> &dyn Type {
        &DOUBLE_TYPE
    }
}

pub type Boolean = bool;

impl Value for Boolean {
    fn getType(&self) -> &dyn Type {
        &BOOLEAN_TYPE
    }
}