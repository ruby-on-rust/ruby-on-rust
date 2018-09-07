class Scanner
  attr_reader :patterns

  def initialize name
    @name = name
    @patterns = []

    $scanners[@name] = self
  end

  # 
  # pattern
  # 
  # action
  #   nil
  #   :action_name
  #   code block
  def p! pattern, action
    pattern = case pattern
              when Symbol
                Pattern.parse pattern
              when Pattern
                pattern
              end

    @patterns << {
      pattern: pattern,
      action: action
    }
  end
end
