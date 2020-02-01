use rand::Rng;

struct Reading {
    digits: [&'static str; 10],
    ten: &'static str,
    hundred: &'static str,
}

// On readings from 0 to 9
const ON: Reading = Reading {
    digits: ["れい", "いち", "に", "さん", "し", "ご", "ろく", "しち", "はち", "きゅう"],
    ten: "じゅう",
    hundred: "ひゃく",
};

fn jap_numeral(number: u32) -> Result<String, ()> {
    let number = number as usize;
    let nums = ON;
    let mut string = String::new();
    if number >= 1000 { return Err(()); }
    if number >=  200 { string.push_str(nums.digits[number % 1000 / 100]); }
    if number >=  100 { string.push_str(nums.hundred); }
    if number >=   20 { string.push_str(nums.digits[number % 100 / 10]); }
    if number >=   10 { string.push_str(nums.ten); }
    if number >=    1 { string.push_str(nums.digits[number % 10]); }
    else              { string.push_str(nums.digits[0]) };
    Ok(string)
}

fn main() {
    let mut rng = rand::thread_rng();
    println!("Kon'nichiwa!");
    // TODO: choosable difficulty
    loop {
        let number = rng.gen_range(0, 100);
        println!("{}", jap_numeral(number).unwrap());

        'valid_anwser: loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: u32 = input.trim()
                .parse()
                .expect("Failed to parse input");

            if input == number {
                println!("Correct");
                break 'valid_anwser;
            } else {
                println!("Try again");
            }
        }
    }
}
