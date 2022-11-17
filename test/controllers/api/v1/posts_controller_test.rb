require 'test_helper'

class Api::V1::PostsControllerTest < ActionDispatch::IntegrationTest
  test 'index: should return 200' do
    get api_v1_posts_url
    assert_response :success
  end

  test 'show: should return 200' do
    get api_v1_post_url(posts(:one))
    assert_response :success
  end

  test 'show: should return 404' do
    get api_v1_post_url('hogebar')
    assert_response :not_found
  end

  test 'create: should return 200' do
    post api_v1_posts_url, params: { post: { title: 'test', content: 'hello' } }
    assert_response :success
  end

  test 'create: should return 422' do
    post api_v1_posts_url, params: { post: { title: '', content: '' } }
    assert_response :unprocessable_entity
  end

  test 'update: should return 200' do
    post = posts(:one)
    patch api_v1_post_url(post), params: { post: { title: post.title, content: post.content } }
    assert_response :success
  end

  test 'update: should return 404' do
    patch api_v1_post_url('hogebar'), params: { post: { title: 'post', content: 'not found' } }
    assert_response :not_found
  end

  test 'update: should return 422' do
    post = posts(:one)
    patch api_v1_post_url(post), params: { post: { title: '', content: '' } }
    assert_response :unprocessable_entity
  end

  test 'destroy: should return 200' do
    post = posts(:one)
    delete api_v1_post_url(post) 
    assert_response :success
  end

  test 'destroy: should return 404' do
    delete api_v1_post_url('hogebar')
    assert_response :not_found
  end

  # test 'destroy: should return 422' do
  #   post = posts(:one)
  #   delete api_v1_post_url(post)
  #   assert_response :not_found
  # end
end
