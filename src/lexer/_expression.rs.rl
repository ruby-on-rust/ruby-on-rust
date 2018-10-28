%%{
#
# === EXPRESSION PARSING ===
#

# These rules implement a form of manually defined lookahead.
# The default longest-match scanning does not work here due
# to sheer ambiguity.

ambiguous_fid_suffix =         # actual    parsed
    [?!]    %{ tm = p }      | # a?        a?
    [?!]'=' %{ tm = p - 2 }    # a!=b      a != b
;

ambiguous_ident_suffix =       # actual    parsed
    ambiguous_fid_suffix     |
    '='     %{ tm = p }      | # a=        a=
    '=='    %{ tm = p - 2 }  | # a==b      a == b
    '=~'    %{ tm = p - 2 }  | # a=~b      a =~ b
    '=>'    %{ tm = p - 2 }  | # a=>b      a => b
    '==='   %{ tm = p - 3 }    # a===b     a === b
;

ambiguous_symbol_suffix =      # actual    parsed
    ambiguous_ident_suffix |
    '==>'   %{ tm = p - 2 }    # :a==>b    :a= => b
;

# Ambiguous with 1.9 hash labels.
ambiguous_const_suffix =       # actual    parsed
    '::'    %{ tm = p - 2 }    # A::B      A :: B
;

# Resolving kDO/kDO_COND/kDO_BLOCK ambiguity requires embedding
# @cond/@cmdarg-related code to e_lbrack, e_lparen and e_lbrace.

e_lbrack = '[' % {
    self.cond.push(false); self.cmdarg.push(false);
};

# Ruby 1.9 lambdas require parentheses counting in order to
# emit correct opening kDO/tLBRACE.

e_lparen = '(' % {
    self.cond.push(false); self.cmdarg.push(false);

    self.paren_nest += 1;
};

e_rparen = ')' % {
    self.paren_nest -= 1;
};

# Ruby is context-sensitive wrt/ local identifiers.
action local_ident {
    !emit T_IDENTIFIER;

    // TODO WIP
    // if !@static_env.nil? && @static_env.declared?(tok)
    //   fnext expr_endfn; fbreak;
    // else
    //   fnext *arg_or_cmdarg; fbreak;
    // end
    fnext *self.arg_or_cmdarg(); fnbreak;
}
}%%
