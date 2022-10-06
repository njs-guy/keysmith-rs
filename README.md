# Keysmith

Create keys/ids for data with Rust.

Here's some example keys:

```
w~z69_iv.u@cxbguln_3vvpmk0l(nxje(oteuxe~j6p8xcuer76ump-1rc39(8~q
7y--i3spd1r0q-a8ahyelv1r8_bcr@zc0zlms@sag29q@.wredx_o~p.p3-_28jy
u7law0~xu7p@qwpa)@-3(iv3o5t-.di0~ihxj4d47xuq5@yftb_9bdpo2)@8)ry)
```

This looks like gibberish, but that's the point. Two id's in a database should not overlap. 
So if you need a lot of entries, you need to reduce the odds of that happening. 
If the odds of two identical id's are astronomically low, you shouldn't even need to check for an existing id in the first place.
Keysmith does that generation for you and outputs it as a String.

# Building
1. Install [rustup](https://www.rust-lang.org/tools/install) if you haven't already.
2. Clone this repo. Usually with `git clone https://github.com/njshockey/keysmith-rs.git`.
3. Run `cargo run` at the root of this directory to test changes.
   - To run the test package, run `cargo run -p ks-tests` instead.
4. Run `cargo build` or `cargo build --release` to build.

# License
Keysmith uses the Rust standard MIT/Apache 2.0 dual license for best compatibility. See [LICENSE-APACHE.txt](LICENSE-APACHE.txt) and [LICENSE-MIT.txt](LICENSE-MIT.txt) for the full licenses.