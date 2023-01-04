class ProjectsController < ApplicationController
  include Requests
  include Authentication
  before_action :set_project, only: %i[ show update destroy ]
  before_action :auth, only: %i[ create update destroy ]

  # GET /projects
  def index
    @projects = Project.all
    @projects.each do |project|
      project.update_attribute :skills, []
      project.skill_ids.each do |skill_id|
        project.skills.push(Skill.find(skill_id))
      end
    end

    render json: @projects, methods: ['skills']
  end

  # GET /projects/1
  def show
    render json: @project, methods: ['skills']
  end

  # POST /projects
  def create
    project_params = JSON.parse(request.body.read)
    @project = Project.new(project_params)

    if @project.save
      render json: @project, status: :created, location: @project, methods: ['skills']
    else
      render json: @project.errors, status: :unprocessable_entity
    end
  end

  # PATCH/PUT /projects/1
  def update
    project_params = JSON.parse(request.body.read)
    if @project.update(project_params)
      render json: @project, methods: ['skills']
    else
      render json: @project.errors, status: :unprocessable_entity
    end
  end

  # DELETE /projects/1
  def destroy
    @project.destroy
  end

  def load_from_github
    @project = Project.find_by!(slug: params[:slug])
    all_skills = Skill.all

    github_data = http_graphql("https://api.github.com/graphql", helpers.repository_query, helpers.get_project_names(@project.github_name), ENV['GITHUB_ACCESS_TOKEN'])

    parsed = JSON.parse(github_data.body)
    repository = parsed['data']['repository']

    repository['languages']['edges'].each do |language|
      skill = all_skills.find { |skill| skill.name == language['node']['name'] }
      if skill && !@project.skill_ids.include?(skill.id)
        @project.skill_ids.push(skill.id)
      end
    end

    get_project_skills(@project)

    render json: @project, methods: ['skills']
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_project
      @project = Project.find_by!(slug: params[:slug])
      get_project_skills(@project)
    end

    def get_project_skills(project)
      project.update_attribute :skills, []
      project.skill_ids.each do |skill_id|
        project.skills.push(Skill.find(skill_id))
      end
    end

    # Only allow a list of trusted parameters through.
    # def project_params
    #   # params.fetch(:project, {})
    #   params.require(:project).permit(:name, :description, :github_name, :slug)
    # end

    def auth
      # get the Authorization header
      # split the token into two parts
      begin
        token = request.headers['Authorization'].split(' ').last
      rescue
        raise "Invalid Authentication"
      end

      helpers.authenticate(token)
    end

end
