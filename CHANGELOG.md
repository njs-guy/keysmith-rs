For the source code of each version, see the [tags page](https://github.com/njshockey/keysmith-rs/tags).

# v0.3.0
- Cleaned up a lot of code under the hood.

# v0.2.1
You can now generate keys with only specific kinds of characters. For more details, see [usage](README.md#usage).

- gen_nums() to generate only numbers.
- gen_letters() to generate only letters (either uppercase or lowercase).
- gen_letters_lower() to generate only lowercase letters.
- gen_letters_upper() to generate only uppercase letters.
- gen_special_chars() to generate only safe special characters.
- gen_special_chars_unsafe() to generate only unsafe special characters.
  - WARNING: These are unsafe because they tend to break file structures and URL's and the like.
  - This is not recommended but might be useful to *someone*.

# v0.2.0
- Can now generate version 4 UUID's with gen_uuid('4'). See [usage](README.md#usage).
- "Nonstandard" UUID's can also be generated with gen_uuid('n'). See [usage](README.md#usage).

# v0.1.1
- gen_key() can now generate uppercase letters.

# v0.1.0
- First release.
- Generate a key with gen_key(). See [usage](README.md#usage).