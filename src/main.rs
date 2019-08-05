use std::string::String;
use devsys::models::*;
use std::rc::Rc;

fn main() {
    let p = Person{name:"asd".to_string(),age:1,sex:2};
//    let p = RC::Person{name:"asd".to_string(),age:1,sex:2};
    let mut team = NewTeamWith("ios".to_string(),"人".to_string());
    let p2 = NewPerson("吴荣华".to_string(),2,1);
    team.addPerson(p2);
    team.allPersons();

    let dd = p.des();


    println!("Hello, world!");

}
