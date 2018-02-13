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
- auto rewrite ragel transitions?

differents between @cs(current state in ragel) and the top state of the states stack

#

use rust macro top construct enum

#

[1-9] digit* '_'? %{ @num_base = 10; @num_digits_s = @ts } int_dec

somethingA %{actionB} somethingB

stateA --(transition and invoke action)--> stateB

like this?
