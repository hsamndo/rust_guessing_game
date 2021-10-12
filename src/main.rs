use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the number!");
    
    // Generate a random number from the range of 1-(101-1)
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Infinite loop until break
    loop {
        println!("Please input your guess.");
           
        // guess is a mutabble empty string
        let mut guess = String::new();
        
        // Read input from user and put on guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Convert string to u32 type
        // if the user put a value that cannot be converted to that type, the Err launch a continue to the previous step
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
           
        // Match will check the structure that match with the cmp.cmp(secret_number)
        // if the match is less or greater it will print the msg on console and repea the loop
        // otherwise print "You Win!", break the loop and the program ends.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
