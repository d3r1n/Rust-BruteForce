use std::env;
use rand::Rng;
use std::thread;
use std::process::exit;

const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn crack(password: String) {
    let pass_list: Vec<char> = password.chars().collect();
    let chrst: Vec<char> = CHARSET.chars().collect();
    let mut rng = rand::thread_rng();
    let mut guesses: Vec<String> = Vec::new();
    let mut tries: i32 = 0;
    let _lenght: i32 = password.len() as i32;

    loop {
        if guesses.contains(&password.to_string()) {
            println!("Found Password in {:?} tries!\nPassword is: {:?}", &tries, &guesses[guesses.iter().position(|x| x == &password).unwrap()]);
            exit(0);
        }
        else {
            let mut guess: String = String::new();
            for _i in &pass_list {
                let index = rng.gen_range(0..CHARSET.len());
                guess = guess + &chrst[index].to_string();
            }
            if guesses.contains(&guess) {
                continue
            }
            else {
                tries += 1;
                println!("Guessed: {:?} | Tries: {:?}",&guess, &tries);
                guesses.push(guess);
            }
        }
    }
    

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let password = &args[1].to_owned();
    crack(password.to_string());
    println!("Possible Guesses: {:?}", i32::pow(CHARSET.len() as i32, args[1].len() as u32));
}