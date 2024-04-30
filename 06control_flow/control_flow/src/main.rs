use std::io;

fn main() {
    //println!("Hello, world!");
    calcular_situacao_aluno(7.0, 5.5);
}

fn calcular_situacao_aluno(np1: f32, np2: f32) {
    let media: f32 = (np1 + np2) / 2.0;
    println!("MEDIA: {}", media);
    if media <= 4.0 {
        println!("Reprovado por Média!");
    } else if media >= 7.0 {
        println!("Aprovado com Conceito A");
    } else {
        println!("Aluno de Final!");
        let nf = ler_numero();
        if nf < 4.0 {
            println!("Reprovado na Prova Final")
        } else {
            let media_final: f32 = (media + nf) / 2.0; 
            let situacao = if media_final < 5.0 { "Reprovado na Média Final" } else { "Aprovado com Conceito B" };
            println!("{}",situacao);
        }
    }
}

fn ler_numero() -> f32 {

    let mut nota_final_str = String::new();
    let mut nota_final: f32 = 0.0;

    loop {
        println!("Entre com a nota final: ");

        io::stdin()
            .read_line(&mut nota_final_str)
            .expect("Falha ao ler a linha!");

        nota_final = match nota_final_str.trim().parse() {
            Ok(nf) => nf,
            Err(_) => continue,
        };

        return nota_final;
    }    
}
