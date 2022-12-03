class Api::V1::SearchesController < ApplicationController
  before_action :set_search

  rescue_from ActiveRecord::RecordNotFound, with: :record_not_found

  # GET /api/v1/search/posts?q=asdf
  def posts
    render(json: @search, status: 200)
  end

  private

  def set_search
    @search = Post.search!(params[:q])
  end

  def record_not_found
    render nothing: true, status: 404
  end
end
