class ProjectsController < ApplicationController
  include Authentication
  before_action :set_project, only: %i[ show update destroy ]
  before_action :auth

  # GET /projects
  def index
    @projects = Project.all
    @projects.each do |project|
      project.update_attribute :skills, []
      project.skill_ids.each do |skill_id|
        project.skills.push(Skill.find(skill_id))
      end
    end

    render json: @projects
  end

  # GET /projects/1
  def show
    helpers.get_github
    render json: @project
  end

  # POST /projects
  def create
    @project = Project.new(project_params)

    if @project.save
      render json: @project, status: :created, location: @project
    else
      render json: @project.errors, status: :unprocessable_entity
    end
  end

  # PATCH/PUT /projects/1
  def update
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
    def project_params
      # params.fetch(:project, {})
      params.require(:project).permit(:name, :description, :github_name, :slug)
    end

    def auth
      redirect = helpers.authenticate_redirect
      if redirect != nil 
        redirect_to redirect
      end
    end

end
