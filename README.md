# example-rust-api

このリポジトリには、Rust言語の`actix-web`と`seaorm`を組み合わせたAPIサーバーを構成するためのソースコードがあります。

以下のコマンドを実行することで、動作を確認することができます。

試してみましょう！

### 動作要件

ここにあるソースコードを実行するためには`Docker`が必要になります。

インストールしておきましょう。

### 動かす

APIの立ち上げ

```bash
make run
```

APIの終了

```bash
make down
```

データベースにアクセス

```bash
make db-access
```

APIにアクセス

```bash
make api-bash
```

APIのテストを実行

```bash
make api-test
```

### 操作する

ルートにアクセス

```bash
curl http://0.0.0.0:8080
```

記事を投稿する

```bash
curl http://0.0.0.0:8080/posts -X POST -H 'Content-Type: application/json' -d '{"title": "bar", "text": "bar"}'
```

記事を閲覧する

```bash
curl http://0.0.0.0:8080/posts/1
```

記事を編集する

```bash
curl http://0.0.0.0:8080/posts/1 -X PATCH -H 'Content-Type: application/json' -d '{"title": "bar", "text": "bar"}'
```

記事のリストを見る

```bash
curl http://0.0.0.0:8080/posts
```

記事を削除する

```bash
curl http://0.0.0.0:8080/posts/1 -X DELETE
```
