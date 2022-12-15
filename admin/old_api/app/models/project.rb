class Project < ApplicationRecord
	attr_accessor :skills

  def initialize()
    @skills = []
  end
end
