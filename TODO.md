# build.rs not invoked

current build cmd

```
./vendor/lemon/lemon_rust src/parser/parser.y -Tvendor/lemon/lempar.rs
```

#

- mechanism of ragel
  - how to handle eof
  - difference between @p and p
  - % in matching pattern
- testing
- auto rewrite ragel transitions?

#

use rust macro top construct enum

#

[1-9] digit* '_'? %{ @num_base = 10; @num_digits_s = @ts } int_dec

somethingA %{actionB} somethingB

stateA --(transition and invoke action)--> stateB

like this?
