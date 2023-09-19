mod quina;
mod lotofacil;
mod megasena;
mod lotomania;
mod timemania;
mod duplasena;

fn main() {

    println!();
    println!("Dezenas aleatorias da quina!");
    quina::quina_fn();
    println!("----------------------------------");
    println!("Dezenas aleatorias da lotofacil!");
    lotofacil::lotofacil_fn();
    println!("----------------------------------");
    println!("Dezenas aleatorias da megasena!");
    megasena::megasena_fn();
    println!("----------------------------------");
    println!("Dezenas aleatorias da lotomania!");
    lotomania::lotomania_fn();
    println!("----------------------------------");
    println!("Dezenas aleatorias da timemania!");
    timemania::timemania_fn();
    println!("----------------------------------");
    println!("Dezenas aleatorias da duplasena!");
    duplasena::duplasena_fn();
}

