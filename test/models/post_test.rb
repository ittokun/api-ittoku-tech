require "test_helper"

class PostTest < ActiveSupport::TestCase
  setup do
    @post = posts(:one)
  end

  test 'should be invalid' do
    assert_not Post.new.valid?
  end

  test 'should be valid' do
    assert posts(:one).valid?
  end

  test 'title should be presence' do
    @post.title = nil
    assert_not @post.valid?
  end

  test 'body should be presence' do
    @post.body = nil
    assert_not @post.valid?
  end

  test 'title max length should be 1000' do
    @post.title = 'x' * 1001
    assert_not @post.valid?
  end

  test 'body max length should be 100000' do
    @post.body = 'x' * 100001
    assert_not @post.valid?
  end
end
