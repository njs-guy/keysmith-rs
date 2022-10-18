# Keysmith

Create keys or ids for data with Rust.

***WARNING***

Keysmith is currently a WIP and not recommended for production projects yet.

Here's some example keys:

```
5WdT@KfYRyRDqh0AAVUsyitSXbm4OQwzClX9.XYn5kLmKeObCf8YE3HqzHdU3UTS
g1Y_Zp9-9rDf7VAaR1KGnH9Uf5klGjSQ6P2rAjK35iK-GnZ@dVXYu.aElzTfTOaK
M3rkxIbrD0lz-SpUBe704X2)Gd(_WprzRJW7N51O@_58180Gs9esIVBO5.OdFOlY
```

This looks like gibberish, but that's the point. Two id's in a database should not overlap. 
So if you need a lot of entries, you need to reduce the odds of that happening. 
If the odds of two identical id's are astronomically low, you shouldn't even need to check for an existing id in the first place.
Keysmith does that generation for you and outputs it as a String.

You can also generate version 4 UUID's. Examples:
```
be3b5529-931b-6b75-1678-a057bccf71c9
440146ab-a19a-8a36-2d4b-ba19a79570d4
0090da40-6ce2-59d3-629d-11b293c9e2d3
```

# Usage

First, add this to your `cargo.toml` dependencies. Check out the [crate page](https://crates.io/crates/keysmith) to make sure you're using the latest version.

```toml
[dependencies]
keysmith = "0.2.0"
```
Next, in the actual code:

```rs
use keysmith;

fn main() {
   let key1: String = keysmith::gen_key(64);
   println!("{}", key1);
   // ex: aVCkmMFkv3UqLIU2tC52DQOWrIg4RsaH.xvTCTvA_PVHY@MOIWH~y1610KIZ@qM@

   let key2: String = keysmith::keysmith::gen_uuid('4');
   println!("{}", key2);
   // ex: da748b8b-e915-661b-466a-0d6a0480112a

   let key3: String = keysmith::keysmith::gen_uuid('n');
   println!("{}", key3);
   // "Nonstandard" UUID
   // ex: eko0c6ph-k2ok-60rr-pj78-mns182t9vurf
}
```
The first function generates a key with a length of 64 characters as a String. The second generates a version 4 UUID. 
The third generates a "nonstandard" UUID. It uses the same structure as a version 4, but it can have any letter instead of just a-f.
The "nonstandard" version is obviously not standard, but may be useful depending on your preferences.

# Changelog

You can find the changelog in [CHANGELOG.md](CHANGELOG.md).

# Building
1. Install [rustup](https://www.rust-lang.org/tools/install) if you haven't already.
2. Clone this repo. Usually with `git clone https://github.com/njshockey/keysmith-rs.git`.
3. Run `cargo build` or `cargo build --release` to build.
   - To run the test package, run `cargo run -p ks-tests` instead.

# License
Keysmith uses the Rust standard MIT/Apache 2.0 dual license for best compatibility. See [LICENSE-APACHE.txt](LICENSE-APACHE.txt) and [LICENSE-MIT.txt](LICENSE-MIT.txt) for the full licenses.