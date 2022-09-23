require 'uri'
require 'net/http'

module ProjectsHelper
	def get_github()
		uri = URI('https://jsonplaceholder.typicode.com/posts')
		res = Net::HTTP.post_form(uri, 'title' => 'foo', 'body' => 'bar', 'userID' => 1)
		puts res.body  if res.is_a?(Net::HTTPSuccess)
	end
end
