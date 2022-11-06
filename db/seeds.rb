puts 'db seed:'
Post.create!(title: 'First post', content: 'Hello World!!!');

10.times do |i|
  title = 'Post: ' + i.to_s;
  content = 'loop ' + i.to_s + ' times';
  p Post.create!(title: title, content: content)
end
