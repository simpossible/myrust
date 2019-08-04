
use std::string::String;
pub struct Person {
   pub name : String,
    pub age: u8,
    pub  sex: u8,
}

pub fn des(person: &mut Person) -> String {
    person.name = "sd s".to_string();
    println!("person is {}", person.name);
    "".to_string()
}