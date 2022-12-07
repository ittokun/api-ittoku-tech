require "test_helper"

class SearchesControllerTest < ActionDispatch::IntegrationTest
  # expected response
  # {
  #   "search_count": "1",
  #   "items": [
  #     {
  #       "id": "1234567890",
  #       "title": "test",
  #       "content": "# hello",
  #       "created_at": DATETIME,
  #       "updated_at": DATETIME
  #     }
  #   ]
  # }
  test 'posts: should return 200' do
    post = posts(:one)
    get search_posts_url, params: { q: 'post' }
    assert_response :success
    res = JSON.parse(response.body)
    assert_equal 1, res['search_count']
    assert_equal post.id, res['items'][0]['id']
  end

  test 'posts: should return 404' do
    posts(:one)
    get search_posts_url
    assert_response :not_found
    get search_posts_url, params: { q: '' }
    assert_response :not_found
    get search_posts_url, params: { q: 'post', page: '100' }
    assert_response :not_found
  end
end
