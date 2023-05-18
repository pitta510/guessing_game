use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivnhe o numero!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);

    println!("O numero secreto: {numero_secreto}");

    loop {
        println!("Digite o seu paltpite:");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite
            .trim()
            .parse(){
                Ok(num) => (num),
                Err(_) => continue,
            };

        println!("voce disse: {palpite}");

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Voce acertou");
                break;
            }
        }
    }
}
