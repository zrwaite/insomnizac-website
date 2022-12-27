require 'uri'
require 'net/http'

module ProjectsHelper
	include Authentication
	def get_github
		uri = URI('https://jsonplaceholder.typicode.com/posts')
		res = Net::HTTP.post_form(uri, 'title' => 'foo', 'body' => 'bar', 'userID' => 1)
		puts res.body  if res.is_a?(Net::HTTPSuccess)
	end

	def authenticate(token)
		jwt_result = decode_jwt(token)
		if !jwt_result[:success]
			raise "Invalid Authentication"
		else
			begin
				user = User.find(jwt_result[:user]['user_id'])
				unless user.confirmed
					raise "Invalid Authentication"
				end
			rescue => error
				sign_out
				raise "Invalid Authentication" + error.to_s
			end
		end
		return nil
	end

end
