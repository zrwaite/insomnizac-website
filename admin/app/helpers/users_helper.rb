require 'jwt'
module UsersHelper
	def encode_jwt(payload)
		return JWT.encode payload, ENV["JWT_SECRET"], 'HS256'
	end

end
