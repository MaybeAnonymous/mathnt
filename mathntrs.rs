// WORK IN PROGRESS!!!
// made my MaybeAnonymous

// use
use std::fmt;

use std::io;
use std::io::stdout;
use std::io::Write;

// main function
fn main() {
    let mut n1: String = String::new();
    let mut n2: String = String::new();
    let mut op: String = String::new();

    // FIRST NUMBER
    print!("Input first number: ");
    stdout().flush().expect("Error");

    // read / write
    io::stdin()
        .read_line(&mut n1)
        .expect("Cannot get first number");
    
        // SECOND NUMBER
    print!("Input second number: ");
    stdout().flush().expect("Error");
    
    // read / write
    io::stdin()
        .read_line(&mut n2)
        .expect("Cannot get second number");
    
        // OPERATOR
    print!("Operator: ");
    stdout().flush().expect("Error");

    // read / write
    io::stdin()
        .read_line(&mut op)
        .expect("Cannot get operator");
    
        // END OF GETTING NUMBERS AND OPERATORS
    n1.pop(); n2.pop(); //op.pop();
    let n1i = n1.parse::<i32>().unwrap();
    let n2i = n2.parse::<i32>().unwrap();
    
    let opstr: &str = &op;

    let n1n2 = format!("{}{}", n1, n2);

    match opstr {
        "+\n" | "p\n" => println!("{}", n1n2),
        "-\n" | "m\n" => println!("{}", (n1i - n2i) * -1 ),
        _ => println!("Invalid operator!"),
    }

}
