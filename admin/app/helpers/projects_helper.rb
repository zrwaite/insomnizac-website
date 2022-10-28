require 'uri'
require 'net/http'

module ProjectsHelper
	include Authentication
	def get_github
		uri = URI('https://jsonplaceholder.typicode.com/posts')
		res = Net::HTTP.post_form(uri, 'title' => 'foo', 'body' => 'bar', 'userID' => 1)
		puts res.body  if res.is_a?(Net::HTTPSuccess)
	end

	def authenticate_redirect
		jwt_result = decode_jwt(cookies[:token])
		if !jwt_result[:success]
			return users_login_url
		else
			begin
				user = User.find(jwt_result[:user]['user_id'])
				unless user.confirmed
					return user
				end
			rescue => error
				sign_out
				return users_login_url
			end
		end
		return nil
	end

end
