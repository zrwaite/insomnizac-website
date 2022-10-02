require 'uri'
require 'net/http'

module ProjectsHelper
	include Authentication
	def get_github
		uri = URI('https://jsonplaceholder.typicode.com/posts')
		res = Net::HTTP.post_form(uri, 'title' => 'foo', 'body' => 'bar', 'userID' => 1)
		puts res.body  if res.is_a?(Net::HTTPSuccess)
	end

	def authenticate
		jwt_result = decode_jwt(cookies[:token])
		if !jwt_result[:success]
			redirect_to users_login_url
		else
			begin
				user = User.find(jwt_result[:user]['user_id'])
				unless user.confirmed
					redirect_to user
				end
			rescue => error
				sign_out
				redirect_to users_login_url
			end
		end
	end

end
