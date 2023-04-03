mod utils;
use utils::{gen_struct, run_init, run_test};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // テスト構成ファイルの構造体を生成する
    let (test_config, json_data) = gen_struct("index.yml".to_string())?;

    // initステップを実行し、クッキーを取得する
    let cookie_map = run_init(&test_config.base_url, test_config.init, &json_data)?;

    run_test(
        &test_config.base_url,
        test_config.steps,
        &json_data,
        &cookie_map,
    )?;

    Ok(())
}
