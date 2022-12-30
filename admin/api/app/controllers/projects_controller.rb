class ProjectsController < ApplicationController
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
    helpers.get_github
    render json: @project, methods: ['skills']
  end

  # POST /projects
  def create
    project_params = JSON.parse(request.body.read)
    @project = Project.new(project_params)

    if @project.save
      render json: @project, status: :created, location: @project
    else
      render json: @project.errors, status: :unprocessable_entity
    end
  end

  # PATCH/PUT /projects/1
  def update
    project_params = JSON.parse(request.body.read)
    if @project.update(project_params)
      render json: @project
    else
      render json: @project.errors, status: :unprocessable_entity
    end
  end

  # DELETE /projects/1
  def destroy
    @project.destroy
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_project
      @project = Project.find_by!(slug: params[:slug])
      @project.update_attribute :skills, []
      @project.skill_ids.each do |skill_id|
        @project.skills.push(Skill.find(skill_id))
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

      helpers.authenticate_redirect(token)
    end

end
