use regex::Regex;
use reqwest::{Client, Error, Response};
use std::fs::File;
use std::io::BufReader;
use std::{collections::HashMap, str::FromStr};
use tokio::task::JoinHandle;

use serde_json::Value;

mod types;
use types::{InitStep, JsonMap, TestConfig, TestStep};

// テスト構成ファイルの構造体を生成する関数
pub fn gen_struct(index_path: String) -> Result<(TestConfig, JsonMap), Box<dyn std::error::Error>> {
    // テスト構成ファイルを読み込む
    let config_file = File::open(&index_path)?;
    let reader = BufReader::new(config_file);
    let test_config: TestConfig = serde_yaml::from_reader(reader)?;

    // データファイルのパス指定が正しいかチェックする
    if !test_config.data.starts_with("json://") {
        return Err("Invalid data file path".into());
    }

    // データの格納されているjsonファイルを読み込む
    let data_file = File::open(test_config.data.trim_start_matches("json://"))?;
    let reader = BufReader::new(data_file);
    let json_data: JsonMap = serde_json::from_reader(reader)?;

    // 成功として、テスト構成ファイルの構造体とjsonデータを返す
    Ok((test_config, json_data))
}

// initステップを実行する関数
// 引数：base_url: &String -> テスト対象のベースURL。不変参照
//       init: Vec<InitStep> -> initステップの構造体の配列。所有権を移動する
//       json_data: &JsonMap -> jsonデータの連想配列。不変参照
pub async fn run_init(
    base_url: &String,
    init: Vec<InitStep>,
    json_data: &JsonMap,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // クッキーを格納するハッシュマップを作成
    let mut cookie_map: HashMap<String, String> =
        init.iter().fold(HashMap::new(), |mut acc, key| {
            acc.insert(key.name.to_string(), "".to_string());
            acc
        });

    // HTTPクライアントを初期化
    let client = Client::new();

    let tasks: Vec<JoinHandle<Result<Response, Error>>> = init
        .iter()
        .map(|init_step| {
            // クライアントをクローンする
            let client_clone = client.clone();

            // アクセスするURLを作成する
            let url = format!("{}/{}", base_url, init_step.path);

            // リクエストクライアントの作成
            let mut request = client_clone.request(
                reqwest::Method::from_str(init_step.method.as_str()).unwrap(),
                &url,
            );

            // リクエストボディがある場合は、jsonデータよりリクエストボディを設定する
            if let Some(body) = &init_step.body {
                let init_data = json_data.get(body).unwrap().get("body").unwrap();
                request = request.json(&init_data);
            }

            tokio::spawn(async move {
                // リクエストを送信
                request.send().await
            })
        })
        .collect();

    for (i, task) in tasks.into_iter().enumerate() {
        let response = task.await??;

        // クッキーをハッシュマップに格納する
        if let Some(value) = cookie_map.get_mut(init[i].name.as_str()) {
            *value = response
                .headers()
                .get("set-cookie")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }
    }

    Ok(cookie_map)
}

// テストステップを実行する関数
pub async fn run_test(
    base_url: &String,
    steps: Vec<TestStep>,
    json_data: &JsonMap,
    cookie_map: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // HTTPクライアントを初期化
    let client = Client::new();

    let tasks: Vec<JoinHandle<Result<Response, Error>>> = steps
        .iter()
        .map(|test_step| {
            let client_clone = client.clone();

            // クエリの指定がある場合は、jsonデータよりクエリを設定し、URLを書き換える
            let rewrite_path = if let Some(query) = &test_step.query {
                let test_query = &json_data.get(query).unwrap().get("query").unwrap();
                println!("query -> {:?}", test_query);

                // 正規表現をコンパイル
                let re = Regex::new(r"\{(\w+)\}").unwrap();
                let replaced_string =
                    re.replace_all(test_step.path.as_str(), |captures: &regex::Captures| {
                        let key = &captures[1];
                        let query_original = test_query.get(key).unwrap();
                        match query_original {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            _ => "".to_string(),
                        }
                    });

                println!("replaced -> {:?}", replaced_string);
                replaced_string.to_string()
            // クエリの指定がない場合は、パスをそのまま使用する
            } else {
                test_step.path.clone()
            };

            // アクセスするURLを作成する
            let url = format!("{}/{}", base_url, rewrite_path);

            println!("url -> {:?}", url);

            // リクエストクライアントの作成
            let mut request = client_clone.request(
                reqwest::Method::from_bytes(test_step.method.as_bytes())
                    .unwrap_or(reqwest::Method::GET),
                &url,
            );

            // リクエストボディがある場合は、jsonデータよりリクエストボディを設定する
            if let Some(body) = &test_step.body {
                let test_body = &json_data.get(body).unwrap().get("body").unwrap();
                println!("body -> {:?}", test_body);
                request = request.json(test_body);
            }

            // ログインが必要な場合は、クッキーを設定する
            if let Some(login) = &test_step.login {
                println!("hogehoge Password required");

                let cookie = cookie_map.get(login).unwrap();
                println!("cookie -> {:?}", cookie);

                request = request.header("Cookie", cookie);
            }

            tokio::spawn(async move { return request.send().await })
        })
        .collect();

    for task in tasks {
        let response = task.await??;

        println!("status -> {:?}", response.status());
        println!("headers -> {:?}", response.headers());
        println!("text -> {:?}", response.text().await?);
    }

    // テストステップを実行する
    /*
    for test_step in steps {
        let client_clone = client.clone();

        // クエリの指定がある場合は、jsonデータよりクエリを設定し、URLを書き換える
        let rewrite_path = if let Some(query) = test_step.query {
            let test_query = &json_data.get(&query).unwrap().get("query").unwrap();
            println!("query -> {:?}", test_query);

            // 正規表現をコンパイル
            let re = Regex::new(r"\{(\w+)\}").unwrap();
            let replaced_string =
                re.replace_all(test_step.path.as_str(), |captures: &regex::Captures| {
                    let key = &captures[1];
                    let query_original = test_query.get(key).unwrap();
                    match query_original {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => "".to_string(),
                    }
                });

            println!("replaced -> {:?}", replaced_string);
            replaced_string.to_string()
        // クエリの指定がない場合は、パスをそのまま使用する
        } else {
            test_step.path.clone()
        };

        // アクセスするURLを作成する
        let url = format!("{}/{}", base_url, rewrite_path);

        println!("url -> {:?}", url);

        // リクエストクライアントの作成
        let mut request = client.request(
            reqwest::Method::from_bytes(test_step.method.as_bytes())?,
            &url,
        );

        // リクエストボディがある場合は、jsonデータよりリクエストボディを設定する
        if let Some(body) = test_step.body {
            let test_body = &json_data.get(&body).unwrap().get("body").unwrap();
            println!("body -> {:?}", test_body);
            request = request.json(test_body);
        }

        // ログインが必要な場合は、クッキーを設定する
        if let Some(login) = test_step.login {
            println!("hogehoge Password required");

            let cookie = cookie_map.get(&login).unwrap();
            println!("cookie -> {:?}", cookie);

            request = request.header("Cookie", cookie);
        }

        let response = request.send()?;

        println!("text -> {:?}", response.text()?);


        if response.status() != test_step.expect_status {
            println!("Error: status code is not expected");
        } else if response.status() == test_step.expect_status {
            println!("Success: status code is expected");
        } else {
            return Err("Error: 予期せぬエラー".into());
        }
    }
    */

    Ok(())
}
