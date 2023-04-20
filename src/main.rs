use std::io::stdin;
use rand::Rng;

fn main() {
    start_game();
    let mut turns = 0;
    let target_number = generate_number_between(1, 10);
    let mut user_guess = String::new();
    let mut game_over = false;

    while turns < 10 && game_over != true {
        if turns == 0 {
            println!("Please Enter a number between 1 and 10");
        } else if turns != 0 && game_over == false {
            println!("That was incorrect.Please try again!")
        }

        stdin().read_line(&mut user_guess).expect("Failed to read user input");

        turns += 1;

        if user_guess.trim() == target_number.to_string() {
            game_over = true;
        }
        // Add the end of each cycle reset user_guess as readline appends to existing var
        reset_user_guess(&mut user_guess)
    }
    if game_over {
        println!("I don't know how you did it but you are right the number is {} and you did it in only {}, turn(s)", target_number, turns);
    } else {
        println!("I'm sorry the number was {}, not {}. Game Over in {} turn(s)!", target_number, user_guess, turns)
    }
}


fn reset_user_guess(user_guess_ref: &mut String) {
    user_guess_ref.clear();
}

fn generate_number_between(starting_number: i32, ending_number: i32) -> i32 {
    rand::thread_rng().gen_range(starting_number..ending_number)
}

fn start_game() {
    println!("Welcome to the Rusty Number Guessing Game");
}
