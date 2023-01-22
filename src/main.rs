// cargo run, compiles and runs the code
// cargo check will check the project for warnings or errors
// rustfmt formats code

use rand::Rng;
use std::cmp::Ordering;
use std::io; // Found in dependencies in Toml file

fn main() {
    // formatted_print();
    // variables();
    guessing_game();
}

fn formatted_print() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("Base 10:               {}", 69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C
    println!("{number:>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}

fn variables() {
    let x: i32 = 1; // Explicitly specified type, Immutable
    let y = 2; // Implicit specified type, Immutable
    println!("x is {0} y is {1}", x, y);

    let mut xMut = 1;
    xMut = 10; // xMut is mutable due to mut
    println!("xMut is {}", xMut)
}

fn guessing_game() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess) // &, indicates the argument is a reference and can be accessed multiple places
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
