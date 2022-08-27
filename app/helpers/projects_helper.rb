module ProjectsHelper
	def format_project(project)
		project.name = project.name.gsub('_', ' ')
		return project
	end
	def format_projects(projects)
		projects.each do |project|
			format_project(project)
		end
		return projects
	end
	def unformat_project(project)
		project.name = project.name.gsub(' ', '_')
		return project
	end
	def create_project_params(old_params)
		new_params = old_params.clone
    	new_params[:name] = new_params[:name].gsub(' ', '_')
		return new_params
	end
end
