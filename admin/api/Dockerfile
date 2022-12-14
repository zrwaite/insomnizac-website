FROM ruby:3.1.2
RUN apt-get update -qq && apt-get install -y nodejs postgresql-client
WORKDIR /app
COPY Gemfile /app/Gemfile
COPY Gemfile.lock /app/Gemfile.lock
RUN bundle install

COPY . /app

CMD ["rails", "server", "-b", "0.0.0.0", "-p", "8012"]
