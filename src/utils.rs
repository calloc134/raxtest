use anyhow::anyhow;
use regex::Regex;
use reqwest::{Client, Error, Response};
use serde_json::{to_string_pretty, to_writer_pretty, Value};
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use std::{collections::HashMap, str::FromStr};
use tokio::task::JoinHandle;

// とりあえず, mainから呼べるようにpubをつけている. いい方法かは不明
pub mod types;
use types::{Category, InitStep, JsonMap, RaxResult, ResultData, TestConfig, TestResult};

// テスト構成ファイルの構造体を生成する関数
// 引数：index_path: String -> テスト構成ファイルのパス。所有権を移動する
// 戻り値：RaxResult<(TestConfig, JsonMap)> -> テスト構成ファイルの構造体とjsonデータの連想配列のタプル
pub fn gen_struct(index_path: String) -> RaxResult<(TestConfig, JsonMap)> {
    // テスト構成ファイルを読み込む
    println!("[* ] Loading test config file...");
    let config_file = File::open(&index_path)?;
    let reader = BufReader::new(config_file);
    let test_config: TestConfig = serde_yaml::from_reader(reader)?;

    // データファイルのパス指定が正しいかチェックする
    println!("[* ] Checking data file path...");

    if !test_config.data.starts_with("json://") {
        return Err(anyhow!("Invalid data file path"));
        // 代替案: bail!("Invalid data file path"); -> anyhowのマクロ anyhowのエラーを返す
        // 代替案2: ensure!(test_config.data.starts_with("json://"),"Invalid data file path"); -> ensureマクロ ifをつかわずに書ける
    }

    // データの格納されているjsonファイルを読み込む
    println!("[* ] Loading json data file...");
    let data_file = File::open(test_config.data.trim_start_matches("json://"))?;
    let reader = BufReader::new(data_file);
    let json_data: JsonMap = serde_json::from_reader(reader)?;

    // 成功として、テスト構成ファイルの構造体とjsonデータを返す
    Ok((test_config, json_data))
}

// initステップを実行する関数
// 引数
// - base_url: &String -> テスト対象のベースURL。不変参照
// - init: Vec<InitStep> -> initステップの構造体の配列。所有権を移動する
// - json_data: &JsonMap -> jsonデータの連想配列。不変参照
// 戻り値：RaxResult<HashMap<String, String>> -> クッキーの連想配列をRaxResultでラップしたもの
pub async fn run_init(
    base_url: &String,
    init: Vec<InitStep>,
    json_data: &JsonMap,
) -> RaxResult<HashMap<String, String>> {
    // クッキーを格納するハッシュマップを作成
    let mut cookie_map: HashMap<String, String> =
        init.iter().fold(HashMap::new(), |mut acc, key| {
            acc.insert(key.name.to_string(), "".to_string());
            acc
        });

    // HTTPクライアントを初期化
    println!("[* ] Initializing HTTP client...");
    let client = Client::new();

    // タスクのベクタに、initステップの数だけクロージャを格納してテスト実行の前準備
    let tasks: Vec<JoinHandle<Result<Response, Error>>> = init
        .iter()
        .map(|init_step| {
            // クライアントをクローンする
            let client_clone = client.clone();

            // アクセスするURLを作成する
            let url = format!("{}{}", base_url, init_step.path);
            println!("[* -{name}] Accessing {}...", url, name = init_step.name);

            // リクエストクライアントの作成
            let mut request = client_clone.request(
                reqwest::Method::from_str(init_step.method.as_str()).unwrap(),
                &url,
            );

            // リクエストボディがある場合は、jsonデータよりリクエストボディを設定する
            if let Some(body) = &init_step.body {
                // リクエストボディをjsonデータから取得
                let init_data = json_data.get(body).unwrap().get("body").unwrap();
                println!(
                    "[* -{name}] Request body: {}",
                    to_string_pretty(&init_data).unwrap(),
                    name = init_step.name
                );
                // リクエストボディを設定
                request = request.json(&init_data);
            }

            // リクエストを送信
            tokio::spawn(async move {
                // リクエストを送信
                request.send().await
            })
        })
        .collect();

    // タスクのベクタに格納したクロージャを実行
    for (i, task) in tasks.into_iter().enumerate() {
        let response = task.await??;

        // クッキーをハッシュマップに格納する
        if let Some(cookie) = response.headers().get("set-cookie") {
            // クッキーのハッシュマップにクッキーの値を格納する
            cookie_map.insert(
                init[i].name.to_string(),
                cookie.to_str().unwrap().to_string(),
            );

            println!("[# -{name}] Cookie: {:?}", cookie, name = init[i].name);
        }
        // レスポンスボディを表示する
        let body = response.text().await?;
        println!("[# -{name}] Response body: {}", body, name = init[i].name);
        println!("[# -{name}] Init step {} completed", name = init[i].name);
        println!("")
    }

    Ok(cookie_map)
}

