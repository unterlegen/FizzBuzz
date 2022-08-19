use std::io;

fn main() {
    let mut number = String::new();
    
    io::stdin() .read_line(&mut number) .expect("Failed to read line");
    let number: i128 = number.trim().parse().ok().expect("Input is not an integer");

    if number % 3 == 0 && number % 5 == 0{
        println!("FizzBuzz")
    }
    else if number % 3 == 0{
        println!("Fizz");
    }
    else if number % 5 == 0{
        println!("Buzz");
    }
    else {
        println!("Your number is for Bozos!");
    }
}