# Markdown file import

Markdown のインポートの解決のために作った

```txt
#import(./demo/demo.md)
```

のように記述すると、その部分を指定したファイルで置換してくれる。

## Usage

```bash
cargo install --git https://github.com/usagrada/md-file-import.git
```

## Example

この README.md を実際に置換することができます。

```bash
md_file_import README.md
```

## Misc

ファイルは dist/元のファイル名 に出力されます。(相対パスで読んだ場合にはdist の外側に出力されることもあります。)

複数入力にも対応しています。
