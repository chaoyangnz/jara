use std::rc::Rc;
use crate::class::Class;
use crate::value::Value;
use crate::types::Type;

pub struct Object {
    hash_code: i32,
    class: Rc<Class>,
    // TODO monitor
    slots: Vec<Box<Value>>
}

pub struct Reference {
    oop: Option<Rc<Object>>
}

impl Value for Reference {
    fn get_type(&self) -> &dyn Type {
        unimplemented!()
    }
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