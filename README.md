
### p5をnext.jsで動かすためのテンプレートファイルを作るためのコマンド
入力した文字列に応じてディレクトリを作り、その中にテンプレートファイルを作る

**注意**
- フォルダの出力場所のパスを指定しないといけません。指定しないと実行時のディレクトリに作成されます
- あるプロジェクトのルートにいる場合(`./src/app/(pages)/`)以下に続けて新しいフォルダ名を入力することを想定


**入力**
- 新しい*フォルダ名*となる名前をパスカルケースで入力する. (ex. EuclidPattern)
- 上位ディレクトリまで指定する場合`./src/app/(pages)/EuclidPattern`のようになる

**出力**
- 入力したパスカルケースのディレクトリ
- `{入力名}.tsx`, `page.tsx`, `Setup.ts`, `Draw.ts`ファイル

```sh
// 入力がEuclidPatternの時
|-(pages)
|   |
|   |-EuclidPattern // ディレクトリ
|   |   |- Draw.ts
|   |   |- EuclidPattern
|   |   |- page.tsx
|       |- Setup.ts
| 
```


