require "test_helper"

class CommentTest < ActiveSupport::TestCase
  setup do
    @comment = comments(:one)
  end

  test 'should be invalid' do
    assert_not Comment.new.valid?
  end

  test 'should be valid' do
    assert @comment.valid?
  end

  test 'content should be required' do
    @comment.content = ''
    assert_not @comment.valid?
  end

  test 'content max length should be 1000' do
    @comment.content = 'x' * 1001
    assert_not @comment.valid?
  end

  test 'post_id should be required' do
    @comment.post_id = nil
    assert_not @comment.valid?
  end

  test 'post_id should be correct reference' do
    @comment.post_id = 1234
    assert_not @comment.valid?
  end
end
