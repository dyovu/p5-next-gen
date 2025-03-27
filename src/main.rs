use std::env;
use std::fs;
use std::path::Path;

// テンプレートを埋め込む
const TEMPLATE_PAGE: &str = include_str!("templates/page.txt");
const TEMPLATE_DRAW: &str = include_str!("templates/Draw.txt");
const TEMPLATE_SETUP: &str = include_str!("templates/Setup.txt");
const TEMPLATE_GENERIC: &str = include_str!("templates/template.txt");

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の数が間違っています");
        std::process::exit(1);
    }

    // 引数の最初の文字が大文字かどうかをチェック
    if !args[1].chars().next().unwrap().is_uppercase() {
        eprintln!("最初の文字は大文字である必要があります");
        std::process::exit(1);
    }

    // 引数で受け取った名前をディレクトリ名として使用
    let dir_name = &args[1];
    let dir_path = Path::new(dir_name);

    // ディレクトリを作成
    if let Err(e) = fs::create_dir_all(dir_path) {
        eprintln!("Failed to create directory {}: {}", dir_name, e);
        std::process::exit(1);
    }

    let name = format!("{}.tsx", args[1]); // 引数で受け取った名前に基づくファイル名

    // 作成するファイルのリスト
    let filenames: Vec<(&str, &str)> = vec![
        ("page.tsx", TEMPLATE_PAGE),
        ("Draw.ts", TEMPLATE_DRAW),
        ("Setup.ts", TEMPLATE_SETUP),
        (&name, TEMPLATE_GENERIC), // 引数を使ったファイル名に対応するテンプレート
    ];

    for (filename, template) in filenames {
        // プレースホルダーを置換
        let content = template.replace("{placeholder}", args[1].as_str());

        // ファイルパスを生成
        let file_path = dir_path.join(filename);

        // ファイルを作成
        if let Err(e) = fs::write(&file_path, content) {
            eprintln!("Failed to create {}: {}", file_path.display(), e);
        } else {
            println!("Created {}", file_path.display());
        }
    }
}
