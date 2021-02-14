use crate::types::{Type};
use std::rc::Rc;
use crate::object::Object;

pub enum Value {
    Byte(i8),
    Short(i16),
    Char(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    Reference(Reference)
}

impl Value {
    fn get_type(&self) -> Rc<Type> {
        match self {
            Value::Byte(_) => Rc::from(Type::Byte),
            Value::Short(_) => Rc::from(Type::Short),
            Value::Char(_) => Rc::from(Type::Char),
            Value::Int(_) => Rc::from(Type::Int),
            Value::Long(_) => Rc::from(Type::Long),
            Value::Float(_) => Rc::from(Type::Float),
            Value::Double(_) => Rc::from(Type::Double),
            Value::Boolean(_) => Rc::from(Type::Boolean),
            Value::Reference(reference) => unimplemented!() //&(reference.oop.unwrap()).class,
        }
    }
}

pub struct Reference {
    oop: Option<Rc<Object>>
}

impl Reference {
    fn is_null(&self) -> bool {
        self.oop.is_none()
    }

    fn is_array(&self) -> bool {
        unimplemented!()
    }

    fn is_equal(&self, reference: &Reference) -> bool {
        if self.is_null() && reference.is_null() {
            return true
        }
        if !self.is_null() && !reference.is_null() {
            return Rc::ptr_eq(
                self.oop.as_ref().unwrap(),
                reference.oop.as_ref().unwrap());
        }
        false
    }
}

pub type ObjectReference = Reference;
pub type ArrayReference = Reference;


pub const NULL: &Reference = &Reference { oop: Option::None };

