# raxtest

<p align="center">
  <img src="RAX_logo.png" alt="logo" width="200"/>
  <h1 align="center">raxtest</h1>
</p>

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
json形式で出力することができるため、CI/CDツールに組み込みやすくなっています。

## index.ymlの書き方 / how to write index.yml

以下に例を示します。
```yaml 
base_url: http://localhost
data: json://data.json
init:
- name: ApiAuthLogin(POST)
  path: /api/auth/login
  method: POST
  ref_data: ApiAuthLogin(POST)
  option:
    query: false
    body: true

categories:
  no_loginStep:
    - name: ApiUserMe(GET)
      path: /api/user/me
      method: GET
      ref_data: no_login/ApiUserMe(GET)
      option:
        query: false
        body: false

  loginStep:
    login: ApiAuthLogin(POST)
    - name: ApiUserMe(PUT)
      path: /api/user/me
      method: PUT
      ref_data: ApiAuthLogin(POST)/ApiUserMe(PUT)
      option:
        query: false
        body: true
    - name: ApiUserMe(DELETE)
      path: /api/user/me
      method: DELETE
      ref_data: ApiAuthLogin(POST)/ApiUserMe(DELETE)
      option:
        query: false
        body: false
    - name: ApiProfileScreenName_GET
      path: /api/profile/@{screenName}
      method: GET
      ref_data: ApiAuthLogin(POST)/ApiUserMe(DELETE)
      option:
        query: true
        body: false

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
  - steps: テストを行うステップのシーケンス

loginで、init内からログイン情報(クッキー)を参照するステップの名前を指定します。
stepsで、複数のステップを指定できます。

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

 - ref_data: リクエストのデータを参照する際のタグ  
リクエストのデータを参照する際に使用するタグを指定します。
ここで指定したタグは、bodyやqueryで使用することができます。

  - option: リクエストのオプション  
ここでは、queryとbodyをデータから参照するかどうかを指定できます。


## data.jsonの書き方 / how to write data.json

以下に例を示します。

```json
{
  "ApiAuthLogin(POST)": [
    {
      "body": {
        "handle": "johndoe2",
        "password": "Password123@"
      },
      "expect_status": 0
    }
  ],
  "ApiAuthLogin(POST)/ApiUserMe(GET)": [
    {
      "expect_status": 200
    }
  ],
  "ApiAuthLogin(POST)/ApiUserMe(PUT)": [
    {
      "body": {
        "bio": "じょんどえじょんどえ"
      },
      "expect_status": 200
    },
    {
      "body": {
        "screen_name": "じょんどえ2"
      },
      "expect_status": 200
    },
    {
      "body": {
        "hidden_comment": "じょんどえhidden"
      },
      "expect_status": 400
    },
    ...
  ],
}

```

data.jsonは、json形式のファイルです。  
このファイル内に、テストに使用するデータを格納します。  
プロパティはbody, query, expect_statusが対応しており、使用するデータは複数格納することが可能です。  
bodyオプションでは、`body`キーの値を、queryオプションでは、`query`キーの値を参照します。
また、expect_statusオプションでは、`expect_status`キーの値を参照します。

## output.jsonの構成 / structure of output.json

出力されるoutput.jsonは以下の通りです。

```json
{
  "base_url": "http://localhost",
  "results": [
    {
      "name": "no_login/ApiUserMe(DELETE)[0]",
      "category": "no_login",
      "status": "success",
      "duration": 0.0126284,
      "message": "success (status: 401 Unauthorized, expect status: 401)"
    },
    {
      "name": "no_login/ApiUserMe(GET)[0]",
      "category": "no_login",
      "status": "success",
      "duration": 0.0099064,
      "message": "success (status: 401 Unauthorized, expect status: 401)"
    },
    {
      "name": "no_login/ApiUserMe(PUT)[0]",
      "category": "no_login",
      "status": "success",
      "duration": 0.007595,
      "message": "success (status: 401 Unauthorized, expect status: 401)"
    },
    ...
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

 - category
ステップの所属するカテゴリです。
  
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
