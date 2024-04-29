
fn main() {
    println!("Hello, world!");
    soma(2,3);   
    println!("produto: {}", mult(5,3)); 
}

fn soma(x: i32, y: i32) {
    
    let resultado = {
        let soma = 0;
        soma + x + y
    };

    println!("soma: {resultado}");
}

fn mult(x: i32, y: i32) -> i32 {
    x * y 
}