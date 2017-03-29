extern crate rand;

use rand::Rng;
use std::collections::HashSet;

const LIMIT: usize = 100;

// On readings from 0 to 10 and 100
const ON: [&'static str; 12] =
["れい", "いち", "に", "さん", "し", "ご", "ろく", "しち", "はち", "きゅう", "じゅう", "ひゃく"];

fn jap_numeral(number: u16) -> String {
    let number = number as usize;
    let nums = ON;
    let mut string = String::new();
    if number >= 1000 { return "Out of range".to_owned(); }
    if number >=  200 { string.push_str(nums[number % 1000 / 100]); }
    if number >=  100 { string.push_str(nums[11]); }
    if number >=   20 { string.push_str(nums[number % 100 / 10]); }
    if number >=   10 { string.push_str(nums[10]); }
    if number !=    0 { string.push_str(nums[number % 10]); }
    else              { string.push_str(nums[0]) };
    string
}

fn main() {
    let mut counter = HashSet::with_capacity(LIMIT);
    let mut rng = rand::thread_rng();
    println!("Kon'nichiwa!");
    for _ in 0..LIMIT {
        let mut number;
        loop {
            number = rng.gen_range::<u16>(0, 100);
            if !counter.contains(&number) { break; }
        }
        println!("{}", jap_numeral(number));
        // println!("Debug: {}; {:?}", number, counter);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim()
            .parse::<u16>()
            .expect("Failed to parse input");
        if input == number {
            counter.insert(number);
            println!("Correct");
        } else {
            println!("Try again");
        }
    }
    println!("Congratulation");
}
