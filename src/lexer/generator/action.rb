class Action
  def self.find_by_name name
    pair = $actions.find do |id, action|
      action.name == name
    end
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

    # fbreak
    @code.gsub! 'fbreak;', 'println!("      # breaking..."); is_breaking = true;'

    # 
    # fgoto *expression
    # 
    #     fgoto *push_literal(tok, tok, @ts);
    # 
    # NOTE
    # fgoto and fgoto * emulations will transfer the state after this action, instead of immediately,
    # apparently that's ok, since all `fgoto` occurs at the end of an action
    @code.gsub!(/fgoto \*(.+);/) do |match|
      expression = $1
      """
      self.next_state = Some( ( #{expression} ) );
      println!(\"      # setting next_state to {:?}\", self.next_state);
      """
    end
    # fgoto
    # 
    #   fgoto expr_value;
    # 
    # 
    @code.gsub!(/fgoto (.+);/) do |match|
      "self.next_state = Some(String::from(\"#$1\")); println!(\"      # setting next_state to {:?}\", self.next_state);"
    end

    # fnext
    # 
    #   fnext expr_value;
    # 
    @code.gsub!(/fnext (.+);/) do |match|
      "self.next_state = Some(String::from(\"#$1\")); println!(\"      # setting next_state to {:?}\", self.next_state);"
    end

    # TODO fnext*

    # emit
    # 
    # emit TIdentifier, token_start_offset, token_end_offset;
    # 
    #     for TIdentifier(ts+start_offset, te+start_offset)
    # 
    # emit TIdentifier; => emit TIdentifier, 0, 0;
    # 
    @code.gsub!(/emit (\w+);/) do |match| "emit #$1, 0, 0;" end
    @code.gsub!(/emit (\w+), (\d+), (\d+);/) do |match|
      variant = $1
      start_offset = $2
      end_offset = $3

      # TIdentifier -> T_IDENTIFIER
      variant = variant[0] + '_' + variant.slice(1..-1).upcase

      """
      let slice = self.get_input_slice(matched_slice_start_pos + #{start_offset}, matched_slice_end_pos + #{end_offset});
      let token = Token::#{variant}(slice);
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

  end
end
