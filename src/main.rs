use std::borrow::Borrow;

mod types;
mod constants;
mod class;
mod value;
mod object;
mod class_file;

fn main() {
    let class = class_file::read("/Users/chao.yang/Work/datetime/out/production/classes/HelloWorld.class");
    println!("{}", "ok");
}
