// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut guess_word: Vec<char> = vec!['_'; secret_word.len()];
    let mut guessed_char = String::new();
    let mut chances: u32 = NUM_INCORRECT_GUESSES;
    let mut correct = 0;

    loop {
        let guess_string: String = guess_word.clone().into_iter().collect();
        println!("The word so far is {}", guess_string);
        println!("You have guessed the following letters: {}", guessed_char);
        println!("You have {} guesses left", chances);
        print!("Please guess a letter: ");

        io::stdout()
            .flush()
            .expect("Error flushing stdout.");

        let mut guess_letter = String::new();

        io::stdin()
            .read_line(&mut guess_letter)
            .expect("Read fail!");

        
        guess_letter.truncate(1);

        let gchar = guess_letter.chars().next().expect("Out of Bound!");
        let mut flag = false;
        guessed_char.push(gchar);

        for index in 0..secret_word_chars.len() {
            if secret_word_chars[index] == gchar {
                println!("Good guess!");
                guess_word[index] = gchar;
                secret_word_chars[index] = '_';
                correct += 1;
                flag = true;
                break;
            }
        }

        if flag == false {
            println!("Error guess!");
            chances -= 1;
        }

        if chances == 0 {
            println!("Game over!");
            break;
        }

        if correct == secret_word.len() {
            println!("You win!");
            break;
        }
    }

}
