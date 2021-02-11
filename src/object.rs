use std::rc::Rc;
use crate::class::Class;
use crate::value::Value;
use crate::types::Type;

pub struct Object {
    hashCode: i32,
    class: Rc<Class>,
    // TODO monitor
    slots: Vec<Box<Value>>
}

pub struct Reference {
    oop: Option<Rc<Object>>
}

impl Value for Reference {
    fn getType(&self) -> &dyn Type {
        unimplemented!()
    }
}

impl Reference {
    fn isNull(&self) -> bool {
        self.oop.is_none()
    }

    fn isArray(&self) -> bool {
        unimplemented!()
    }

    fn isEqual(&self, reference: &Reference) -> bool {
        if self.isNull() && reference.isNull() {
            return true
        }
        if !self.isNull() && !reference.isNull() {
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