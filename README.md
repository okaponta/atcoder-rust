atcoder の回答をただ貯めておくリポジトリです

## 自分用カンペ

コンテストごと

- procon-support でエイリアス読み込んだ状態で problem abcXXX で A-F 問題を開いておく
- cargo compete new abcXXX
- Cmd+Shift+N で新規 VSCode のウィンドウ開く
- Cmd+O で abcXXX を開く
- procon-support でエイリアス読み込み
- join でコミット
- cargo compete open

問題ごと

- test a
- submit a
- commit a
- run a

Rust のバージョンが 1.49.0 以降になったら以下でたぶんバージョン固定できる

- rustup toolchain add 1.49.0
- echo '1.49.0' > rust-toolchain
