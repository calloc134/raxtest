mod utils;
use clap::Parser;
use utils::types::AppResult;
use utils::{gen_struct, render_results, run_init, run_test};
// 引数を格納する構造体を定義
// raxtest
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// インプットするymlファイルのパス
    #[arg(short, long, required = true)]
    input_yml_path: String,

    /// 出力先のjsonファイルのパス
    #[arg(short, long, required = true)]
    output_json_path: String,

    /// リクエストの詳細を表示するかどうか
    /// true: 表示する
    /// false: 表示しない
    #[arg(short, long, default_value = "false")]
    print_flag: bool,

    /// クッキーが取得できなかった場合にテストを中断するかどうか
    #[arg(short, long, default_value = "false")]
    cookie_error_continue: bool,
}

#[tokio::main]
async fn main() -> AppResult<()> {
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

    let print_flag = args.print_flag;
    let cookie_error_continue = args.cookie_error_continue;

    // テスト構成ファイルの構造体を生成する
    let (test_config, json_data) = gen_struct(args.input_yml_path)?;

    // initステップを実行し、クッキーを取得する
    let cookie_map = run_init(
        &test_config.base_url,
        test_config.init,
        &json_data,
        &print_flag,
    )
    .await?;

    // テストステップを実行する
    let results = run_test(
        &test_config.base_url,
        test_config.categories,
        &json_data,
        &cookie_map,
        &print_flag,
        &cookie_error_continue,
    )
    .await?;

    // テスト結果をレンダリング
    render_results(&test_config.base_url, &args.output_json_path, results)?;

    Ok(())
}
