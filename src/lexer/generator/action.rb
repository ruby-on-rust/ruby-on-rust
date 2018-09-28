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
    # @code.gsub!(/fgoto (.+);/) do |state|
    #   'self.next_state = \1;'
    # end
    # TODO the code above doesnot work

    if match = @code.match(/fgoto (.+);/)
      state = match.captures.first

      @code.gsub! /fgoto (.+);/, "self.next_state = Some(String::from(\"#{state}\"));"
    end

    # byebug if state

    # "foobar".gsub(/(o+)/){|s|s+'ball'}
  end
end
