class Action
  def self.find_by_name name
    pair = $actions.find do |id, action|
      action.name == name
    end

    return nil if pair.nil?

    pair[1]
  end

  def self.find_by_id id
    $actions[id]
  end

  attr_reader :id
  attr_reader :name
  attr_reader :code

  def initialize code, name
    @code = code
    @name = name

    transform!

    # add to $actions
    @id = $actions.keys.size
    $actions[@id] = self
  end

  private

  def transform!
    header_comment =  if @name
                        "action: #{@name}"
                      else
                        "anonymouse action"
                      end

    # wrapper
    @code = """
            {
                // #{header_comment}
                println!(\"      invoking action: #{@name}\");
                #{@code}
            }
            """

    # fhold
    @code.gsub! 'fhold;', 'println!("      # holding 1 char..."); self.p = matched_slice_start_pos as isize - 1;'

    # fholdslice
    # NOTE not a ragel command, holds current slice instead of one char
    # TODO apparently they will be the same in lexer?
    @code.gsub! 'fholdslice;', 'println!("      # holding slice..."); self.p = matched_slice_start_pos as isize - 1;'

    # 
    # NOTE fnext and fgoto
    # fnext, not like fgoto, transfers the state after the action, instead of immediately,
    # apparently that's ok, since all existing `fnext` occurs at the end of an action
    # 
    # so we treat all `fgoto a` as `fnext a`
    # 

    # fgoto
    # 
    #   fgoto expr_value;
    # 
    @code.gsub!(/fgoto (.+);/) do |match|
      """
      fnext #$1;
      """
    end

    # fbreak
    @code.gsub! 'fbreak;', 'println!("      # breaking..."); is_breaking = true;'

    # fnext *
    # 
    #   fnext *expression;
    # 
    @code.gsub!(/fnext \*(.+);/) do |match|
      expression = $1
      """
      self.next_state = Some( #{expression} );
      println!(\"      # setting next_state to {:?}\", self.next_state);
      """
    end

    # fnext
    # 
    #   fnext expr_value;
    # 
    @code.gsub!(/fnext (.+);/) do |match|
      """
      self.next_state = Some(String::from(\"#$1\"));
      println!(\"      # setting next_state to {:?}\", self.next_state);
      """
    end

    # TODO fnext*

    # fcall
    # 
    #     fcall expr_value;
    # 
    @code.gsub!(/fcall (.+);/) do |match|
      state = $1;

      """
      println!(\"      invoking fcall #{state}\");
      self.state_stack_push(String::from(\"#{state}\"));
      self.next_state = Some(String::from(\"#{state}\"));
      """
    end

    # emit
    # 
    # emit T_IDENTIFIER, token_start_offset, token_end_offset;
    # 
    #     for T_IDENTIFIER(ts+start_offset, te+start_offset)
    # 
    # emit T_IDENTIFIER; => emit T_IDENTIFIER, 0, 0;
    # 
    # TODO HACKING
    # for Token variant without value, like Token::T_LBRACK, use a trailing _
    # 
    #     emit T_LBRACK_
    # 
    @code.gsub!(/emit (\w+);/) do |match| "emit #$1, 0, 0;" end
    @code.gsub!(/emit (\w+), (\d+), (\d+);/) do |match|
      variant = $1
      start_offset = $2
      end_offset = $3

      token = if variant.end_with? '_'
                "Token::#{variant.delete_suffix '_'}"
              else
                "Token::#{variant}(slice)"
              end

      """
      let slice = self.get_input_slice(matched_slice_start_pos + #{start_offset}, matched_slice_end_pos + #{end_offset});
      
      let token = #{token};
      self.emit_token(token);
      """
    end

    # emit_table KEYWORD
    @code.gsub!(/emit_table (\w+);/) do |match|
      table = $1

      """
      let slice = self.get_input_slice(matched_slice_start_pos, matched_slice_end_pos);
      let token = self.get_current_slice_as_token_from_table(\"#{table}\", slice);
      self.emit_token(token);
      """
    end

    # embed_action
    # 
    #     embed_action e_lbrack;
    # 
    @code.gsub!(/embed_action (\w+);/) do |match|
      action_name = $1
      action = Action.find_by_name action_name.to_sym

      action.code
    end
  end
end
