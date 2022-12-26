Rails.application.routes.draw do
  root 'application#index'

  get '/search/posts', to: 'searches#posts'

  resources :posts, only: %i[index show create update destroy] do
    resources :comments, only: %i[index create destroy]
  end

  devise_for :users

  match '*unmatched', to: 'application#url_not_found', via: :all
end
