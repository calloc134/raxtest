use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// テスト構成ファイルの構造体を定義する
#[derive(Debug, Deserialize, Serialize)]
pub struct TestConfig {
    pub base_url: String,
    pub data: String,
    pub init: Vec<InitStep>,
    pub categories: HashMap<String, Category>,
}

// カテゴリーの構造体を定義する
#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub login: Option<String>,
    pub steps: Vec<TestStep>,
}

// initステップの構造体を定義する
#[derive(Debug, Deserialize, Serialize)]
pub struct InitStep {
    pub name: String,
    pub path: String,
    pub method: String,
    pub body: Option<String>,
}

// テストのステップの構造体を定義する
#[derive(Debug, Deserialize, Serialize)]
pub struct TestStep {
    pub name: String,
    pub path: String,
    pub method: String,
    pub expect_status: u16,
    pub query: Option<String>,
    pub body: Option<String>,
}

// テストの結果を格納する構造体を定義する
#[derive(Debug, Deserialize, Serialize)]
pub struct TestResult {
    pub name: String,
    pub category: String,
    pub status: String,
    pub duration: f64,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultData {
    pub base_url: String,
    pub results: Vec<TestResult>,
}

// Jsonの内容を格納する連想配列を定義する
// 引数：String -> jsonのキー。所有権を移動する
pub type JsonMap = HashMap<String, HashMap<String, HashMap<String, Value>>>;

// anyhowを使用したResult型のエイリアス
// 名前は適当につけているので、好きな名前に変更しても良い
// anyhow::Result<T> は Result<T, anyhow::Error> と同じ意味なので，そっちのほうが良いかも
pub type RaxResult<T> = Result<T, Error>;
