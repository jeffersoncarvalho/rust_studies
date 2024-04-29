use std::io;

fn main() {
    /* 
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("SOMA: {}", sum);
    println!("DIFERENÇA: {}", difference);
    println!("PRODUTO: {}", product);
    println!("QUOCIENTE: {}", quotient);
    println!("TRUNCADO: {}", truncated);
    println!("MÓDULO: {}", remainder);

    bool_type();
    character_type();
    tuple_type();
    array_type();
    */
    array_select();
}

fn bool_type() {

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t: {}, f: {}",t,f);
}

fn character_type() {

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c: {}",c);
    println!("z: {}",z);
    println!("heart_eyed_cat: {}",heart_eyed_cat);

}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("tup: {}, {}, {}", x, y, z);
    println!("tup: {}, {}, {}", tup.0, tup.1, tup.2);
}

fn array_type() {

    //let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //type and length
    println!("{}",a[2]);
    //let a = [3; 5];
    //same as 
    //let a = [3, 3, 3, 3, 3];
}

fn array_select() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}