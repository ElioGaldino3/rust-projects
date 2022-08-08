enum StatusDoQuarto {
    Ocupado,
    Livre,
    EmManutencao,
    Fechado,
    Abandonado,
}

use StatusDoQuarto::*;

fn main() {
    let meu_quarto = StatusDoQuarto::Ocupado;

    if let Ocupado = meu_quarto {
        println!("Está ocupado mesmo!");
    }

    match meu_quarto {
        StatusDoQuarto::Ocupado => println!("Quarto ocupado"),
        StatusDoQuarto::Livre => println!("Quarto livre"),
        StatusDoQuarto::EmManutencao => println!("Quarto em manutencao"),
        StatusDoQuarto::Fechado => println!("Quarto fechado"),
        StatusDoQuarto::Abandonado => println!("Quarto abandonado"),
    }
}
