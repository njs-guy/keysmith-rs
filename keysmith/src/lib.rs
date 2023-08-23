//! Create keys, uuid's, timestamps, and more with Rust..
//!
//!
//! Here's some example keys:
//!
//! ```text
//! 5WdT@KfYRyRDqh0AAVUsyitSXbm4OQwzClX9.XYn5kLmKeObCf8YE3HqzHdU3UTS
//! g1Y_Zp9-9rDf7VAaR1KGnH9Uf5klGjSQ6P2rAjK35iK-GnZ@dVXYu.aElzTfTOaK
//! M3rkxIbrD0lz-SpUBe704X2)Gd(_WprzRJW7N51O@_58180Gs9esIVBO5.OdFOlY
//! ```
//!
//! This looks like gibberish, but that's the point. Two id's in a database should not overlap.
//! So if you need a lot of entries, you need to reduce the odds of that happening.
//! If the odds of two identical id's are astronomically low, you shouldn't even need to check for an existing id in the first place.
//! Keysmith does that generation for you and outputs it as a String.
//!
//! You can also generate version 4 UUID's. Examples:
//! ```text
//! be3b5529-931b-6b75-1678-a057bccf71c9
//! 440146ab-a19a-8a36-2d4b-ba19a79570d4
//! 0090da40-6ce2-59d3-629d-11b293c9e2d3
//! ```
//!
//! Keysmith also supports many other data formats.
//! - Keys strings from custom character sets
//! - "Nonstandard" V4 UUID's using any letter as opposed to just a-f
//! - Timestamps (ex: 50546851)
//! - Dates (ex: 2023-07-17), times (ex: 19:34:07), and datetimes (ex: 2023-07-17T19:34:07)
//! - Individual characters
//!
//! # Usage
//!
//! First, add this to your `Cargo.toml` dependencies.
//! Check out the [crate page](https://crates.io/crates/keysmith) to make sure you're using the latest version.
//!
//! ```toml
//! [dependencies]
//! keysmith = "0.4.0"
//! ```
//! Next, in the actual code:
//!
//! ```
//! use keysmith::{key, uuid};
//!
//! let key1: String = key::key(64);
//! println!("{}", key1);
//! // ex: aVCkmMFkv3UqLIU2tC52DQOWrIg4RsaH.xvTCTvA_PVHY@MOIWH~y1610KIZ@qM@
//!
//! let key2: String = uuid::uuid();
//! println!("{}", key2);
//! // ex: da748b8b-e915-661b-466a-0d6a0480112a
//!
//! let key3: String = uuid::uuidn();
//! println!("{}", key3);
//! // "Nonstandard" UUID
//! // ex: eko0c6ph-k2ok-60rr-pj78-mns182t9vurf
//! ```
//! The first function generates a key with a length of 64 characters as a String.
//! The second generates a version 4 UUID.
//! The third generates a "nonstandard" UUID.
//! It uses the same structure as a version 4, but it can have any letter instead of just a-f.
//! The "nonstandard" version is obviously not standard,
//! but may be useful depending on your preferences.
//!
//!
//! You can also generate a key consisting of only a specific kind of character.
//!
//! ```
//! use keysmith::key;
//!
//! let key1: String = key::key(32);
//! println!("{}", key1);
//! // ex: 04356417134317004828941212534445
//!
//! let key2: String = key::letters(32);
//! println!("{}", key2);
//! // ex: PBSZWwSTmRalGnzeSbQUKmXRikKUWXvj
//!
//! let key3: String = key::special_chars(32);
//! println!("{}", key3);
//! // Special characters generally considered "safe."
//! // Possible characters: -_.()~@
//! // ex: )@-_~@_@._))~)@))@.)(-)@(.@(~((@
//! ```
//!
//! If you want only lowercase or uppercase letters, you can do that, too!
//!
//! ```
//! use keysmith::key;
//!
//! let key1: String = key::letters_lower(32);
//! println!("{}", key1);
//! // ex: xoewhgvjsqzctfgpaqwnhanbgweflpqc
//!
//! let key2: String = key::letters_upper(32);
//! println!("{}", key2);
//! // ex: EVQMPIHKDBPLZJBPCHTXTIBLYRSFFFUY
//! ```
//!
//! If you need something more specific than what's already here by default,
//! you can generate a single char.
//!
//! ```
//! use keysmith::char::{char, GenCharOpts};
//!
//! let opts = GenCharOpts {
//!     nums: true,
//!     letters: true,
//!     upper: true,
//!     safe_sp_chars: true,
//!     unsafe_sp_chars: false,
//! };
//!
//! let c: char = char(opts);
//! println!("{}", c);
//! // ex: H
//! ```
//!
//! Or just use your own character set.
//!
//! ```
//! use keysmith::key;
//!
//! let charset = String::from("among_us");
//! let key: String = key::key_custom(32, charset);
//! println!("{}", key);
//!
//! // ex: unnusgnggauogoauusu__msguo_gmomg
//! ```
//!
//! Keysmith also lets you get a timestamp.
//! This is in seconds since the first second of 2022.
//! String is the default type for this, but you can also return an i64.
//! ```
//! use keysmith::timestamp;
//!
//! let stamp: String = timestamp::timestamp();
//! let stamp2: i64 = timestamp::timestamp_i64();
//!
//! println!("{}", stamp);
//! println!("{}", stamp2);
//! // ex: 25928414
//! ```
//!
//! Timestamps can also be in milliseconds.
//!
//! ```
//! use keysmith::timestamp;
//!
//! let stamp: String = timestamp::timestamp();
//! let stamp2: i64 = timestamp::timestamp_i64();
//!
//! println!("{}", stamp);
//! println!("{}", stamp2);
//! // ex: 25928414
//! ```
//!
//! Finally, you can get the current date or time.
//!
//! ```
//! use keysmith::date;
//!
//! let date: String = date::date();
//! let time: String = date::time();
//! let datetime: String = date::datetime();
//!
//! println!("{}", date); // ex: 2023-08-16
//! println!("{}", time); // ex: 19:29:10
//! println!("{}", datetime); // ex: 2023-08-16T19:29:10
//! ```
//!
//! Those were just the basics. For more functionality, see the modules below.

pub mod char;
pub mod date;
pub mod key;
mod test_utils;
pub mod timestamp;
pub mod uuid;
