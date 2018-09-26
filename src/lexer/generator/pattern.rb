# 
# TODO NOTE
# pattern is a what
# 
class Pattern
  attr_reader :regex

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
  def initialize *p
    @regex =  case p.size
              when 1
                Pattern.parse_segment p[0]
              when 2
                rule_1 = Pattern.parse_segment p[0]
                rule_2 = Pattern.parse_segment p[1]

                "(#{rule_1})(#{rule_2})"
              when 3
                raise 'unreachable' unless p[1] == :-

                rule_1 = Pattern.parse_segment p[0]
                rule_2 = Pattern.parse_segment p[2]

                "[(#{Pattern.parse_segment rule_1})&&[^(#{Pattern.parse_segment rule_2})]]"
              else
                raise 'unreachable'
              end
  end

  private

    # returns a regex as string
    def self.parse_segment pattern
      case pattern
      when Pattern
        pattern.regex
      when Symbol
        raise "unknown machine :#{pattern}" unless machine = $machines[pattern]
        machine.regex
      when String
        pattern
      when Array
        '[' + pattern.map{|p| "(#{parse_segment p})" }.join + ']'
      else
        raise "unreachable with pattern #{pattern.inspect}"
      end
    end

end
