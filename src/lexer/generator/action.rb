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
    @code.gsub! 'fhold;', 'is_holding = true;'

    # fbreak
    @code.gsub! 'fbreak;', 'is_breaking = true;'

    # fgoto
    # 
    #   fgoto expr_value;
    # 
    # NOTE
    # this emulated-fgoto will transfer the state after this action, instead of immediately,
    # apparently that's ok, since all `fgoto` occurs at the end of an action
    # 
    @code.gsub!(/fgoto (.+);/) do |match|
      "self.next_state = Some(String::from(\"#$1\"));"
    end

    # fnext
    # 
    #   fnext expr_value;
    # 
    @code.gsub!(/fnext (.+);/) do |match|
      "self.next_state = Some(String::from(\"#$1\"));"
    end

    # TODO fnext*

    # emit
    # 
    # emit TIdentifier;
    # 
    @code.gsub!(/emit ([TK].+);/) do |match|
      variant = $1

      # TIdentifier -> T_IDENTIFIER
      variant = variant[0] + '_' + variant.slice(1..-1).upcase

      """
      let token = Token::#{variant}(current_matched_slice.clone());
      self.tokens.push(token);
      """
    end
  end
end
