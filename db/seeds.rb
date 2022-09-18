puts 'db seed:'
Post.create!(title: 'First post', body: 'Hello World!!!');

10.times do |i|
  title = 'Post: ' + i.to_s;
  body = 'loop ' + i.to_s + ' times';
  p Post.create!(title: title, body: body)
end
