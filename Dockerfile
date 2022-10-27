FROM ruby:3.1.2

RUN apt-get update -qq && apt-get install -y postgresql-client

WORKDIR /api
ADD Gemfile /api/Gemfile
ADD Gemfile.lock /api/Gemfile.lock
RUN bundle install
ADD . /api

COPY entrypoint.sh /usr/bin
RUN chmod +x /usr/bin/entrypoint.sh
ENTRYPOINT ["entrypoint.sh"]
EXPOSE 8000

CMD ["rails", "server", "-b", "0.0.0.0", "-p", "8000"]
