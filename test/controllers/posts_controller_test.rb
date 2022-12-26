require 'test_helper'

class PostsControllerTest < ActionDispatch::IntegrationTest
  test 'index should return 200' do
    get posts_url
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal 1, res['post_count']
    assert_equal posts(:one).id, res['posts'][0]['id']
  end

  test 'index should return 404' do
    get posts_url, params: { page: 100 }
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end

  test 'show should return 200' do
    post = posts(:one)
    get post_url(post)
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal post.id, res['id']
  end

  test 'show should return 404' do
    get post_url('hogebar')
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end

  test 'create should return 200' do
    post posts_url, params: { post: { title: 'test', content: 'hello' } }
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal 'hello', res['content']
  end

  test 'create should return 422' do
    post posts_url, params: { post: { title: '', content: '' } }
    assert_response :unprocessable_entity
  end

  test 'update should return 200' do
    post = posts(:one)
    patch post_url(post), params: { post: { title: post.title, content: post.content } }
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal post.id, res['id']
  end

  test 'update should return 404' do
    patch post_url('hogebar'), params: { post: { title: 'post', content: 'not found' } }
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end

  test 'update should return 422' do
    post = posts(:one)
    patch post_url(post), params: { post: { title: '', content: '' } }

    assert_response :unprocessable_entity
  end

  test 'destroy should return 200' do
    post = posts(:one)
    delete post_url(post)
    res = JSON.parse(response.body)

    assert_response :success
    assert_equal post.id, res['id']
  end

  test 'destroy should return 404' do
    delete post_url('hogebar')
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end
end
