class SearchesController < ApplicationController
  before_action :set_search

  # GET /search/posts?q=asdf
  def posts
    if @search['items'].any?
      render(pretty_json: @search, status: 200)
    else
      render(pretty_json: { message: 'Post Not Found' }, status: 404)
    end
  end

  private

  def set_search
    result = Post.search(params[:q])
    @search = {}

    @search['search_count'] = result.length
    @search['items'] = result.page(params[:page])
  end
end
