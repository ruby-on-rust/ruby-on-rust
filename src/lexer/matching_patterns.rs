use std::collections::HashMap;

use regex::Regex;

pub fn construct() -> HashMap<&'static str, Regex> {
    // TODO auto prepend ^
    // TODO macro-ize
    let mut patterns = HashMap::new();

    // ORIGINAL
    //     c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
    patterns.insert("c_eof", Regex::new(r"^\z").unwrap());

    // ORIGINAL
    //     c_nl       = '\n' $ do_nl;
    //     c_eol      = c_nl | c_eof;
    patterns.insert("c_eol", Regex::new(r"^\n").unwrap());

    // ORIGINAL
    //     c_any = any - c_eof;
    patterns.insert("c_any", Regex::new(r"(?s)^.").unwrap()); // TODO is this right?

    patterns.insert("int_dec", Regex::new(r"^[1-9][[:digit:]]*_?([[:digit:]]_)*[[:digit:]]*_?").unwrap());

    patterns
}
