module Authentication
  def is_authenticated
    jwt_result = helpers.decode_jwt(cookies[:token])
    if jwt_result[:success]
      user = User.find(jwt_result[:user]['user_id'])
      redirect_to user
    end
  end

  def sign_out
    cookies.delete :token
  end

  def decode_jwt(token)
    begin
      decoded_token = JWT.decode token, ENV["JWT_SECRET"], true, { algorithm: 'HS256' }
      return {
        user: decoded_token[0],
        success: true
      }
    rescue => error
      puts error
      return {
        user: nil,
        success: false
      }
    end
  end

  def encode_jwt(payload)
    JWT.encode payload, ENV["JWT_SECRET"], 'HS256'
  end
end
