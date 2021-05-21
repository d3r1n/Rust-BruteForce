use std::env;
use rand::Rng;
use std::time::Instant;

const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn crack(password: String) {
    let now = Instant::now();
    let pass_list: Vec<char> = password.chars().collect();
    let chrst: Vec<char> = CHARSET.chars().collect();
    let mut rng = rand::thread_rng();
    let mut tries: i32 = 0;
    let _lenght: i32 = password.len() as i32;

    loop {

        let mut guess: String = String::new();
        for _i in &pass_list {
            let index = rng.gen_range(0..CHARSET.len());
            guess = guess + &chrst[index].to_string();
            tries += 1;
        }
        if guess == password {
            println!("Guessed: {:?} | Tries: {:?} | Elapsed Time: {:?}",&guess, &tries, now.elapsed().as_secs());
            println!("Possible Guesses: {:?}", i32::pow(CHARSET.len() as i32, _lenght as u32));
            break;
        }
    }
}

fn main() {


    let args: Vec<String> = env::args().collect();
    let password = &args[1].to_owned();
    crack(password.to_string());

}