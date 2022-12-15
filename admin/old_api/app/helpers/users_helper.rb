module UsersHelper
  include Authentication
  def redirect_with_authentication
    jwt_result = decode_jwt(cookies[:token])
    if jwt_result[:success]
      begin
        user = User.find(jwt_result[:user]['user_id'])
        return user
      rescue
        sign_out
      end
    end
    return nil
  end
end
