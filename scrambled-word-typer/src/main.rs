use std::{io};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::time::{Duration, Instant};



fn scramble_word(a_word: &str) -> String {
    let mut chars: Vec<char> = a_word.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}


fn main() {
    let words_array = ["batman", "hippopotamus", "falcon"];
    println!("Welcome to the speed typing competition. To begin this game, type 'start' and press the enter key, and the timer will start. Good luck");
    
    let mut start_word = String::new();
    io::stdin()
    .read_line(&mut start_word)
    .expect("error when getting word");

    if start_word.trim() == "start" {
        let start = Instant::now();
        for word in words_array {
            let scrambled = scramble_word(word);
            println!("Your word is: {}", scrambled);
            let mut attemp_word = String::new();
            
            while attemp_word.trim() != scrambled {
                attemp_word.clear();
                io::stdin()
            .read_line(&mut attemp_word)
            .expect("did not get word");
            }
            
        }
        let elapsed = start.elapsed();
        println!("You have finished the challenge. It took you: {:.2?} seconds.", elapsed);
        if elapsed <= Duration::from_secs(3) {
            println!("Excellent job, that was fast. Gold medal!");
        } else if elapsed <= Duration::from_secs(5) {
            println!("Good work, that was pretty fast. Silver medal!");
        } else if elapsed <= Duration::from_secs(8){
            println!("Nice, that was okay. Bronze medal!")
        } else {
            println!("Try to get fast next time.")
        }
    }

}

