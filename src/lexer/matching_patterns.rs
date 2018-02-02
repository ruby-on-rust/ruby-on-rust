use std::collections::HashMap;

use regex::Regex;

pub fn construct() -> HashMap<&'static str, Regex> {
    // TODO macro-ize
    // TODO auto prepend ^
    let mut patterns = HashMap::new();

    // patterns.insert("w_any", Regex::new(r"^[[:space:]]").unwrap());

    patterns.insert("c_any", Regex::new(r"^.").unwrap());

    patterns.insert("int_dec", Regex::new(r"^[1-9][[:digit:]]*_?([[:digit:]]_)*[[:digit:]]*_?").unwrap());

    patterns
}
