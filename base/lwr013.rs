// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//struct 
//trait
//impl                                                                                                    -->
////////////////////////////////////////
struct Person {
    name: String,
    age: u8,
    cel: String,
    dia: u8,
    mes: u8,
    ano: u32,
}

trait HVBox {
    // speak
    fn speak(&self);

    // check is can speak
    fn can_speak(&self) -> bool;
}

impl HVBox for Person {

         fn speak(&self) {
             println!("Ola, meu nome : {}", self.name);
         }
         
         fn can_speak(&self) -> bool {
            if self.age > 0 {
                return true;
            }   return false;
             
         }
     }

pub fn lwr13 () {

    let person = Person {
        name: String::from("Deg"),
        age : 75,
        cel : String::from("99826-5775"),
        dia : 11,
        mes : 04,
        ano : 1945,

    };
        println!(" {} pode falar? {}", person.name, person.can_speak());
        println!(" sua idade hoje : {}",person.age);
        println!(" celular : {}",person.cel);
        println!(" nasc.dia {} ",person.dia);
        println!(" nasc.mes {} ",person.mes);
        println!(" nasc.ano {} ",person.ano);
}
