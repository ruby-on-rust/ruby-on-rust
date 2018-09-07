class Pattern
  # 
  # takes rules, returns a string for building regex
  # 
  # m rule
  # m '\n'
  # m '[ \t]'
  # m :other_machine
  # 
  # m rule_1, rule_2      => rule_1 concat rule_2
  # m rule_1, :-, rule_2  => rule_1 - rule_2
  # m [rule_1, rule_2...] => rule_1 or rule_2 or ...
  # 
  def self.parse *p
    regex = case p.size
    when 1
      parse_segment p[0]
    when 2
      rule_1 = parse_segment p[0]
      rule_2 = parse_segment p[1]

      "(#{rule_1})(#{rule_2})"
    when 3
      raise 'unreachable' unless p[1] == :-

      rule_1 = parse_segment p[0]
      rule_2 = parse_segment p[2]

      "[(#{parse_segment rule_1})&&[^(#{parse_segment rule_2})]]"
    else
      raise 'unreachable'
    end
  end

  private

  def self.parse_segment pattern
    case pattern
    when Symbol
      raise "unknown machine :#{pattern}" unless machine = $machines[pattern]
  
      machine[:regex]
    when String
      pattern
    when Array
      '[' + pattern.map{|p| "(#{parse_segment p})" }.join + ']'
    else
      raise 'unreachable'
    end
  end
end
