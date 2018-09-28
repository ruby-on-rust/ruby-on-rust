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
    # wrapper
    @code = """
            {
                #{@code}
            }
            """

    # fhold
    @code.gsub! 'fhold;', 'self.p -= 1;' # TODO should use a flag like is_holding
  end
end
