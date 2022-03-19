use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret_number is {}", secret_number);

    let mut guess = String::new();
    loop {
        println!("Please input your guess ");
    io::stdin().read_line(&mut guess).expect("Error expected");
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) =>continue,
    };

    println!("You have guessed : {}", guess);
    

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("You've won");
                break;
            }
        }
    }
}
