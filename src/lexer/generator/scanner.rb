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
                $machines[pattern] or raise "unknown pattern :#{pattern}"
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
                byebug unless p[:pattern]
                action = p[:action]

            """
                //
                // pattern: #{p.inspect}
                //

                let pattern_regex = Regex::new(r\"^(?ms:#{p[:pattern].regex})\").unwrap(); // TODO PERFORMANCE

                println!(\"      matching pattern with regex: {:?}\", pattern_regex);

                let captures = pattern_regex.captures(&slice_from_current_pos);
                match captures {
                    None => {
                        println!(\"        matched none\");
                    },
                    Some(capture) => {
                        let match_ = capture.get(0).unwrap();
                        let matched_slice_ = String::from(match_.as_str());
                        let matched_slice_len = matched_slice_.len() as isize;

                        if matched_slice_len > longest_match_len {
                            longest_match_len = matched_slice_len;
                            matched_slice = Some(matched_slice_.clone());
                            matched_action_id = #{action.id};
                        }

                        println!(\"        matched slice: {:?}\", matched_slice.clone());
                        // println!(\"        DEBUGGING CAPTURE: capture: {:?}\", capture);
                    }
                }
            """
            }.join}

            if longest_match_len == -1 {
                panic!(\"unreachable! matched nothing\");
            }

        },
        """
  end
end
