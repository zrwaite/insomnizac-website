json.extract! user, :id, :email, :password_digest, :confirmed, :created_at, :updated_at
json.url user_url(user, format: :json)
