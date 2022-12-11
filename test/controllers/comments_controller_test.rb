require "test_helper"

class CommentsControllerTest < ActionDispatch::IntegrationTest
  test 'create: should return 200' do
    post post_comments_url(comments(:one).post), params: { comment: { content: 'hello' } }
    assert_response :success
  end

  test 'create: should return 404' do
    post post_comments_url(1234), params: { comment: { content: 'hello'} }
    assert_response :not_found
  end

  test 'create: should return 422' do
    post post_comments_url(comments(:one).post), params: { comment: { content: '' } }
    assert_response :unprocessable_entity
  end

  test 'destroy: should return 200' do
    comment = comments(:one)
    delete post_comment_url(comment.post, comment)
    assert_response :success
  end

  test 'destroy: should return 404' do
    delete post_comment_url(1234, 5678)
    assert_response :not_found
  end
end
