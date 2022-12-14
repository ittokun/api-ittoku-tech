puts 'db seed:'

post = Post.create!(title: 'First post', content: 'Hello World!!!')
Comment.create!(content: '1 Comment', post: post)

too_long_post = Post.create!(title: 'x' * 999, content: 'x' * 99_999)
Comment.create!(content: 'x' * 999, post: too_long_post)

posts_hash = []

50.times do |i|
  hash = {}
  hash['title'] = "Post: #{i}"
  hash['content'] = "loop #{i} times"
  posts_hash << hash
end

posts = Post.create!(posts_hash)
puts 'posts created:'
puts posts

comments_hash = [] 

50.times do |i|
  hash = {}
  hash['content'] = "#{i} Comment"
  hash['post'] = posts[rand(0..9)]
  comments_hash << hash
end

comments = Comment.create!(comments_hash)
puts 'comments created:'
puts comments
