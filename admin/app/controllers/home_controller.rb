class HomeController < ApplicationController
  def index
    cookies[:hello] = "world"
  end
end
