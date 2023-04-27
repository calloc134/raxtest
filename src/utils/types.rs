use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// テスト構成ファイルの構造体を定義する
#[derive(Debug, Deserialize)]
pub struct InputConfigration {
    pub base_url: String,
    pub data: String,
    pub init: Vec<InputStep>,
    pub categories: HashMap<String, InputCaterogy>,
}

// カテゴリーの構造体を定義する
#[derive(Debug, Deserialize)]
pub struct InputCaterogy {
    pub login: Option<String>,
    pub steps: Vec<InputStep>,
}

// ステップの構造体を定義する
#[derive(Debug, Deserialize)]
pub struct InputStep {
    pub name: String,
    pub path: String,
    pub method: String,
    pub ref_data: String,
    pub option: InputOption,
}
#[derive(Debug)]
pub struct FlattenStep {
    pub name: String,
    pub path: String,
    pub method: String,
    pub input_data: InputData,
}

// オプションの構造体を定義する
#[derive(Debug, Deserialize)]
pub struct InputOption {
    pub body: bool,
    pub query: bool,
}

// Jsonで与えられたデータの内容を格納する連想配列を定義する
// 引数：String -> jsonのキー。所有権を移動する
pub type InputDataMap = HashMap<String, Vec<InputData>>;

// Jsonの内部データを格納する連想配列を定義する
#[derive(Debug, Deserialize, Clone)]
pub struct InputData {
    pub body: Option<HashMap<String, Value>>,
    pub query: Option<HashMap<String, Value>>,
    pub expect_status: u16,
}

// テストの結果を格納する構造体を定義する
#[derive(Debug, Serialize)]
pub struct OutputResult {
    pub name: String,
    pub category: String,
    pub status: String,
    pub duration: f64,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct OutputData {
    pub base_url: String,
    pub results: Vec<OutputResult>,
}

// anyhowを使用したResult型のエイリアス
pub type AppResult<T> = Result<T, Error>;
