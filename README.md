# raxtest
```
    _____  _____  __  __  ____  _____  _____  ____ 
    /  _  \/  _  \/  \/  \/    \/   __\/  ___>/    \
    |  _  <|  _  |>-    -<\-  -/|   __||___  |\-  -/
    \__|\_/\__|__/\__/\__/ |__| \_____/<_____/ |__|   
```

![image](raxtest.output.svg)

## 概要 / general

raxtestは、apiのテストを行うための、Rustで書かれた非同期で動作する軽量なツールです。

## インストール / install

```bash
$ cargo install --git https://github.com/calloc134/raxtest.git
```

## 使い方 / usage

```bash
$ raxtest -i (index.ymlのパス) -o (output.jsonのパス)
```

 - index.yml: テストの設定を格納するyaml形式のファイル
 - output.json: テストの結果を保存するjson形式のファイル

## 特徴 / features

 - 非同期で動作  
テストステップは全て非同期で動作します。
 - テストの初期化処理を行うことができる  
ログイン処理など、テスト前に発声する初期化処理を自動化することができます。
 - テストの結果をjson形式で出力することができる  
テストの結果をjson形式で出力することができるため、CI/CDツールに組み込みやすくなっています。

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

categories:
  no_login:
    - name: apiall
      path: api/profile/all
      method: GET
      expect_status: 200
    - name: ApiProfileMe_GET
      path: /api/profile/me
      method: GET
      expect_status: 401

  loginStep:
    login: loginStep
    - name: ApiProfileMe_GET
      path: /api/profile/me
      method: GET
      expect_status: 200
    - name: ApiProfileMe_PUT
      path: /api/profile/me
      method: PUT
      body: ApiProfileMe_PUT
      expect_status: 200
    - name: ApiProfileScreenName_GET
      path: /api/profile/@{screenName}
      method: GET
      query: ApiProfileScreenName_GET
      expect_status: 200

```
それぞれの項目の意味を以下に示します。

 - base_url  
テスト対象のサーバのベースURLを指定します。
 -  data  
テストに使用するデータを格納したファイルのパスを指定します。  
json形式のファイルを指定することができます。  
パスは相対パスで指定できますが、index.ymlと同じディレクトリに配置することを推奨します。
 -  init  
テストの初期化を行うステップを指定します。  
ここではシーケンスを用いて、複数のステップを指定することができます。
 -  categories  
テストを行うステップのカテゴリを指定します。 

カテゴリのオプションは以下の通りです。

  - login: init内の参照するログイン情報のステップの名前
  - steps: テストを行うステップ

stepsは、シーケンスを用いながら複数のステップで指定します。

ステップのオプション項目は以下の通りです。

  - name: ステップの名前    
ステップの名前は、他のステップから参照する際に使用します。  
そのため、ステップの名前は一意である必要があります。

  - path: リクエストのパス  
リクエストのパスは、base_urlと結合されてリクエストのURLとなります。  
また、pathには、queryオプションで指定したファイル内のデータを参照することができます。  
その際は、`{name}`のように、`{}`で囲んで指定します。
この場合、dataで指定したファイル内に、`name`というキーが存在する必要があります。

  - method: リクエストのメソッド  
リクエストのメソッドは、GET, POST, PUT, DELETEなどを指定できます。

  - body: リクエストのボディ  
リクエストのボディは、dataで指定したファイル内のデータを参照することができます。  
ここで指定したデータを、リクエストのボディとしてjson形式で送信します。

  - query: リクエストのクエリ  
リクエストのクエリは、dataで指定したファイル内のデータを参照することができます。  
ここで指定したデータを、リクエストのクエリとしてjson形式で送信します。

  - expect_status: 期待するステータスコード  
期待するステータスコードは、リクエストのレスポンスのステータスコードと比較します。  
この値とレスポンスのステータスコードが一致しない場合、テストは失敗します。

  - login: ログイン情報  
init内のステップを参照し、ログイン情報を取得します。

## data.jsonの書き方 / how to write data.json

以下に例を示します。

```json
{
    "init": {
        "body": {
            "screenName": "johndoe",
            "password": "Password"
        }
    },
    "ProfileUsername": {
        "query": {
            "name": "johndoe"
        }
    },
    "Article": {
        "body": {
            "title": "テスト",
            "body": "テストで投稿した記事です。"
        }
    }
}
```

data.jsonは、json形式のファイルです。  
このファイル内に、テストに使用するデータを格納します。  
このデータは、index.yml内のbodyオプションとqueryオプションにて参照することができます。
bodyオプションでは、`body`キーの値を、queryオプションでは、`query`キーの値を参照します。

## output.jsonの構成 / structure of output.json

出力されるoutput.jsonは以下の通りです。

```json
{
  "base_url": "http://localhost",
  "results": [
    {
      "name": "apiall",
      "status": "success",
      "duration": 0.000048,
      "message": "passed"
    },
    {
      "name": "apiProfileUsername",
      "status": "success",
      "duration": 0.0113897,
      "message": "passed"
    },
    {
      "name": "isLogin",
      "status": "success",
      "duration": 0.0034016,
      "message": "passed"
    },
    {
      "name": "PostNewArticle",
      "status": "failure",
      "duration": 0.0000487,
      "message": "failed (status: 400 Bad Request, expect status: 200)"
    }
  ]
}
```

これらのオプションは以下の通りです。

  - base_url  
テスト対象のサーバのベースURLを指定します。
  
  - results  
テスト結果を格納する配列です。

また、results配列内の各要素は以下の通りです。

  - name  
ステップの名前です。
  
  - status  
ステップの結果を示します。
`success`または`failure`のいずれかの値を取ります。
  
  - duration  
ステップの実行時間を示します。

  - message  
ステップの詳細結果を示します。  
テストが成功した場合は、`passed`となります。  
テストが失敗した場合は、`failed (status: XXX Bad Request, expect status: XXX)`のように、ステータスコードと期待するステータスコードを示します。

## 注意事項 / caution
このプログラムは現在開発中のため、バグが含まれている可能性があります。  
また、バグを発見した場合は、PRを送っていただけると幸いです。

## 姉妹プロジェクト / sister projects
 - [openapi2raxtest](https://github.com/calloc134/openapi2raxtest) : OpenAPIからRaxTestのテストケースを生成するツール
