use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: my_cli <name>");
        std::process::exit(1);
    }
    
    let name = &args[1];

    // 作成するファイルのリスト
    let filenames = vec![
        "page.tsx",
        "Draw.ts",
        "Setup.ts",
        &format!("{name}.tsx"),  // 引数を使ったファイル名
    ];

    for filename in &filenames {
        let path = Path::new(filename);
        if let Err(e) = fs::write(path, format!("// {}\n", filename)) {
            eprintln!("Failed to create {}: {}", filename, e);
        } else {
            println!("Created {}", filename);
        }
    }
}
