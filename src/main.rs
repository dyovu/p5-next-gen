use std::io::stdin;
use std::fs;
use std::path::PathBuf;

// テンプレートを埋め込む
const TEMPLATE_PAGE: &str = include_str!("templates/page.txt");
const TEMPLATE_DRAW: &str = include_str!("templates/Draw.txt");
const TEMPLATE_SETUP: &str = include_str!("templates/Setup.txt");
const TEMPLATE_GENERIC: &str = include_str!("templates/template.txt");

fn main() {
    println!("パスカルケースで新しいディレクトリ名を入力してください");
    println!("パスを指定することもできます");

    // 標準入力でディレクトリ名もしくはパスを受け取る
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let arg: Vec<&str> = input.trim().split('/').collect();
    let arg_len: usize = arg.len();
    let directory_name: &str = arg.last().unwrap_or(&"");
    let directory_path:PathBuf = arg
        .iter()
        .take(arg_len - 1)
        .fold(PathBuf::new(), |mut acc, &dir| {
            acc.push(dir);
            acc
        });

    println!("ディレクトリ名: {}", directory_name);
    println!("ディレクトリパス: {}", directory_path.display());


    // 引数の最初の文字が大文字かどうかをチェック
    if !directory_name.chars().next().unwrap().is_uppercase() {
        eprintln!("最初の文字は大文字である必要があります");
        std::process::exit(1);
    }

    // 引数で受け取った名前をディレクトリ名として使用
    let dir_path = directory_path.join(directory_name);

    // ディレクトリを作成
    if let Err(e) = fs::create_dir_all(&dir_path) {
        eprintln!("Failed to create directory {}: {}", directory_name, e);
        std::process::exit(1);
    }

    let name = format!("{}.tsx", directory_name); // 引数で受け取った名前に基づくファイル名

    // 作成するファイルのリスト
    let filenames: Vec<(&str, &str)> = vec![
        ("page.tsx", TEMPLATE_PAGE),
        ("Draw.ts", TEMPLATE_DRAW),
        ("Setup.ts", TEMPLATE_SETUP),
        (&name, TEMPLATE_GENERIC), // 引数を使ったファイル名に対応するテンプレート
    ];

    for (filename, template) in filenames {
        // プレースホルダーを置換
        let content = template.replace("{placeholder}", directory_name);

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
