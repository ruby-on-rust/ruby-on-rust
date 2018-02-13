use lexer::Lexer;
use lexer::LexingState;

impl Lexer {
    // def arg_or_cmdarg
    //   if @command_state
    //     self.class.lex_en_expr_cmdarg
    //   else
    //     self.class.lex_en_expr_arg
    //   end
    // end
    pub fn arg_or_cmdarg(&self) -> LexingState {
        if self.command_state {
            LexingState::ExprCmdarg
        } else {
            LexingState::ExprArg
        }
    }
}
