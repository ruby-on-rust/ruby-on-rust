# #
# # === TOKEN DEFINITIONS ===
# #

# # All operators are punctuation. There is more to punctuation
# # than just operators. Operators can be overridden by user;
# # punctuation can not.

# # A list of operators which are valid in the function name context, but
# # have different semantics in others.
# operator_fname      = '[]' | '[]=' | '`'  | '-@' | '+@' | '~@'  | '!@' ;
m! :operator_fname, '\[\]' # TODO ESCAPE

# # A list of operators which can occur within an assignment shortcut (+ → +=).
# operator_arithmetic = '&'  | '|'   | '&&' | '||' | '^'  | '+'   | '-'  |
#                       '*'  | '/'   | '**' | '~'  | '<<' | '>>'  | '%'  ;

# # A list of all user-definable operators not covered by groups above.
# operator_rest       = '=~' | '!~' | '==' | '!=' | '!'   | '===' |
#                       '<'  | '<=' | '>'  | '>=' | '<=>' | '=>'  ;

# # Note that `{` and `}` need to be referred to as e_lbrace and e_rbrace,
# # as they are ambiguous with interpolation `#{}` and should be counted.
# # These braces are not present in punctuation lists.

# # A list of punctuation which has different meaning when used at the
# # beginning of expression.
# punctuation_begin   = '-'  | '+'  | '::' | '('  | '['  |
#                       '*'  | '**' | '&'  ;

# # A list of all punctuation except punctuation_begin.
# punctuation_end     = ','  | '='  | '->' | '('  | '['  | ']'   |
#                       '::' | '?'  | ':'  | '.'  | '..' | '...' ;

# # A list of keywords which have different meaning at the beginning of expression.
# keyword_modifier    = 'if'     | 'unless' | 'while'  | 'until' | 'rescue' ;

# # A list of keywords which accept an argument-like expression, i.e. have the
# # same post-processing as method calls or commands. Example: `yield 1`,
# # `yield (1)`, `yield(1)`, are interpreted as if `yield` was a function.
# keyword_with_arg    = 'yield'  | 'super'  | 'not'    | 'defined?' ;

# # A list of keywords which accept a literal function name as an argument.
# keyword_with_fname  = 'def'    | 'undef'  | 'alias'  ;

# # A list of keywords which accept an expression after them.
# keyword_with_value  = 'else'   | 'case'   | 'ensure' | 'module' | 'elsif' | 'then'  |
#                       'for'    | 'in'     | 'do'     | 'when'   | 'begin' | 'class' |
#                       'and'    | 'or'     ;

# # A list of keywords which accept a value, and treat the keywords from
# # `keyword_modifier` list as modifiers.
# keyword_with_mid    = 'rescue' | 'return' | 'break'  | 'next'   ;

# # A list of keywords which do not accept an expression after them.
# keyword_with_end    = 'end'    | 'self'   | 'true'   | 'false'  | 'retry'    |
#                       'redo'   | 'nil'    | 'BEGIN'  | 'END'    | '__FILE__' |
#                       '__LINE__' | '__ENCODING__';

# # All keywords.
# keyword             = keyword_with_value | keyword_with_mid |
#                       keyword_with_end   | keyword_with_arg |
#                       keyword_with_fname | keyword_modifier ;

# constant       = c_upper c_alnum*;
# bareword       = c_alpha c_alnum*;

# call_or_var    = c_lower c_alnum*;
# class_var      = '@@' bareword;
# instance_var   = '@' bareword;
# global_var     = '$'
#     ( bareword | digit+
#     | [`'+~*$&?!@/\\;,.=:<>"] # `
#     | '-' c_alnum
#     )
# ;

# # Ruby accepts (and fails on) variables with leading digit
# # in literal context, but not in unquoted symbol body.
# class_var_v    = '@@' c_alnum+;
# instance_var_v = '@' c_alnum+;

# label          = bareword [?!]? ':';
