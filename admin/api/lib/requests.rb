require 'net/http'

module Requests
	def http_get(url, token=nil)
		return http_request(url, "GET")
	end
	def http_post(url, body, token=nil)
		return http_request(url, "POST", body)
	end
	def http_put(url, body, token=nil)
		return http_request(url, "PUT", body)
	end
	def http_delete(url, token=nil)
		return http_request(url, "DELETE")
	end
	def http_graphql(url, query_string, variables, token=nil)
		data = {
			"query": query_string,
			"variables": variables.to_json
		}.to_json
		return http_request(url, "POST", data, token)
	end
	private
	def http_request(url, method, body = nil, token=nil)
		uri = URI(url)
		headers = { 'Authorization' => 'Bearer ' + token }
		
		http = Net::HTTP.new(uri.host, uri.port)
		http.use_ssl = true


		case method
		when "GET"
			req = Net::HTTP::Get.new(uri.path, headers)
		when "POST"
			req = Net::HTTP::Post.new(uri.path, headers)
		when "PUT"
			req = Net::HTTP::Put.new(uri.path, headers)
		when "DELETE"
			req = Net::HTTP::Delete.new(uri.path, headers)
		else
		  	raise "You passed something other than a string"
		end

		
		req.body = body
		
		res = http.request(req)
		return res
	end
end