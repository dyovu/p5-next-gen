use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の数が間違っています");
        std::process::exit(1);
    }

    let name = format!("{}.tsx", args[1]); // &strではなくStringにする

    // 作成するファイルのリスト
    let filenames: Vec<&str> = vec![
        "page.tsx",
        "Draw.ts",
        "Setup.ts",
        &name, // 引数を使ったファイル名
    ];

    for filename in filenames {
        let content = if filename == &name {
            read_template(&name, "template") // template.txt を使う
        } else {
            read_template(&name, filename) // 各ファイルごとのテンプレートを使う
        };

        // contentはread_tmpateの戻り値で、Result型,そのエラーをここで処理している
        match content {
            Ok(content) => {
                if let Err(e) = fs::write(Path::new(filename), content) {
                    eprintln!("Failed to create {}: {}", filename, e);
                } else {
                    println!("Created {}", filename);
                }
            }
            Err(e) => eprintln!("Failed to read template: {}", e),
        }
    }
}

// 各テンプレートファイルを読み込む
fn read_template(name: &str, filename: &str) -> Result<String, std::io::Error> {
    let filepath = format!("src/{}.txt", filename.split('.').next().unwrap()); // フォーマットを適用
    let template = fs::read_to_string(filepath)?; // ?はエラー型の場合, この時点でエラーを返す
    Ok(template.replace("{placeholder}", name.split('.').next().unwrap())) // プレースホルダーを置換
}
