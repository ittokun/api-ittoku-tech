class Post < ApplicationRecord
  has_many :comments, dependent: :destroy

  validates :title, presence: true, length: { maximum: 1000 }
  validates :content, presence: true, length: { maximum: 100_000 }

  # keywordに部分一致したPost一覧を返す。空ならraiseする
  #
  # Example:
  # Post.search('post') => [{title: 'post 1', content: '...'}, ...]
  # Post.search('not found') => []
  # Post.search('') => []
  def self.search(keyword)
    # 絶対に検索に引っかからないようにランダムな文字列を代入
    keyword = SecureRandom.hex if keyword.blank?
    # 複数のキーワードで検索する
    keywords = keyword.split(' ').map! { |kw| "%#{kw}%" }
    # title, contentの値で一致した文字列が含まれていた場合
    query = ransack(title_or_content_matches_any: keywords)

    query.result
  end
end
