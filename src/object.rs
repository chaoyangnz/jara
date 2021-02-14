use std::rc::Rc;
use crate::values::Value;
use crate::types::{Type, Class};

pub struct Object {
    pub(crate) hash_code: i32,
    pub(crate) class: Rc<Class>,
    // TODO monitor
    pub(crate) slots: Vec<Box<Value>>
}


