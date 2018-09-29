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
    @code.gsub! 'fhold;', 'println!("      # holding..."); is_holding = true;'

    # fbreak
    @code.gsub! 'fbreak;', 'println!("      # breaking..."); is_breaking = true;'

    # fgoto
    # 
    #   fgoto expr_value;
    # 
    # NOTE
    # this emulated-fgoto will transfer the state after this action, instead of immediately,
    # apparently that's ok, since all `fgoto` occurs at the end of an action
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
      self.tokens.push(token);
      """
    end

  end
end
