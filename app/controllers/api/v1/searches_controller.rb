class Api::V1::SearchesController < ApplicationController
  before_action :set_search

  # GET /api/v1/search/posts?q=asdf
  def posts
    if @search.any?
      render(pretty_json: @search, status: 200)
    else
      render(pretty_json: { message: 'Post Not Found' }, status: 404)
    end
  end

  private

  def set_search
    @search = Post.search(params[:q]).page(params[:page])
  end
end
