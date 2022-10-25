use crate::gen_char::gen_uuid_char;

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