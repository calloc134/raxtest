mod utils;
use clap::Parser;
use utils::types::RaxResult;
use utils::{gen_struct, render_results, run_init, run_test};

/// 引数を格納する構造体を定義
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// インプットするymlファイルのパス
    #[arg(short, long, default_value_t = String::from("index.yml"))]
    input_yml_path: String,

    // 出力先jsonファイルのパス
    #[arg(short, long, default_value_t = String::from("result.json"))]
    output_json_path: String,
}

#[tokio::main]
async fn main() -> RaxResult<()> {
    // コマンドライン引数をパースする
    let args = Args::parse();

    // ASCIIアートを表示する
    let ascii_art = r#"
    _____  _____  __  __  ____  _____  _____  ____ 
    /  _  \/  _  \/  \/  \/    \/   __\/  ___>/    \
    |  _  <|  _  |>-    -<\-  -/|   __||___  |\-  -/
    \__|\_/\__|__/\__/\__/ |__| \_____/<_____/ |__|    
"#;
    println!("{}", ascii_art);

    // テスト構成ファイルの構造体を生成する
    let (test_config, json_data) = gen_struct(args.input_yml_path)?;

    // initステップを実行し、クッキーを取得する
    let cookie_map = run_init(&test_config.base_url, test_config.init, &json_data).await?;

    // テストステップを実行する
    let results = run_test(
        &test_config.base_url,
        test_config.categories,
        &json_data,
        &cookie_map,
    )
    .await?;

    // テスト結果をレンダリング
    render_results(&test_config.base_url, &args.output_json_path, results)?;

    Ok(())
}
