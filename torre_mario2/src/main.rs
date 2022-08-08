fn main() {
    let tamanho: u8 = get_tamanho("Digite o tamanho: ");

    for x in 0..tamanho {
        for y in 0..tamanho {
            if y + x + 1 < tamanho {
                print!(" ");
            } else {
                print!("#");
            }
        }
        print!(" ");

        for z in (0..tamanho).rev() {
            if x + z + 1 < tamanho {
                print!(" ");
            } else {
                print!("#");
            }
        }

        print!("\n");
    }
    print!("\n");
}

fn get_tamanho(msg: &str) -> u8 {
    let mut result = 0;
    loop {
        let mut input = String::new();
        println!("\n{msg}");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u8>() {
                Ok(int) => match int {
                    1..=8 => {
                        result += int;
                        break;
                    }
                    _ => continue,
                },
                Err(erro) => {
                    println!("Isso não é um número");
                    println!("Erro: {erro}");
                    continue;
                }
            },
            Err(erro) => {
                println!("Não conseguir indentificar o que digitou");
                println!("Erro: {erro}");
                continue;
            }
        }
    }

    result
}
