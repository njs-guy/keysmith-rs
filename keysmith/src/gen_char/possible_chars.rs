use std::collections::HashMap;

pub fn get_poss_chars() -> HashMap<&'static str, &'static str> {
    let possible_chars = HashMap::from([
        ("numbers","0123456789"),
        ("en_alphabet", "abcdefghijklmnopqrstuvwxyz"),
        ("safe_sp_chars", "-_.()~@"),
        ("unsafe_sp_chars", r#"#%&*+={}\/<>?!$:'"`|"#),
    ]);

    // See this https://stackoverflow.com/a/40415059
    // Might want to look more into this later

    possible_chars
}