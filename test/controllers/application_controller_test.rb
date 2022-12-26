require "test_helper"

class ApplicationControllerTest < ActionDispatch::IntegrationTest
  test 'root should return 200' do
    get root_url
    assert_response :success
  end

  test 'GET /hogebar should return 404' do
    get '/hogebar'
    res = JSON.parse(response.body)

    assert_response :not_found
    assert_equal 'URL Not Found', res['message']
  end
end
