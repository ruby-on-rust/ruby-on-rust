use std::collections::HashMap;

use regex::Regex;

pub fn construct() -> HashMap<&'static str, Regex> {
    let mut patterns = HashMap::new();

    macro_rules! pattern {
        ($name:expr, $regex:expr) => {
            patterns.insert($name, Regex::new( &format!(r"^{}", $regex) ).unwrap());
        };
    }

    // TODO maybe impl a macro patterns!

    // 
    // CHARACTER CLASSES
    // 

    pattern!("any", ".");

    // ORIGINAL
    //     c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
    pattern!("c_eof", "\\z");

    // ORIGINAL
    //     c_nl       = '\n' $ do_nl;
    //     c_eol      = c_nl | c_eof;
    pattern!("c_nl", "\\n");
    pattern!("c_eol", "\\n");

    //     ORIGINAL
    //         c_any = any - c_eof;
    patterns.insert("c_any", Regex::new(r"(?s)^.").unwrap()); // TODO not sure if this is correct?


    // 
    // TOKEN DEFINITIONS
    // 

    // TODO INCOMPLETED
    //     ORIGINAL keyword_with_end
    //         # A list of keywords which do not accept an expression after them.
    //         keyword_with_end    = 'end'    | 'self'   | 'true'   | 'false'  | 'retry'    |
    //                               'redo'   | 'nil'    | 'BEGIN'  | 'END'    | '__FILE__' |
    //                               '__LINE__' | '__ENCODING__';
    pattern!("keyword_with_end", "(true)");

    // TODO INCOMPLETED
    //     ORIGINAL keyword
    //         # All keywords.
    //         keyword             = keyword_with_value | keyword_with_mid |
    //                               keyword_with_end   | keyword_with_arg |
    //                               keyword_with_fname | keyword_modifier ;
    pattern!("keyword", "(true)");

    // 
    // NUMERIC PARSING
    // 

    // TODO ORIGINAL
    pattern!("int_dec", "[1-9][[:digit:]]*_?([[:digit:]]_)*[[:digit:]]*_?");

    patterns
}
