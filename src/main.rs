use std::io::stdin;
use rand::Rng;

fn main() {
    start_game();

    let mut user_turns = 1;
    let mut user_guess = String::new();
    let mut game_won = false;

    let target_number = generate_number_between(1, 10);

    while user_turns <= 10 && game_won != true {
        if user_turns == 1 {
            println!("Please Enter a number between 1 and 10");
        } else if user_turns != 1 && game_won == false {
            println!("That was incorrect.Please try again!")
        }

        get_user_guess(&mut user_guess);

        if user_guess.trim() == target_number.to_string() {
            game_won = true;
            break;
        }
        // At the end of each cycle reset user_guess as readline appends to existing var
        reset_user_guess(&mut user_guess);

        user_turns += 1;
    }

    if game_won && user_turns == 1 {
        println!("I don't know how you did it but you are right the number is {} and you did it in only {}, turn", target_number, user_turns);
    } else if game_won {
        println!("You guessed {} the right number in {} turns, Congratulations!", target_number, user_turns);
    } else {
        println!("I'm sorry you did not guess the number in {} turn(s) Game Over!", user_turns)
    }
}

fn get_user_guess(mut user_guess: &mut String) -> usize {
    stdin().read_line(&mut user_guess).expect("Failed to read user input")
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
