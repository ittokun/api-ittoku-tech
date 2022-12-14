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
    @post.content = 'x' * 100_001
    assert_not @post.valid?
  end

  test 'comments includes @comment' do
    @comment = comments(:one)
    assert_includes @post.comments, @comment
  end

  test 'comments should be dependent destroyed' do
    assert_difference('Comment.count', -1) do
      @post.destroy
    end
  end

  test '.search(keyword) count is 1' do
    assert_equal @post, Post.search(@post.title).first
  end

  test '.search(keyword) should be ignore case' do
    assert_equal @post, Post.search(@post.title.upcase).first
  end

  test '.search(keyword) should be return []' do
    assert Post.search('').empty?
    assert Post.search('search not found').empty?
  end
end
