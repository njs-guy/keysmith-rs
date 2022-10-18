mod possible_chars;
use possible_chars::{get_poss_chars, get_uuid_chars};

use rand::Rng;

// Gets a character set from possible_chars
// and then returns that for use in String.push_str()
fn push_poss_chars(char_set_name: &str) -> &str {
    let possible_chars = get_poss_chars();
    let expect_msg = format!("Could not convert {} in possible_chars Hashmap.", char_set_name);
    
    return possible_chars.get(char_set_name).expect(&expect_msg);
}

// Generates a char for a key
pub fn gen_char(
    nums: bool, 
    letters:bool, 
    upper: bool, 
    safe_sp_ch: bool, 
    unsafe_sp_ch: bool,
) -> char {
    let mut chars = String::from("");

    // Set allowed characters
    if nums {
        chars.push_str(push_poss_chars("numbers"));
    }

    if letters {
        chars.push_str(push_poss_chars("en_alphabet"));
        
        if upper {
            chars.push_str(&push_poss_chars("en_alphabet").to_uppercase());
        }
    }

    if safe_sp_ch {
        chars.push_str(push_poss_chars("safe_sp_chars"));
    }

    if unsafe_sp_ch {
        chars.push_str(
            push_poss_chars("unsafe_sp_chars"));
    }

    let mut rng = rand::thread_rng();

    // get a rand index from chars
    let idx = rng.gen_range(0..chars.len());

    // get the value of the index
    let c = chars.chars().nth(idx).expect("Could not get value of char.");

    c // Return output as char
}

pub fn gen_uuid_char(version: char) -> char {
    let c: char;

    match version {
        '4' => c = gen_uuid_v4_char(),
        'n' => c = gen_uuid_nonstandard_char(),
        _ => c = '0',
    }

    c // return c as a char
}

// Generate numbers and letters (no uppercase)
fn gen_uuid_nonstandard_char() -> char {
    gen_char(true, true, false, false, false)
}

fn gen_uuid_v4_char() -> char {
    let mut chars = String::from("");
    let uuid_chars = get_uuid_chars();
    let expect_msg = format!("Could not convert uuid chars in Hashmap.");

    chars.push_str(uuid_chars.get("numbers").expect(&expect_msg));
    chars.push_str(uuid_chars.get("letters").expect(&expect_msg));

    let mut rng = rand::thread_rng();

    // Get a rand index from chars
    let idx = rng.gen_range(0..chars.len());

    // Get value of the index
    let c = chars.chars().nth(idx).expect("Could not get the value of the char.");

    c // return output as char
}