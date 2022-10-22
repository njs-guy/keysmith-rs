mod test_key_gen;
mod test_uuid;
mod test_readme;

fn main() {
    // The length of each key
    let num_of_keys = 10;

    // test readme code
    let readme_code = true;

    // which gen_key functions to test
    let gen_key_opts = test_key_gen::TestKeyOpts {
        keys: true,
        nums: true,
        letters: true,
        letters_lower: true,
        letters_upper: true,
        nums_and_letters: true,
        nums_and_letters_lower: true,
        nums_and_letters_upper: true,
        special_chars: true,
        special_chars_unsafe: true
    };

    test_key_gen::test_key_gen(num_of_keys, gen_key_opts);

    // which uuid functions to test
    let uuid_opts = test_uuid::TestUuidOpts {
        uuid4: true,
        uuidn: true
    };

    test_uuid::test_uuid(num_of_keys, uuid_opts);

    if readme_code {
        test_readme::readme_code1();
        println!("");
        test_readme::readme_code2();
        println!("");
        test_readme::readme_code3();
        println!("");
        test_readme::readme_code4();
    }
}