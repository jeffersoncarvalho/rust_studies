fn main() {

    shadowing();
    //mutables();
    //constants();
    
}

fn shadowing() {

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("O valor de x no escopo interno é: {x}");
    }
    println!("O valor de x é: {x}");
    
}

fn mutables() {

    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x agora é: {}", x);

}

fn constants() {

    const PI_VALUE: f32 = 3.14;
    println!("O valor de PI é: {PI_VALUE}");

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("{}",THREE_HOURS_IN_SECONDS);
}