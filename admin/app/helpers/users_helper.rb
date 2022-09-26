module UsersHelper
	class Login
		attr_accessor :errors
		def initialize() 
			@errors = Array.new
		end
	end
	
	def new_login() 
		return Login.new
	end
end
