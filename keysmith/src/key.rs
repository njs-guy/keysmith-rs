use crate::gen_char::gen_char;

struct GenCharOpts {
    nums: bool,
    letters: bool,
    upper: bool,
    safe_sp_ch: bool,
    unsafe_sp_ch: bool,
}

fn gen_char_from_opts(length: u32, opts: GenCharOpts) -> String {
    let mut output = String::from("");
    for _n in 1..=length {
        let c = gen_char(
            opts.nums, // nums
            opts.letters, // letters
            opts.upper, // upper
            opts.safe_sp_ch, // safe_sp_ch
            opts.unsafe_sp_ch // unsafe_sp_ch
        );

        output.push(c);
    }

    output // Return output as String
}

// Generates a key string. Ex: LlyqZk2W-Hm6Eoid~m(A8ymiM0q3ksyJ
pub fn gen_key(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: true, 
        letters: true, 
        upper: true, 
        safe_sp_ch: true, 
        unsafe_sp_ch: false 
    };

    gen_char_from_opts(length, opts)
}

pub fn gen_nums(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: true, 
        letters: false, 
        upper: false, 
        safe_sp_ch: false, 
        unsafe_sp_ch: false 
    };

    gen_char_from_opts(length, opts)
}

pub fn gen_letters(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: false, 
        letters: true, 
        upper: true, 
        safe_sp_ch: false, 
        unsafe_sp_ch: false 
    };

    gen_char_from_opts(length, opts)
}

pub fn gen_letters_lower(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: false,
        letters: true,
        upper: false,
        safe_sp_ch: false,
        unsafe_sp_ch: false
    };

    gen_char_from_opts(length, opts)
}

pub fn gen_letters_upper(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: false,
        letters: true,
        upper: false,
        safe_sp_ch: false,
        unsafe_sp_ch: false
    };

    gen_char_from_opts(length, opts).to_uppercase()
}

pub fn gen_nums_and_letters(length: u32) -> String {
    let opts = GenCharOpts {
        nums: true,
        letters: true,
        upper: true,
        safe_sp_ch: false,
        unsafe_sp_ch: false
    };

    gen_char_from_opts(length, opts)
}

pub fn gen_nums_and_letters_lower(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: true,
        letters: true,
        upper: false,
        safe_sp_ch: false,
        unsafe_sp_ch: false
    };

    gen_char_from_opts(length, opts)
}

pub fn gen_nums_and_letters_upper(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: true,
        letters: true,
        upper: false,
        safe_sp_ch: false,
        unsafe_sp_ch: false
    };

    gen_char_from_opts(length, opts).to_uppercase()
}

pub fn gen_special_chars(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: false, 
        letters: false, 
        upper: false, 
        safe_sp_ch: true, 
        unsafe_sp_ch: false 
    };

    gen_char_from_opts(length, opts)
}

pub fn gen_special_chars_unsafe(length: u32) -> String {
    let opts = GenCharOpts { 
        nums: false, 
        letters: false, 
        upper: false, 
        safe_sp_ch: false, 
        unsafe_sp_ch: true 
    };

    gen_char_from_opts(length, opts)
}