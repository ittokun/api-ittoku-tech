Rails.application.routes.draw do
  root 'application#index'

  get '/search/posts', to: 'searches#posts'

  resources :posts, only: %i[index show create update destroy] do
    resources :comments, only: %i[create destroy]
  end

  devise_for :users
end
