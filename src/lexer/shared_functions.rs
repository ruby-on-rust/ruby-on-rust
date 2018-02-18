use lexer::Lexer;
use lexer::LexingState;

use parser::parser::Token;

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

    // def emit_do(do_block=false)
    //     if @cond.active?
    //     emit(:kDO_COND, 'do'.freeze)
    //     elsif @cmdarg.active? || do_block
    //     emit(:kDO_BLOCK, 'do'.freeze)
    //     else
    //     emit(:kDO, 'do'.freeze)
    //     end
    // end
    pub fn emit_do(&mut self, do_block: bool) {
        if self.cond.is_active() {
            self.emit_token(Token::K_DO_COND);
        } else {
            if self.cmdarg.is_active() || do_block {
                self.emit_token(Token::K_DO_BLOCK);
            } else {
                self.emit_token(Token::K_DO);
            }
        }
    }
}
