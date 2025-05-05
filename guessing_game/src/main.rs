///////////////////////////////////////////////////////////////////////
//////////////// RUST program for guessign game ///////////////////////
/////////////// Rushikesh Shahaji Mhaske       ///////////////////////
/////////////////////////////////////////////////////////////////////

//creates 
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// main Function

fn main() {
    println!("Hello, world!, Welcome to Guessing Game!!!!");
    println!("Plese enter your gussed number !!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {secret_number}");
    loop {

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    let guess :i32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("Guessed Number is : {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Number is Low"),
        Ordering::Greater => println!("Number is Greater"),
        Ordering::Equal => {
            println!("You win");
            break;
        }
    }
    }
}


