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

  test 'content should be presence' do
    @post.content = nil
    assert_not @post.valid?
  end

  test 'title max length should be 1000' do
    @post.title = 'x' * 1001
    assert_not @post.valid?
  end

  test 'content max length should be 100000' do
    @post.content = 'x' * 100001
    assert_not @post.valid?
  end
end
