# Keysmith

Create keys or ids for data with Rust.

***NOTICE***

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

# Usage

First, add this to your `cargo.toml` dependencies. Check out the [crate page](https://crates.io/crates/keysmith) to make sure you're using the latest version.

```toml
[dependencies]
keysmith = "0.1.1"
```
Next, in the actual code:

```rs
use keysmith;

fn main() {
   let key: String = keysmith::gen_key(64);
   println!("{}", key);
   // ex: rrPa(Z@~(4zSRj2cqzRd8R6QAxh35f76-Y_S-VvWLgzOCdfGGzYqqGrfxvg94GVU
}
```
The above generates a key with a length of 64 characters as a String.

# Changelog

You can find the changelog in [CHANGELOG.md](CHANGELOG.md).

# Building
1. Install [rustup](https://www.rust-lang.org/tools/install) if you haven't already.
2. Clone this repo. Usually with `git clone https://github.com/njshockey/keysmith-rs.git`.
3. Run `cargo build` or `cargo build --release` to build.
   - To run the test package, run `cargo run -p ks-tests` instead.

# License
Keysmith uses the Rust standard MIT/Apache 2.0 dual license for best compatibility. See [LICENSE-APACHE.txt](LICENSE-APACHE.txt) and [LICENSE-MIT.txt](LICENSE-MIT.txt) for the full licenses.