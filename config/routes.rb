Rails.application.routes.draw do
  namespace :api do
    namespace :v1 do
      get '/search/posts', to: 'searches#posts'
      resources :posts
      devise_for :users
    end
  end
end
