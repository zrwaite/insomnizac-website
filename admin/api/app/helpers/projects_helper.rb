require 'uri'
require 'net/http'

module ProjectsHelper
	include Authentication

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

	def repository_query
		return "
		query ($name: String!, $owner: String!) { 
			repository(name: $name, owner: $owner) {
				description
				languages(first: 10, orderBy: {
					direction: DESC,
					field: SIZE
				}) 
				{
					totalSize
					edges {
						size
						node {
							color
							name
						}
					}
				}
			}
		}
		"
	end
	def get_project_names(github_name)
		username = nil
		project_name = nil
		if !github_name.include?("/")
		  	username = "zrwaite"
		  	project_name = github_name
		else
		  	names = github_name.split("/")
		  	username = names[0]
		  	project_name = names[1]
		end
		return {
			"owner": username, 
			"name": project_name
		}
	end
end
