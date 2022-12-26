# モデルについて

以下のモデルについて説明します。

## Post Model

| Name       | Type     | Etc                               |
| ---------- | -------- | --------------------------------- |
| id         | Instger  |                                   |
| title      | String   | required<br />max length: 1000    |
| content    | Text     | required<br />max length: 100_000 |
| created_at | DateTime |                                   |
| updated_at | DateTime |                                   |

**title**

記事のタイトルを入れる。記事の最も目立つ部分。

**content**

記事の内容を入れる。

### Comment Model

| Name       | Type       | Etc                            |
| ---------- | ---------- | ------------------------------ |
| id         | Integer    |                                |
| content    | Text       | required<br />max length: 1000 |
| post_id    | references | required                       |
| created_at | DateTime   |                                |
| updated_at | DateTime   |                                |

**content**

コメントを入れる。

**post_id**

コメントを入れる記事のIDを指定する。