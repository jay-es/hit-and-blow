## プロジェクトセットアップ
1. GitHub でリポジトリ作成 → WSL 内にクローン
    - Fork に追加 `\\wsl$\Ubuntu-20.04\...`
2. VS Code で Reopen in Container
    - Rust → buster → （追加インストール）なし
    - イメージダウンロードを待つ
3. `cargo init` 実行
4. .vscode 追加
    - settings.json にフォーマット関連

## 実行
```shell
# 初回インストール
$ cargo install cargo-watch

# ソースに変更があるたびにビルド+実行（毎回コンソールをクリア）
cargo watch -x run --clear
```
