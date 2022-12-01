class Post < ApplicationRecord
  validates :title, presence: true, length: { maximum: 1000 }
  validates :content, presence: true, length: { maximum: 100_000 }

  # keywordに部分一致したPost一覧を返す。空ならraiseする
  #
  # Example:
  # Post.search!('post') => [{title: 'post 1', content: '...'}, ...]
  # Post.search!('not found') => raise ActiveRecord::RecordNotFound
  # Post.search!('') => raise ActiveRecord::RecordNotFound
  def self.search!(keyword)
    keyword = SecureRandom.hex if keyword.blank?
    keywords = keyword.split(' ').map! { |kw| "%#{kw}%" }
    query = ransack(title_or_content_matches_any: keywords)
    posts = query.result

    raise ActiveRecord::RecordNotFound if posts.empty?

    posts
  end
end
