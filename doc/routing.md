# ルーティングについて

以下のようにルーティングを設定しています。

## HTTP Status

| HTTP Status | Message            | Description                                                  |
| ----------- | ------------------ | ------------------------------------------------------------ |
| 404         | URL Not Found      | 存在しないURLにアクセスした場合                              |
| 422         | Parametter Invalid | `POST, PATCH, DELETE`などのメソッドで、パラメータが不正だった場合 |

上記の表は、HTTP Statusを返すときの条件と、メッセージを示しています。

また不正なパラメータであっても、200を返します。

## ルート（Application）

| Method | URL  | Controller        |
| ------ | ---- | ----------------- |
| GET    | /    | application#index |

**/**

ルートパスには、下記のルーティングのGETでアクセスすることのできるURLリストを配置しています。ただのindex（目次）です。

## 記事（Post）

| Method | URL        | Controller    |
| ------ | ---------- | ------------- |
| GET    | /posts     | posts#index   |
| POST   | /posts     | posts#create  |
| GET    | /posts/:id | posts#show    |
| PATCH  | /posts/:id | posts#update  |
| DELETE | /posts/:id | posts#destroy |

**GET /posts**

記事の一覧を返します。また、`page`パラメータを指定することでページを捲ることができます。

**POST /posts**

記事の作成を行います。成功したら作成した記事を、失敗したらエラーメッセージを返します。

**GET /posts/:id**

IDと一致した記事を返します。

**PATCH /posts/:id**

記事の編集を行います。成功したら編集した記事を、失敗したらエラーメッセージを返します。

**DELETE /posts/:id**

記事の削除を行います。成功したら削除した記事を、失敗したらエラーメッセージを返します。

## コメント（Comment）

| Method | URL                          | Controller       |
| ------ | ---------------------------- | ---------------- |
| GET    | /posts/:post_id/comments     | comments#index   |
| POST   | /posts/:post_id/comments     | comments#create  |
| DELETE | /posts/:post_id/comments/:id | comments#destroy |

**GET /posts/:post_id/comments**

post_idに一致した記事のコメントを返します。

**POST /posts/:post_id/comments**

post_idに一致した記事のコメントを作成します。

**DELETE /posts/:post_id/comments/:id**

post_idに一致した記事のコメントを削除します。

## 検索（Search）

| Method | URL           | Controller   |
| ------ | ------------- | ------------ |
| GET    | /search/posts | search#posts |

**GET /search/posts**

検索にヒットした記事を返します。以下のパラメータが使用できます。

- `q`、検索するキーワード。複数指定可能。
- `page`、ページを捲ることができます。
