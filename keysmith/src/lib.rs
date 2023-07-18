// TODO: Update docs with new API.
// TODO: Add timestamps as examples.

//! Create keys or ids for data with Rust.
//!
//! ***WARNING***
//!
//! Keysmith is currently a WIP, so breaking changes may occur in between major versions.
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
//! # Usage
//!
//! First, add this to your `Cargo.toml` dependencies.
//! Check out the [crate page](https://crates.io/crates/keysmith) to make sure you're using the latest version.
//!
//! ```toml
//! [dependencies]
//! keysmith = "0.3.1"
//! ```
//! Next, in the actual code:
//!
//! ```
//! use keysmith::{key, uuid};
//!
//! let key1: String = key::gen_key(64);
//! println!("{}", key1);
//! // ex: aVCkmMFkv3UqLIU2tC52DQOWrIg4RsaH.xvTCTvA_PVHY@MOIWH~y1610KIZ@qM@
//!
//! let key2: String = uuid::gen_uuid('4');
//! println!("{}", key2);
//! // ex: da748b8b-e915-661b-466a-0d6a0480112a
//!
//! let key3: String = uuid::gen_uuid('n');
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
//! let key1: String = key::gen_nums(32);
//! println!("{}", key1);
//! // ex: 04356417134317004828941212534445
//!
//! let key2: String = key::gen_letters(32);
//! println!("{}", key2);
//! // ex: PBSZWwSTmRalGnzeSbQUKmXRikKUWXvj
//!
//! let key3: String = key::gen_special_chars(32);
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
//! let key1: String = key::gen_letters_lower(32);
//! println!("{}", key1);
//! // ex: xoewhgvjsqzctfgpaqwnhanbgweflpqc
//!
//! let key2: String = key::gen_letters_upper(32);
//! println!("{}", key2);
//! // ex: EVQMPIHKDBPLZJBPCHTXTIBLYRSFFFUY
//! ```
//!
//! You can also generate a key using only "unsafe" special characters.
//! Be careful with this, as these characters generally break file structures or URL's which is why they are considered unsafe.
//! Generating a key of this type is not recommended, but could be useful to *someone*.
//!
//! ```
//! use keysmith::key;
//!
//! let key: String = key::gen_special_chars_unsafe(32);
//! println!("{}", key);
//!
//! // Possible characters: #%&*+={}\/<>?!$:'"`|
//! // ex: <#=`=*%{:`*%!<{"|*?'!#\#|?\+{=\}
//! ```
//!
//! If you need something more specific than what's already here by default,
//! you can generate a single char.
//!
//! ```
//! use keysmith::gen_char::{gen_char, GenCharOpts};
//!
//! let opts = GenCharOpts {
//!     nums: true,
//!     letters: true,
//!     upper: true,
//!     safe_sp_chars: true,
//!     unsafe_sp_chars: false,
//! };
//!
//! let c: char = gen_char(opts);
//! println!("{}", c);
//! // ex: H
//! ```
//!
//! Keysmith also lets you get a timestamp.
//! This is in seconds since the first second of 2022.
//! String is the default type for this, but you can also return an i64.
//! ```
//! use keysmith::timestamp;
//!
//! let stamp: String = timestamp::get_timestamp();
//! let stamp2: i64 = timestamp::get_timestamp_i64();
//!
//! println!("{}", stamp);
//! println!("{}", stamp2);
//! // ex: 25928414
//! ```

pub mod char;
pub mod date;
pub mod key;
pub mod timestamp;
pub mod uuid;

// TODO: 0.4 - More tests. Timestamp formatting, UUID lengths, possible chars use the correct chars.
