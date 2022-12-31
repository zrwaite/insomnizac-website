class UsersController < ApplicationController
  include Authentication
  before_action :set_user, only: %i[ show update destroy ]

  # GET /users
  def index
    @users = User.all

    render json: @users
  end

  # GET /users/1
  def show
    render json: @user
  end

  # POST /users or /users.json
  def signup
    params = user_all_params
    @user = User.new
    if params[:password] != params[:confirm_password]
      @user.errors.add(:password, ': passwords dont match')
      render json: @user.errors, status: :bad_request
    elsif User.find_by(email: params[:email])
      @user.errors.add(:email, ': email in use')
      render json: @user.errors, status: :bad_request
    else 
      @user.email = params[:email]
      @user.password_digest = BCrypt::Password.create(params[:password])
      if @user.save
        # Success
        @user.errors.clear
        cookies[:token] = encode_jwt({
          user_id: @user.id
        })
        render json: @user, status: :created, location: @user
      else
        render json: @user.errors, status: :unprocessable_entity
      end
    end
  end

  def login_handler
    @user = User.new
    login_body = JSON.parse(request.body.read)
    db_user = User.find_by(email: login_body["email"])
    if !db_user 
      @user.errors.add(":email", ': User not found')
      render json: @user.errors, status: :bad_request
    else
      if BCrypt::Password.new(db_user.password_digest) == login_body["password"]
        puts 'Success!'
        render json: {
          user: filter_user_params(db_user),
          token: encode_jwt({
            user_id: db_user.id
          })
        }, status: :created, location: @user
      else
        @user.errors.add(":password", ': Invalid')
      render json: @user.errors, status: :bad_request
      end
    end
  end

  # PATCH/PUT /users/1
  def update
    if @user.update(user_params)
      render json: @user
    else
      render json: @user.errors, status: :unprocessable_entity
    end
  end

  # DELETE /users/1
  def destroy
    @user.destroy
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_user
      @user = filter_user_params(User.find(params[:id]))
    end

    def filter_user_params(user)
      user.password_digest = ''
      user
    end

    # Only allow a list of trusted parameters through.
    def user_params
      # params.fetch(:user, {})
      params.require(:user).permit(:email)
    end

    def user_all_params
      params.require(:user).permit(:email, :password, :confirm_password)
    end

    def login_params
      params.require(:email)
      params.require(:password)
      params.permit(:password, :email)
    end
end
