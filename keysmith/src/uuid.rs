//! Generates a UUID
//! 
//! Currently only version 4 or nonstandard using more possible letters.

use crate::gen_char::gen_uuid_char;

/// Generate a UUID. 
/// 
/// The version input should be either '4' or 'n'.
/// 
/// Version 'n' is nonstandard which is
/// not necessarily recommended but could be useful.
/// 
/// v4 Ex: fc402d52-70be-7f09-caed-8da65db08985
/// 
/// nonstandard Ex: eko0c6ph-k2ok-60rr-pj78-mns182t9vurf
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