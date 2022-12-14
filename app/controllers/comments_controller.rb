class CommentsController < ApplicationController
  before_action :set_post_comments

  def index
    render(pretty_json: @comments, status: 200)
  end

  def create
    if @comment.save
      render(pretty_json: @comment, status: 200)
    else
      render(pretty_json: @post.errors, status: 422)
    end
  end

  def destroy
    if @comment.destroy
      render(pretty_json: @comment, status: 200)
    else
      render(pretty_json: @post.errors, status: 422)
    end
  end

  private

  def comment_params
    params.require(:comment).permit(:content)
  end

  def set_post_comments
    case action_name
    when 'index'
      @post = Post.eager_load(:comments).find(params[:post_id])
      @comments = @post.comments
    when 'create'
      @post = Post.find(params[:post_id])
      @comment = @post.comments.new(comment_params)
    when 'destroy'
      @post = Post.find(params[:post_id])
      @comment = @post.comments.find(params[:id])
    end
  end
end
