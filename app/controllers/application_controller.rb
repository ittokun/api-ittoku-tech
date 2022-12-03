class ApplicationController < ActionController::API
  before_action :set_url_list

  # GET /
  def index
    render(pretty_json: @url_list)
  end

  private

  def set_url_list
    @url_list = {
      post_list_url: "#{ENV['DOMAIN']}/api/v1/posts",
      post_detail_url: "#{ENV['DOMAIN']}/api/v1/posts/{id}",
      search_posts_url: "#{ENV['DOMAIN']}/api/v1/search/posts?q={query}"
    }
  end
end
