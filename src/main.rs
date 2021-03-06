use std::borrow::Borrow;
use crate::types::Class;

mod types;
mod constants;
mod values;
mod object;
mod class_file;

fn main() {
    let classfile = class_file::read("./java/HelloWorld.class");
    let class = Class::from(&classfile);
    println!("{}", "ok");
}
