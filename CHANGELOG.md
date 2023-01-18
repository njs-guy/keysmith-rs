# Changelog

For the source code of each version, see the [tags page](https://github.com/njshockey/keysmith-rs/tags).

## v0.3.1

Features

- Can now generate timestamps with custom epochs.
- Can now generate timestamps from the first second of 1970 (utc).

Development

- Nicer formatting thanks to [Rustfmt](https://rust-lang.github.io/rustfmt/).

## v0.3.0

**_CAUTION_**: v0.3.0 introduces breaking changes. Mainly with importing modules.
Be careful when upgrading from an older version.

Features

- Can now get a timestamp for the current time.
    - Use get_timestamp() or get_timestamp_i64() for this.
    - Based on number of seconds since the first second of 2022.
- Can now generate keys with only numbers and letters.
    - Use gen_nums_and_letters() for this.
    - gen_nums_and_letters_lower() for just lowercase letters.
    - gen_nums_and_letters_upper() for just uppercase letters.
- Generate a single character with the gen_char module.
    - gen_char() to generate a character found in gen_key().
    - gen_uuid_char() to generate a character found in gen_uuid().

API

- Functions have been separated into different modules.
Import relevant features with keysmith::{key, timestamp, uuid, gen_char}
where necessary. See [docs](https://docs.rs/keysmith/latest/keysmith/).

Docs

- Rust generated auto docs. Link to newest version can be found on [docs.rs](https://docs.rs/keysmith/latest/keysmith/).

Testing

- Can now toggle individual modules and functions.
- Cleaned up ks-tests crate.

Development

- gen_char() and gen_uuid_char() are now public
and can be used with the keysmith crate.
- Cleaned up a lot of code under the hood.

## v0.2.1

You can now generate keys with only specific kinds of characters.
For more details, see [usage](README.md#usage).

- gen_nums() to generate only numbers.
- gen_letters() to generate only letters (either uppercase or lowercase).
- gen_letters_lower() to generate only lowercase letters.
- gen_letters_upper() to generate only uppercase letters.
- gen_special_chars() to generate only safe special characters.
- gen_special_chars_unsafe() to generate only unsafe special characters.
    - WARNING: These are unsafe because they tend to break
      file structures and URL's and the like.
    - This is not recommended but might be useful to _someone_.

## v0.2.0

- Can now generate version 4 UUID's with gen_uuid('4'). See [usage](README.md#usage).
- "Nonstandard" UUID's can also be generated with gen_uuid('n'). See [usage](README.md#usage).

## v0.1.1

- gen_key() can now generate uppercase letters.
- Fixed a build error on Linux.

## v0.1.0

- First release.
- Generate a key with gen_key(). See [usage](README.md#usage).
