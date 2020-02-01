use rand::Rng;

struct Numerals {
    digits: [&'static str; 10],
    ten: &'static str,
    hundred: &'static str,
    thousand: &'static str,
}

// Japanese numerals
const NUMERALS: Numerals = Numerals {
    digits: ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"],
    ten: "十",
    hundred: "百",
    thousand: "千",
};

// On reading
const ON_READING: Numerals = Numerals {
    digits: ["れい", "いち", "に", "さん", "し", "ご", "ろく", "しち", "はち", "きゅう"],
    ten: "じゅう",
    hundred: "ひゃく",
    thousand: "せん",
};

fn jap_numeral(number: u32, numerals: Numerals) -> Result<String, ()> {
    let number = number as usize;
    let mut string = String::new();
    if number >= 1000 { return Err(()); }
    if number >=  200 { string.push_str(numerals.digits[number % 1000 / 100]); }
    if number >=  100 { string.push_str(numerals.hundred); }
    if number >=   20 { string.push_str(numerals.digits[number % 100 / 10]); }
    if number >=   10 { string.push_str(numerals.ten); }
    if number !=    0 { string.push_str(numerals.digits[number % 10]); }
    else              { string.push_str(numerals.digits[0]) };
    Ok(string)
}

fn main() {
    let mut rng = rand::thread_rng();
    println!("Kon'nichiwa!");
    // TODO: add menu, choosable difficulty
    loop {
        let number = rng.gen_range(0, 1000);
        println!("{}", jap_numeral(number, NUMERALS).unwrap());

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
