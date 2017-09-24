class Tuby::Parser
  prechigh
    nonassoc UMINUS
    left '*' '/'
    left '+' '-'
  preclow
rule
  target: scope

  assign: identifier '=' expr     { result = n(:Assign, val[0], val[2]) }

  expr: expr '+' expr             { iseq.add_operator(:"+")
                                    result = n(:Binary, *val[0..2]) }
      | expr '-' expr             { iseq.add_operator(:"-")
                                    result = n(:Binary, *val[0..2]) }
      | expr '*' expr             { iseq.add_operator(:"*")
                                    result = n(:Binary, *val[0..2]) }
      | expr '/' expr             { iseq.add_operator(:"/")
                                    result = n(:Binary, *val[0..2]) }
      | '(' expr ')'              { result = val[1] }
      | number
      | identifier

  number: '-' NUMBER  =UMINUS     { result = n(:Number, -val[1]) }
        | NUMBER                  { result = n(:Number, val[0]) }

  identifier: IDENT               { iseq.add_variable(val[0])
                                    result = n(:Ident, val[0]) }

  scope: statement                { result = n(:Scope, val[0]) }
       | scope stmt_end statement { val[0] << val[2]; result = val[0] }
       | /* none */               { result = n(:Scope) }

  statement: assign | expr

  stmt_end: NEWLINE | ';'
end

---- inner
  attr_reader :lexer, :iseq

  def parse(input)
    @iseq = InstructionSequence.new
    @lexer = Lexer.new(input)
    do_parse
  end

  def compile(input)
    @iseq = InstructionSequence.new
    @lexer = Lexer.new(input)

    iseq.scope = do_parse
    iseq
  end

  def self.parse(input)
    new.parse(input)
  end

  def self.compile(input)
    new.compile(input)
  end

  def next_token
    lexer.next_token
  end

  def n(type, *children)
    Node.const_get(type).new(lexer.lineno, *children)
  end
  