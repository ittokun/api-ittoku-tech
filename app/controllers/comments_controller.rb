class CommentsController < ApplicationController
  before_action :set_comment_post

  rescue_from ActiveRecord::RecordNotFound, with: :record_not_found

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

  def set_comment_post
    case action_name
    when 'create'
      @post = Post.find(params[:post_id])
      @comment = @post.comments.new(comment_params)
    when 'destroy'
      @post = Post.find(params[:post_id])
      @comment = @post.comments.find(params[:id])
    end
  end

  def record_not_found
    render nothing: true, status: 404
  end
end
