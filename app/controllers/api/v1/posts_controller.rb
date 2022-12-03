class Api::V1::PostsController < ApplicationController
  before_action :set_user

  rescue_from ActiveRecord::RecordNotFound, with: :record_not_found

  # GET /api/v1/posts
  def index
    render(pretty_json: @posts, status: 200)
  end

  # GET /api/v1/posts/:id
  def show
    render(pretty_json: @post, status: 200)
  end

  # POST /api/v1/posts
  def create
    if @post.save
      render(pretty_json: @post, status: 200)
    else
      render(pretty_json: @post.errors, status: 422)
    end
  end

  # PATCH /api/v1/posts/:id
  def update
    if @post.update(post_params)
      render(pretty_json: @post, status: 200)
    else
      render(pretty_json: @post.errors, status: 422)
    end
  end

  # DELETE /api/v1/posts/:id
  def destroy
    if @post.destroy
      render(pretty_json: @post, status: 200)
    else
      render(pretty_json: @post.errors, status: 422)
    end
  end

  private

  def post_params
    params.require(:post).permit(:title, :content)
  end

  def set_user
    case action_name
    when 'index'   then @posts = Post.all
    when 'show'    then @post  = Post.find(params[:id])
    when 'create'  then @post  = Post.new(post_params)
    when 'update'  then @post  = Post.find(params[:id])
    when 'destroy' then @post  = Post.find(params[:id])
    end
  end

  def record_not_found
    render nothing: true, status: 404
  end
end
