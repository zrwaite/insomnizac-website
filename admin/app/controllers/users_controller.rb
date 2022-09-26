class UsersController < ApplicationController
  before_action :set_user, only: %i[ show edit update destroy ]

  # GET /users or /users.json
  def index
    @users = User.all
  end

  # GET /users/1 or /users/1.json
  def show
  end

  # GET /users/new
  def new
    @user = User.new
  end

  # GET /users/1/edit
  def edit
  end

  # POST /users or /users.json
  def signup
    params = user_all_params
    @user = User.new
    respond_to do |format|
      if params[:password] != params[:confirm_password]
        @user.errors.add(:password, ': passwords dont match')
        format.html { render :new, status: :bad_request }
        format.json { render json: @user.errors, status: :bad_request }
      elsif User.find_by(email: params[:email])
        @user.errors.add(:email, ': email in use')
        format.html { render :new, status: :bad_request }
        format.json { render json: @user.errors, status: :bad_request }
      else 
        @user.email = params[:email]
        @user.password = params[:password]
        if @user.save
          # Success
          @user.errors.clear
          cookies[:token] = helpers.encode_jwt({
            user_id: @user.id
          })
          puts 'redirecting'
          # redirect_to home_url
          format.html do
            redirect_to home_url
          end
          format.json { render :new, status: :created }
        else
          format.html { render :new, status: :bad_request }
          format.json { render json: @user.errors, status: :bad_request }
        end
      end
    end
  end

  # GET /users/login or /users/login.json
  def login
    @user = User.new
  end

  def login_handler
    @user = User.new

    db_user = User.find_by(email: params[:email])
    params_user = User.new(login_params)

    respond_to do |format|
      if !db_user 
        @user.errors.add(":email", ': User not found')
        puts 'User not found'
        format.html { render :login, status: :bad_request }
        format.json { render json: @user.errors, status: :bad_request }
      else
        if db_user.password == params_user.password
          puts 'Success!'
          cookies[:token] = helpers.encode_jwt({
            user_id: db_user.id
          })
          format.html { redirect_to home_url, notice: "Login succesful." }
          format.json { render json: @user, status: :created }
        else
          @user.errors.add(":password", ': Invalid')
          format.html { render :login, status: :bad_request }
          format.json { render json: @user.errors, status: :bad_request }
        end
      end

    end

    # if user.password == login_params[:password] 
    #   puts 'wtf'
    # end

    
  end

  # PATCH/PUT /users/1 or /users/1.json
  def update
    respond_to do |format|
      if @user.update(user_params)
        format.html { redirect_to user_url(@user), notice: "User was successfully updated." }
        format.json { render :show, status: :ok, location: @user }
      else
        format.html { render :edit, status: :unprocessable_entity }
        format.json { render json: @user.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /users/1 or /users/1.json
  def destroy
    @user.destroy

    respond_to do |format|
      format.html { redirect_to home_url, notice: "User was successfully destroyed." }
      format.json { head :no_content }
    end
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_user
      @user = filter_user_params(User.find(params[:id]))
    end

    def filter_user_params(user)
      user.password_digest = ''
      return user
    end

    # Only allow a list of trusted parameters through.
    def user_params
      params.require(:user).permit(:email)
    end

    # Only allow a list of trusted parameters through.
    def user_all_params
      params.require(:user).permit(:email, :password, :confirm_password)
    end

    def login_params
      params.require(:email)
      params.require(:password)
      params.permit(:password, :email)
    end
end
