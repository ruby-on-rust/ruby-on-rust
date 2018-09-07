#
# === NUMERIC PARSING ===
#

%%{

int_hex  = ( xdigit+ '_' )* xdigit* '_'? ;
int_dec  = ( digit+ '_' )* digit* '_'? ;
int_bin  = ( [01]+ '_' )* [01]* '_'? ;

flo_int  = [1-9] [0-9]* ( '_' digit+ )* | '0';
flo_frac = '.' ( digit+ '_' )* digit+;
flo_pow  = [eE] [+\-]? ( digit+ '_' )* digit+;

int_suffix =
  ''       % { @num_xfrm = lambda { |chars| emit(:tINTEGER,   chars) } }
| 'r'      % { @num_xfrm = lambda { |chars| emit(:tRATIONAL,  Rational(chars)) } }
| 'i'      % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, chars)) } }
| 'ri'     % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, Rational(chars))) } }
| 're'     % { @num_xfrm = lambda { |chars| emit(:tINTEGER,   chars, @ts, @te - 2); p -= 2 } }
| 'if'     % { @num_xfrm = lambda { |chars| emit(:tINTEGER,   chars, @ts, @te - 2); p -= 2 } }
| 'rescue' % { @num_xfrm = lambda { |chars| emit(:tINTEGER,   chars, @ts, @te - 6); p -= 6 } };

flo_pow_suffix =
  ''   % { @num_xfrm = lambda { |chars| emit(:tFLOAT,     Float(chars)) } }
| 'i'  % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, Float(chars))) } }
| 'if' % { @num_xfrm = lambda { |chars| emit(:tFLOAT,     Float(chars), @ts, @te - 2); p -= 2 } };

flo_suffix =
  flo_pow_suffix
| 'r'      % { @num_xfrm = lambda { |chars| emit(:tRATIONAL,  Rational(chars)) } }
| 'ri'     % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, Rational(chars))) } }
| 'rescue' % { @num_xfrm = lambda { |chars| emit(:tFLOAT,     Float(chars), @ts, @te - 6); p -= 6 } };

}%%
