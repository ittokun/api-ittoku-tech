class ApplicationController < ActionController::API
  before_action :set_url_list

  rescue_from ActiveRecord::RecordNotFound, with: :record_not_found

  # GET /
  def index
    render(pretty_json: @url_list)
  end

  private

  def set_url_list
    @url_list = {
      post_list_url: "https://#{ENV['DOMAIN']}/posts{?page}",
      post_detail_url: "https://#{ENV['DOMAIN']}/posts/{id}",
      search_posts_url: "https://#{ENV['DOMAIN']}/search/posts?q={query}{&type,page}"
    }
  end

  def record_not_found(e)
    render(pretty_json: { message: "#{e.model} Not Found" }, status: 404)
  end
end
