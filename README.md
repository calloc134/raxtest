## 概要 / general

raxtestは、apiのテストを行うための、Rustで書かれた非同期で動作する軽量なツールです。

## インストール / install

```bash
cargo install raxtest
```

## 使い方 / usage

```bash
$ raxtest -i (index.ymlのパス) -o (output.jsonのパス)
```

 - index.yml: テストの設定を格納するyaml形式のファイル
 - output.json: テストの結果を保存するjson形式のファイル

## index.ymlの書き方 / how to write index.yml

以下に例を示します。
```yaml 
base_url: http://localhost
data: json://data.json
init:
  - name: loginStep
    path: api/auth/login
    method: POST
    body: init

steps:
  - name: apiall
    path: api/profile/all
    method: GET
    expect_status: 200

  - name: apiProfileUsername
    path: api/profile/@{name}
    method: GET
    query: ProfileUsername
    expect_status: 200

  - name: isLogin
    path: api/profile/me
    method: GET
    login: loginStep
    expect_status: 200

  - name: PostNewArticle
    path: api/post/new
    method: POST
    body: Article
    expect_status: 200
    login: loginStep
```
それぞれの項目の意味を以下に示します。

 - base_url: テスト対象のサーバのベースURL
 - data: テストに使用するデータを格納したファイルのパス
 - init: テストの初期化を行うステップ
 - steps: テストを行うステップ

### base_url 
テスト対象のサーバのベースURLを指定します。
### data
テストに使用するデータを格納したファイルのパスを指定します。
json形式のファイルを指定することができます。
パスは相対パスで指定できますが、index.ymlと同じディレクトリに配置することを推奨します。
### init
テストの初期化を行うステップを指定します。
ここではシーケンスを用いて、複数のステップを指定することができます。
### steps
テストを行うステップを指定します。
ここではシーケンスを用いて、複数のステップを指定することができます。

ステップのオプション項目は以下の通りです。

  - name: ステップの名前
  - 
  - path: リクエストのパス
  - method: リクエストのメソッド
  - body: リクエストのボディ
  - query: リクエストのクエリ
  - expect_status: 期待するステータスコード
  - login: ログインを行うステップ

