require 'uri'
require 'net/http'

module ProjectsHelper
	def get_github()
		uri = URI('https://jsonplaceholder.typicode.com/posts')
		res = Net::HTTP.post_form(uri, 'title' => 'foo', 'body' => 'bar', 'userID' => 1)
		puts res.body  if res.is_a?(Net::HTTPSuccess)
	end

	def decode_jwt(token)
		begin
			decoded_token = JWT.decode token, ENV["JWT_SECRET"], true, { algorithm: 'HS256' }
			return {
				user: decoded_token[0],
				success: true
			}
		rescue => error
			puts error
			return {
				user: nil,
				success: false
			}
		end
	end
end
