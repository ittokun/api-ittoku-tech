require "test_helper"

class CommentsControllerTest < ActionDispatch::IntegrationTest
  test 'index should return 200' do
    comment = comments(:one)
    get post_comments_url(comment.post)
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal comment.content, res[0]['content']
  end

  test 'index should return 404' do
    get post_comments_url(1234)
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end

  test 'create should return 200' do
    comment = comments(:one)
    post post_comments_url(comment.post), params: { comment: { content: 'hello' } }
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal 'hello', res['content']
  end

  test 'create should return 404' do
    post post_comments_url(1234), params: { comment: { content: 'hello' } }
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end

  test 'create should return 422' do
    post post_comments_url(comments(:one).post), params: { comment: { content: '' } }
    assert_response :unprocessable_entity
  end

  test 'destroy should return 200' do
    comment = comments(:one)
    delete post_comment_url(comment.post, comment)
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal comment.content, res['content']
  end

  test 'destroy should return 404' do
    comment = comments(:one)
    delete post_comment_url(1234, comment)
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']

    delete post_comment_url(comment.post, 1234)
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end
end
