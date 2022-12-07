class ApplicationController < ActionController::API
  before_action :set_url_list

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
end
