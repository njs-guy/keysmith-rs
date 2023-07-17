//! Generates a UUID.
//!
//! Currently only version 4 or "nonstandard".
//!
//! Nonstandard is the same as v4 but uses a-z instead of just a-f.

use crate::char::uuid_char;

/// UUID version
#[derive(Debug, Clone, Copy)]
pub enum UUID {
	V4,
	Nonstandard,
}

/// Generate a UUID.
///
/// The version input should be a UUID enum.
///
/// v4 Ex: fc402d52-70be-7f09-caed-8da65db08985
///
/// nonstandard Ex: eko0c6ph-k2ok-60rr-pj78-mns182t9vurf
fn gen_uuid(version: UUID) -> String {
	let mut output = String::from("");
	let mut c: char;

	// first set of digits
	for _n in 1..=8 {
		c = uuid_char(version);

		output.push(c);
	}

	output.push('-'); // Current state: fc402d52-

	// second through fourth sets of digits
	for _n in 1..=3 {
		for _x in 1..=4 {
			c = uuid_char(version);
			output.push(c);
		}

		output.push('-');
		// After one loop: efc402d52-70be-
	}

	// Current state: fc402d52-70be-7f09-caed-

	// final set of digits
	for _n in 1..=12 {
		c = uuid_char(version);
		output.push(c);
	}

	output // fc402d52-70be-7f09-caed-8da65db08985
}

// Public API
// TODO: 0.5 - UUID v1 and v2
// TODO: 0.5.1 - UUID v3 and v5

/// Generate a UUID (version 4).
///
/// ex: fc402d52-70be-7f09-caed-8da65db08985
pub fn uuid4() -> String {
	gen_uuid(UUID::V4)
}

/// Generate a UUID (nonstandard).
/// This is the same as a v4, but letters can be
/// a-z instead of just a-f.
///
/// ex: l8fx3px5-9lyk-gzrb-iu75-d4gp63chor9z
pub fn uuidn() -> String {
	gen_uuid(UUID::Nonstandard)
}

// Tests
// TODO: Ensuring that valid UUID characters are generated.
// TODO: Ensuring that uuid4() and uuidn() generate a valid uuid.
