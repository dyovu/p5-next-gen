use std::io::stdin;
use std::fs::{create_dir, create_dir_all, write};
use std::path::{Path, PathBuf};

// テンプレートを埋め込む
const TEMPLATE_PAGE: &str = include_str!("templates/page.txt");
const TEMPLATE_DRAW: &str = include_str!("templates/Draw.txt");
const TEMPLATE_SETUP: &str = include_str!("templates/Setup.txt");
const TEMPLATE_GENERIC: &str = include_str!("templates/template.txt");

fn main() {
    println!("パスカルケースで新しいディレクトリ名を入力してください");
    println!("パスを指定することもできます (ex: src/app/(pages)/ )");

    // 標準入力でディレクトリ名もしくはパスを受け取る
    let mut new_dir_name: String;
    let mut parent_dir:PathBuf;
    loop{
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let arg: Vec<&str> = input.trim().split('/').collect();
        let arg_len: usize = arg.len();
        new_dir_name = arg.last().expect("有効なディレクトリ名を入力してください").to_string();
        parent_dir = arg
            .iter()
            .take(arg_len - 1) //最後の要素以外を取得する
            .fold(PathBuf::new(), |mut acc, &dir| { // foldは2つの引数(初期値とクロージャ)を取る
                acc.push(dir);
                acc
            });

        println!("ディレクトリ名: {}", new_dir_name);
        println!("ディレクトリパス: {}", parent_dir.display());

        // 引数の最初の文字が大文字かどうかをチェック
        if !new_dir_name.chars().next().unwrap().is_uppercase() {
            eprintln!("新しく作成するディレクトリ名はパスカルケースで始まる必要があります。");
        }else{
            break; // Okだったらループを抜ける
        }
    }
    

    // ディレクトリを作成
    if !Path::exists(&parent_dir) && !parent_dir.as_os_str().is_empty(){
        println!("新しいディレクトリまでのパス {} が存在しません。新しく作成しますか？ Y/N", parent_dir.display());
        loop{
            let mut answer = String::new();
            stdin().read_line(&mut answer).unwrap();
            let answer = answer.trim().to_uppercase();
            if answer == "Y" {
                if let Err(e) = create_dir_all(&parent_dir) {
                    eprintln!("ディレクトリの作成に失敗しました: {}", e);
                    std::process::exit(1);
                }
                break;
            } else if answer == "N" {
                println!("ディレクトリの作成を中止します。");
                std::process::exit(0);
            } else {
                println!("YまたはNで答えてください。");
            }
        }
    }

    let dir_path = parent_dir.join(&new_dir_name);
    if let Err(e) = create_dir(&dir_path) {
        eprintln!("ディレクトリ {} の作成に失敗しました: {}", new_dir_name, e);
        std::process::exit(1);
    }

    let name = format!("{}.tsx", new_dir_name); // 引数で受け取った名前に基づくファイル名

    // 作成するファイルのリスト
    let filenames: Vec<(&str, &str)> = vec![
        ("page.tsx", TEMPLATE_PAGE),
        ("Draw.ts", TEMPLATE_DRAW),
        ("Setup.ts", TEMPLATE_SETUP),
        (&name, TEMPLATE_GENERIC), // 引数を使ったファイル名に対応するテンプレート
    ];

    for (filename, template) in filenames {
        // プレースホルダーを置換
        let content = template.replace("{placeholder}", &new_dir_name);

        // ファイルパスを生成
        let file_path = dir_path.join(filename);

        // ファイルを作成
        if let Err(e) = write(&file_path, content) {
            eprintln!("ファイル {} の作成に失敗しました: {}", file_path.display(), e);
        } else {
            println!("作成しました: {}", file_path.display());
        }
    }
}