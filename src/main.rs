use std::string::String;
use devsys::models::*;

fn main() {
    let mut p = Person{name:"asd".to_string(),age:1,sex:2};
    let mute = &mut p;
    des(mute);
    println!("Hello, world!");
}
