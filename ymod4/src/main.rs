mod base;

fn main() {
    base::temp();
    println!("usando modulo test!");
    println!("agora o modulo test!");
    println!("esta dentro da pasta test!");
    println!("renomeado para mod.rs!");
    println!("mudei a pasta para base!");
    println!("alterei main para mod base!");

    base::variavel::executar();    
    base::noone::execone();
    base::rust4mod::execrust4();
}
