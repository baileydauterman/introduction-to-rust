use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Secret: {secret_num}");

    loop
    {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_num)
        {
            Ordering::Less => println!("{guess} was too small"),
            Ordering::Greater => println!("{guess} was too large"),
            Ordering::Equal => 
            {
                println!("You got it!!!!");
                break;
            },
        }
    }
}
