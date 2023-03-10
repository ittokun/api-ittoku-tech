# api-ittoku-tech

このリポジトリは、ittoku-tech.comのAPIサーバーです。*.ittoku-tech.comからのレスポンスに対応します。

## アプリを動かす

以下のコマンドを実行することでアプリの構築ができます。

```bash
docker-compose build
docker-compose up
```

以下のコマンドを実行して、動作を確認してみましょう。

```bash
curl http://0.0.0.0:8080
```

**データベース**

PostgreSQLにアクセスするには以下のコマンドを実行します。

```bash
make db-access
```

**rust**

rustにアクセスするには以下のコマンドを実行します。

```bash
make api-bash
```
