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

  # test 'should return 404' do
  #   get api_v1_post_url('hogebar')
  #   assert_response :not_found
  # end

  test 'should get create' do
    post = posts(:one)
    post api_v1_posts_url, params: { post: { title: post.title, body: post.body } }
    assert_response :success
  end

  test 'should get update' do
    post = posts(:one)
    patch api_v1_post_url(post), params: { post: { title: post.title, body: post.body } }
    assert_response :success
  end

  test 'should get destroy' do
    post = posts(:one)
    delete api_v1_post_url(post) 
    assert_response :success
  end
end
