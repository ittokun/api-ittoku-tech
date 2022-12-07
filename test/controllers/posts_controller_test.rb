require 'test_helper'

class PostsControllerTest < ActionDispatch::IntegrationTest
  # expected response
  # {
  #   "post_count": "1",
  #   "posts": [
  #     {
  #       "id": "1234567890",
  #       "title": "test",
  #       "content": "# hello",
  #       "created_at": DATETIME,
  #       "updated_at": DATETIME
  #     }
  #   ]
  # }
  test 'index: should return 200' do
    get posts_url
    assert_response :success
    post = posts(:one)
    res = JSON.parse(response.body)
    assert_equal 1, res['post_count']
    assert_equal post.id, res['posts'][0]['id']
  end

  test 'index: should return 404' do
    get posts_url, params: { page: 100 }
    assert_response :not_found
  end

  test 'show: should return 200' do
    get post_url(posts(:one))
    assert_response :success
  end

  test 'show: should return 404' do
    get post_url('hogebar')
    assert_response :not_found
  end

  test 'create: should return 200' do
    post posts_url, params: { post: { title: 'test', content: 'hello' } }
    assert_response :success
  end

  test 'create: should return 422' do
    post posts_url, params: { post: { title: '', content: '' } }
    assert_response :unprocessable_entity
  end

  test 'update: should return 200' do
    post = posts(:one)
    patch post_url(post), params: { post: { title: post.title, content: post.content } }
    assert_response :success
  end

  test 'update: should return 404' do
    patch post_url('hogebar'), params: { post: { title: 'post', content: 'not found' } }
    assert_response :not_found
  end

  test 'update: should return 422' do
    post = posts(:one)
    patch post_url(post), params: { post: { title: '', content: '' } }
    assert_response :unprocessable_entity
  end

  test 'destroy: should return 200' do
    post = posts(:one)
    delete post_url(post)
    assert_response :success
  end

  test 'destroy: should return 404' do
    delete post_url('hogebar')
    assert_response :not_found
  end

  # test 'destroy: should return 422' do
  #   post = posts(:one)
  #   delete post_url(post)
  #   assert_response :not_found
  # end
end
