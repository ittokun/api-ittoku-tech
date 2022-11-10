# ユーザー認証機能について

> 参考：https://github.com/heartcombo/devise

[Devise Token Auth](https://github.com/lynndylanhurley/devise_token_auth)がよさそう。と思ったが動かすには[devise](https://github.com/heartcombo/devise)が必要らしい。ならDevise Token Authはいらない。がまだわからない。とりあえずなしの方向で進める。

### 初めに

gemfileに以下のコードを追加する。

```ruby
gem 'devise'
```

次に以下のコマンドでセットアップを行う。

```shell
rails generate devise:install
```

config/environments/development.rbで以下のコードを追加する。

```ruby
config.action_mailer.default_url_options = { host: 'localhost', port: 8000 }
```

次に、以下のコマンドでdeviseでUserモデルを作成する。

```shell
rails generate devise User
```

マイグレートする。

```shell
rails db:migrate
```

コントローラも一応作成する。

```shell
rails generate devise:controllers api/v1/users
```

### 実装内容

まだわからん。とりあえず保留。

