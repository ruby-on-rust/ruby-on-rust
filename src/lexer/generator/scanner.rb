class Scanner
  attr_reader :patterns

  def initialize name
    @name = name
    @patterns = []

    $scanners[@name] = self
  end

  # 
  # adds a pattern-action pair
  # 
  # pattern
  #   - a Pattern object
  #   - :machine_name
  # 
  # action
  #   nil           -> {}
  #   :action_name
  #   code block like
  #       let a = 1;
  # 
  def p pattern, action
    pattern = case pattern
              when Symbol
                $machines[pattern]
              when Pattern
                pattern
              else
                raise 'unreachable!'
              end

    action =  case action
              when Symbol
                Action.find_by_name action
              when nil
                Action.find_by_name :nil
              when String
                new_action = Action.new action, nil
                new_action
              end

    @patterns << {
      pattern: pattern,
      action: action
    }
  end

  def code
        """
        \"#{@name}\" => {
            //
            // getting the longest match
            //
            let mut longest_match_len: isize = -1; // TODO HACKING init as -1 since there would be matched with len being 0
            let mut longest_match_action_key: isize = -1;

            let slice_from_current_pos: String = self.input.chars().skip(self.p).collect();

            #{@patterns.map{|p|
                regex = p[:pattern].regex
                action = p[:action]

            """
                //
                // pattern: #{p.inspect}
                //

                let pattern_regex = Regex::new(r\"#{regex}\").unwrap(); // TODO PERFORMANCE

                let captures = pattern_regex.captures(&slice_from_current_pos);
                match captures {
                    None => {},
                    Some(capture) => {
                        let match_ = capture.get(0).unwrap();
                        let matched_slice = String::from(match_.as_str());
                        let matched_slice_len = matched_slice.len() as isize;

                        if matched_slice_len > longest_match_len {
                            longest_match_len = matched_slice_len;
                            longest_match_action_key = #{action.id};
                        }

                        // println!(\"        ***** matched str: {:?}\", matched_slice);
                        // println!(\"        DEBUGGING CAPTURE: capture: {:?}\", capture);
                    }
                }
            """
            }.join}

            match longest_match_action_key {
                -1 => {},
                #{$actions.map{ |id, action|
                    """
                #{id} => #{action.code},
                    """
                }.join}
                _ => { panic!(\"unreachable!\"); }
            }
        },
        """
  end
end
