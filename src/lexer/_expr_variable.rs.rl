%%{
# Variable lexing code is accessed from both expressions and
# string interpolation related code.
#
expr_variable := |*
    global_var
    => {
      // TODO
      // if tok =~ /^\$([1-9][0-9]*)$/
      //   emit(:tNTH_REF, tok(@ts + 1).to_i)
      // elsif tok =~ /^\$([&`'+])$/
      //   emit(:tBACK_REF)
      // else
      //   emit(:tGVAR)
      // end

      !emit T_GVAR;
      !fnext_stack_pop;
      fnbreak;
    };

    class_var_v
    => {
      // TODO
      // if tok =~ /^@@[0-9]/
      //   diagnostic :error, :cvar_name, { :name => tok }
      // end

      !emit T_CVAR;
      !fnext_stack_pop;
      fnbreak;
    };

    instance_var_v
    => {
      // TODO
      // if tok =~ /^@[0-9]/
      //   diagnostic :error, :ivar_name, { :name => tok }
      // end

      !emit T_IVAR;
      !fnext_stack_pop;
      fnbreak;
    };
*|;
}%%
