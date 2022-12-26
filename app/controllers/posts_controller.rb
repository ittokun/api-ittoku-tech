class PostsController < ApplicationController
  before_action :set_post

  # GET /posts
  def index
    if @posts['posts'].any?
      render(pretty_json: @posts, status: 200)
    else
      url_not_found
    end
  end

  # GET /posts/:id
  def show
    render(pretty_json: @post, status: 200)
  end

  # POST /posts
  def create
    if @post.save
      render(pretty_json: @post, status: 200)
    else
      render(pretty_json: @post.errors, status: 422)
    end
  end

  # PATCH /posts/:id
  def update
    if @post.update(post_params)
      render(pretty_json: @post, status: 200)
    else
      render(pretty_json: @post.errors, status: 422)
    end
  end

  # DELETE /posts/:id
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

  def set_post
    case action_name
    when 'index'
      @posts = {}
      @posts['post_count'] = Post.count
      @posts['posts'] = Post.page(params[:page])
    when 'show'    then @post  = Post.find(params[:id])
    when 'create'  then @post  = Post.new(post_params)
    when 'update'  then @post  = Post.find(params[:id])
    when 'destroy' then @post  = Post.find(params[:id])
    end
  end
end
