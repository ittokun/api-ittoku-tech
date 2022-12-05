require "test_helper"

class Api::V1::SearchesControllerTest < ActionDispatch::IntegrationTest
  test 'posts: should return 200' do
    posts(:one)
    get api_v1_search_posts_url, params: { q: 'post' }
    assert_response :success
  end

  test 'posts: should return 404' do
    posts(:one)
    get api_v1_search_posts_url
    assert_response :not_found
    get api_v1_search_posts_url, params: { q: '' }
    assert_response :not_found
    get api_v1_search_posts_url, params: { q: 'post', page: '100' }
    assert_response :not_found
  end
end
