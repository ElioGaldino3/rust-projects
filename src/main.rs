use std::io;
use rand::Rng;
use rand;

use std::cmp::Ordering;

fn partida() {
    let numero: u16 = rand::thread_rng().gen_range(0..=100);
    println!("Tente adivinhar o número de 0 a 100, mas ele é {numero}");

    let mut jogo_ativo: bool = true;

    loop {
        
        let mut palpite: String = String::new();
        io::stdin().read_line(&mut palpite).expect("Digite algo ou o jogo fechará");
        let palpite: u16 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if !jogo_ativo {
            match palpite {
                1 => { partida(); break;},
                2 => menu_principal("Parabéns, espero que tenha se divertido!"),
                _ => println!("Você tem que escolhe uma opção válida")
            }
        }

        match palpite.cmp(&numero) {
            Ordering::Less => println!("O número sorteado é maior"),
            Ordering::Greater => println!("O número sorteado é menor"),
            Ordering::Equal => {
                println!("Parabéns, você acertou! 🙌🙌😎🎉🎉✨😃");
                println!("Envie 1 para jogar novamente ou 2 para ir ao menu principal");
                jogo_ativo = false;
            }
            
        }
    }
}

fn menu_principal(export: &str) {
    print!("\x1B[2J");
    println!("---------------- JOGO DA ADIVINHAÇÃO --------------------");
    println!("");
    println!("1 -> Iniciar nova partida\n2 -> Sair do jogo");
    println!("");
    println!("");
    println!("{export}");
    println!("");
    println!("Escolha sua opção:");

    let mut opcao: String = String::new();

    io::stdin().read_line(&mut opcao).expect("Digite algo ou o jogo fechará");

    let opcao: u8 = match opcao.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match opcao {
        1 => partida(),
        2 => (),
        _ => menu_principal("Escolha uma opção válida")
    }
}

fn main() {
    menu_principal("");
}
