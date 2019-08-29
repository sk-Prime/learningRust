use std::io;
use rand::Rng;

fn main() {
    println!("Guessing game in Rust\n-------------------");
    let number = rand::thread_rng().gen_range(1, 10);
    let mut count = 5i8;
    let mut correct = false;
    while count > 0 && !correct {
        count -= 1;
        flow(number, &mut correct, count);
    }
    if count == 0 && !correct{
        println!("failed: the correct number is {}", number);
    }
}

fn input() -> i8 {
    let mut guessed = String::new();
    eprint!("Guess your number :");
    io::stdin()
        .read_line(&mut guessed)
        .expect("Error: Reading error");
    guessed.pop();
    guessed.pop();
    match guessed.parse::<i8>() {
        Ok(n) => return n,
        Err(_) => return -1,
    }
}

fn flow(number: i8, correct: &mut bool, count: i8) {
    match input() {
        n if n == -1 => println!("wrong input ({} chances left)", count),
        n if n > number => println!("{} is too big ({} chances left)", n, count),
        n if n < number => println!("{} is too small ({} chances left)", n, count),
        n if n == number => {
            println!("you guessed it correctly");
            *correct = true;
        }
        _ => println!("don't know what to do"),
    }
}
