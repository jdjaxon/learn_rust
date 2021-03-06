use std::io;
use rand::Rng;
//use std::cmp::Ordering;

fn main() {
    println!("Guess the number!\n");

    // Generating the secret number.
    // NOTE: gen_range(start..end) is exclusive of the end number.
    //       Use gen_range(start..=end) to make it inclusive.
    let secret = rand::thread_rng().gen_range(1..101);

    loop
    {
        println!("Please enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error: failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < secret
        {
            println!("{} is too low!", guess);
        }
        else if guess > secret
        {
            println!("{} is too high!", guess);
        }
        else
        {
            println!("{} is correct! You win!", guess);
            break;
        }

        println!();

        // NOTE: another method for comparisons.
        //
        //match guess.cmp(&secret)
        //{
        //    Ordering::Less => println!("Too small!"),
        //    Ordering::Greater => println!("Too big!"),
        //    Ordering::Equal => {
        //        println!("You win!");
        //        break;
        //    }
        //}
    }
} /* main */
