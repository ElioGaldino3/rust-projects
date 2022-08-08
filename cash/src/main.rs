fn main() {
    let money: f64 = get_cash("Digite o valor do troco");
    let mut money_in_cents: u32 = (money * 100.0).round().to_string().parse().unwrap();

    println!("Isso são {money_in_cents} centavos!");

    let mut moedas: u16 = 0;

    loop {
        match money_in_cents {
            c if c >= 25 => {
                moedas += 1;
                money_in_cents -= 25;
            }
            c if c >= 10 => {
                moedas += 1;
                money_in_cents -= 10;
            }
            c if c >= 5 => {
                moedas += 1;
                money_in_cents -= 5;
            }
            c if c >= 1 => {
                moedas += 1;
                money_in_cents -= 1;
            }
            _ => break,
        }
    }

    println!("{moedas}");
}

fn get_cash(msg: &str) -> f64 {
    let mut result: f64 = 0.0;
    loop {
        let mut input = String::new();
        println!("\n{msg}");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<f64>() {
                Ok(float) if float <= 0.0 => continue,
                Ok(float) => {
                    result += float;
                    break;
                }
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
