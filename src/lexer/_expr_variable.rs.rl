%%{
# Variable lexing code is accessed from both expressions and
# string interpolation related code.
#
expr_variable := |*
    global_var
    => {
      if    tok =~ /^\$([1-9][0-9]*)$/
        emit(:tNTH_REF, tok(@ts + 1).to_i)
      elsif tok =~ /^\$([&`'+])$/
        emit(:tBACK_REF)
      else
        emit(:tGVAR)
      end

      fnext *stack_pop; fbreak;
    };

    class_var_v
    => {
      if tok =~ /^@@[0-9]/
        diagnostic :error, :cvar_name, { :name => tok }
      end

      emit(:tCVAR)
      fnext *stack_pop; fbreak;
    };

    instance_var_v
    => {
      if tok =~ /^@[0-9]/
        diagnostic :error, :ivar_name, { :name => tok }
      end

      emit(:tIVAR)
      fnext *stack_pop; fbreak;
    };
*|;
}%%