// テストステップを実行する関数
// 引数
// - base_url: &String -> テスト対象のベースURL。不変参照
// - steps: Vec<TestStep> -> テストステップの構造体の配列。所有権を移動する
// - json_data: &JsonMap -> jsonデータの連想配列。不変参照
// - cookie_map: &HashMap<String, String> -> クッキーの連想配列。不変参照
// 戻り値：RaxResult<Vec<TestResult>> -> テスト結果の構造体のベクタをRaxResultでラップしたもの
pub async fn run_test(
    base_url: &String,
    categories: HashMap<String, Category>,
    json_data: &JsonMap,
    cookie_map: &HashMap<String, String>,
) -> RaxResult<Vec<TestResult>> {
    // 結果を格納するベクタを初期化
    let mut results: Vec<TestResult> = Vec::new();
    // HTTPクライアントを初期化
    let client = Client::new();

    for (category_name, category) in categories.iter() {
        // タスクのベクタに、テストステップの数だけクロージャを格納してテスト実行の前準備
        let tasks: Vec<JoinHandle<Result<(usize, Response, Instant), Error>>> = category
            .steps
            .iter()
            .enumerate()
            .map(|(index, test_step)| {
                // クライアントをクローンする
                let client_clone = client.clone();

                // クエリの指定がある場合は、jsonデータよりクエリを設定し、URLを書き換える
                let rewrite_path = if let Some(query) = &test_step.query {
                    let test_query = &json_data.get(query).unwrap().get("query").unwrap();

                    // 正規表現をコンパイル
                    let re = Regex::new(r"\{(\w+)\}").unwrap();
                    // 正規表現にマッチした部分を、jsonデータから取得した値に置換する
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

                    println!(
                        "[* -{name}] Query: {}",
                        to_string_pretty(&test_query).unwrap(),
                        name = test_step.name
                    );
                    replaced_string.to_string()

                // クエリの指定がない場合は、パスをそのまま使用する
                } else {
                    test_step.path.clone()
                };

                // アクセスするURLを作成する
                let url = format!("{}{}", base_url, rewrite_path);
                println!("[* -{name}] Accessing {}...", url, name = test_step.name);

                // リクエストクライアントの作成
                let mut request = client_clone.request(
                    reqwest::Method::from_bytes(test_step.method.as_bytes())
                        .unwrap_or(reqwest::Method::GET),
                    &url,
                );

                // リクエストボディがある場合は、jsonデータよりリクエストボディを設定する
                if let Some(body) = &test_step.body {
                    // リクエストボディをjsonデータから取得
                    let test_body = &json_data.get(body).unwrap().get("body").unwrap();
                    request = request.json(test_body);
                    println!(
                        "[* -{name}] Request body: {}",
                        to_string_pretty(&test_body).unwrap(),
                        name = test_step.name
                    );
                }

                // ログインが必要な場合は、クッキーを設定する
                if let Some(login) = &category.login {
                    // クッキーをハッシュマップから取得
                    let cookie = cookie_map.get(login).unwrap();
                    println!("[* -{name}] Cookie: {}", cookie, name = test_step.name);

                    // リクエストヘッダにクッキーを設定
                    request = request.header("Cookie", cookie);
                }

                // リクエストを送信
                tokio::spawn(async move {
                    return match request.send().await {
                        // インデックスとレスポンスをタプルにして返す
                        Ok(response) => Ok((index, response, Instant::now())),
                        Err(e) => Err(e.into()),
                    };
                })
            })
            .collect();

        // タスクのベクタをイテレートして、レスポンスを受け取る
        for task in tasks {
            // タスクの結果を受け取る
            let (index, response, start_time) = task.await??;
            // タスクにかかった時間を計測
            let elapsed_time = start_time.elapsed();
            // ステータスコードを取得
            let status = response.status();

            println!(
                "[* -{name}] Status: {}",
                status,
                name = category.steps[index].name
            );
            println!(
                "[* -{name}] Headers: {:?}",
                response.headers(),
                name = category.steps[index].name
            );
            println!(
                "[* -{name}] Response body: {}",
                response.text().await?,
                name = category.steps[index].name
            );
            println!(
                "[* -{name}] Elapsed time: {}ms",
                elapsed_time.as_millis(),
                name = category.steps[index].name
            );

            // ステータスコードが期待値と一致するか確認し、結果を格納
            if status == category.steps[index].expect_status {
                println!(
                    "[# -{name}] Test passed!",
                    name = category.steps[index].name
                );
                results.push(TestResult {
                    name: category.steps[index].name.clone(),
                    category: category_name.clone(),
                    status: "success".to_string(),
                    message: format!(
                        "success (status: {}, expect status: {})",
                        status, category.steps[index].expect_status
                    ),
                    duration: elapsed_time.as_secs_f64(),
                });
            // 一致しない場合は、失敗として結果を格納
            } else {
                println!(
                    "[! -{name}] Test failed! (status: {}, expect status: {})",
                    status,
                    category.steps[index].expect_status,
                    name = category.steps[index].name
                );

                results.push(TestResult {
                    name: category.steps[index].name.clone(),
                    category: category_name.clone(),
                    status: "failure".to_string(),
                    message: format!(
                        "failed (status: {}, expect status: {})",
                        status, category.steps[index].expect_status
                    ),
                    duration: elapsed_time.as_secs_f64(),
                });
            }

            println!(
                "[# -{name}] Test step {} completed",
                category.steps[index].name,
                name = category.steps[index].name
            );
            println!("")
        }
    }

    Ok(results)
}

// テストの結果を出力する関数
// 引数
// - base_url: テスト対象のURL。不変参照
// - output_json_path: 出力するJSONファイルのパス。不変参照
// - results: テストの結果。所有権を移動
// 戻り値
// - RaxResult<()>: RaxResult型
pub fn render_results(
    base_url: &String,
    output_json_path: &String,
    results: Vec<TestResult>,
) -> RaxResult<()> {
    // 書き出すJSONデータを作成する
    let result_data = ResultData {
        base_url: base_url.clone(),
        results: results,
    };

    println!("[*] Outputting test results...");

    // テスト結果を出力する
    let output_file = File::create(output_json_path)?;
    to_writer_pretty(output_file, &result_data)?;

    println!("[*] Test completed!");

    Ok(())
}
