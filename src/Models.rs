
use std::string::String;
use crate::IDGeter;

pub trait CommonDes {
    fn des(&self) -> String;
}

pub struct Person {
   pub name : String,
    pub age: u8,
    pub  sex: u8,
}

impl CommonDes for Person {
     fn des(&self) -> String {
        let f = format!("这个人名字是:{} 年龄是:{} 性别是:{}",self.name,self.age,self.sex);
        f
    }
}



pub fn NewPerson(withName:String,withAge:u8,withSex:u8) -> Person {
    let p :Person = Person{name:withName,age:withAge,sex:withSex};
    p
}


pub struct DevTeam {
    tealId:u32,
    name:String,
    des:String,
    users:Vec<Person>,
}

impl DevTeam {
    pub fn addPerson(&mut self,person:Person) {
        self.users.push(person);
    }
    pub fn allPersons(&self) {
        for iter in &self.users {
            let d = iter.des();
            print!("{}\n",d);
        }
    }
}

pub fn NewTeamWith(name:String,des:String) -> DevTeam {
    let curID = IDGeter::GetNextId();
    let users:Vec<Person> = Vec::new();
    let team = DevTeam{ tealId: curID,name,des,users};
    team
}


