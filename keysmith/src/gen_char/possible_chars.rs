use std::collections::HashMap;

// Returns a hash map for the possible characters
// of a randomly generated key
pub fn get_poss_chars() -> HashMap<&'static str, &'static str> {
	HashMap::from([
		("numbers", "0123456789"),
		("en_alphabet", "abcdefghijklmnopqrstuvwxyz"),
		("safe_sp_chars", "-_.()~@"),
		("unsafe_sp_chars", r#"#%&*+={}\/<>?!$:'"`|"#),
	])

	// See this https://stackoverflow.com/a/40415059 for special chars
	// Might want to look more into this later
}

// Returns a hash map for the possible characters
// of a uuid
pub fn get_uuid_chars() -> HashMap<&'static str, &'static str> {
	HashMap::from([("numbers", "0123456789"), ("letters", "abcdef")])
}
