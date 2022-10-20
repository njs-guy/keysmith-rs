mod gen_char;
use gen_char::{gen_char, gen_uuid_char};

// Generates a key string. Ex: LlyqZk2W-Hm6Eoid~m(A8ymiM0q3ksyJ
pub fn gen_key(length: u32) -> String {
    let mut output = String::from("");
    for _n in 1..=length {
        let c = gen_char(
            true, // nums
            true, // letters
            true, // upper
            true, // safe_sp_ch
            false // unsafe_sp_ch
        );

        output.push(c);
    }

    output // Return output as String
}

pub fn gen_nums(length: u32) -> String {
    let mut output = String::from("");
    for _n in 1..=length {
        let c = gen_char(
            true, // nums
            false, // letters
            false, // upper
            false, // safe_sp_ch
            false // unsafe_sp_ch
        );

        output.push(c);
    }

    output // Return output as String
}

pub fn gen_letters(length: u32) -> String {
    let mut output = String::from("");
    for _n in 1..=length {
        let c = gen_char(
            false, // nums
            true, // letters
            true, // upper
            false, // safe_sp_ch
            false // unsafe_sp_ch
        );

        output.push(c);
    }

    output // Return output as String
}

// Generate a uuid. Ex: fc402d52-70be-7f09-caed-8da65db08985
// Version 'n' is nonstandard which is
// not necessarily recommended but could be useful.
pub fn gen_uuid(version: char) -> String {
    let mut output = String::from("");
    let mut c: char;

    for _n in 1..=8 {
        c = gen_uuid_char(version);

        output.push(c);
    }

    output.push('-'); // Current state: fc402d52-

    for _n in 1..=3 {
        for _x in 1..=4 {
            c = gen_uuid_char(version);
            output.push(c);
        }

        output.push('-');
        // After one loop: efc402d52-70be-
    }

    // Current state: fc402d52-70be-7f09-caed-

    for _n in 1..=12 {
        c = gen_uuid_char(version);
        output.push(c);
    }

    output // fc402d52-70be-7f09-caed-8da65db08985
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests that the generated key is the correct length
    #[test]
    fn test_length() {
        let length = 32;
        let key = gen_key(length);

        assert_eq!(key.len(), 32);
    }
}
