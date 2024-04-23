use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinha o número!");

    let segredo = rand::thread_rng().gen_range(1..=100);
    //println!("Número secreto: {}", segredo);
    
    loop {
        println!("Entre com um número: ");
        let mut numero = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao ler a linha!");
        //let numero: u32 = numero.trim().parse().expect("ERRO! Entre com um número!");
        let numero: u32 = match numero.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //println!("Você digitou: {}",numero);

        match numero.cmp(&segredo) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
