Rails.application.routes.draw do
  namespace :api do
    namespace :v1 do
      resources :posts do
        collection do
          get 'search'
        end
      end
      devise_for :users
    end
  end
end
