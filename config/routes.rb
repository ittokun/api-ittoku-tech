Rails.application.routes.draw do
  root 'application#index'

  get '/search/posts', to: 'searches#posts'
  resources :posts
  devise_for :users
end
