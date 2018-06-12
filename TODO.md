- parser
- lexer
  - checkout ragel in spack

--- LEGACY BELOW ---

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

use pattern_literals to construct complex patterns inline in action

#

[1-9] digit* '_'? %{ @num_base = 10; @num_digits_s = @ts } int_dec

somethingA %{actionB} somethingB

stateA --(transition and invoke action)--> stateB

like this?

# rust practice

- &'static str
