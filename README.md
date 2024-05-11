# atcoder-rust

AtCoderの回答を貯めておくリポジトリです  
[cargo-compete](https://github.com/qryxip/cargo-compete)を使用しております。

# About Me

[AtCoder](https://atcoder.jp/users/okaponta)で競プロをやっています。  
青レート帯でうろうろしてます。

# ライブラリ

ライブラリは[別のリポジトリ](https://github.com/okaponta/rust-kyopro-library)にまとめてます。

# エイリアス

[自作エイリアス](https://github.com/okaponta/atcoder-rust/blob/master/support.sh)を使用して多少の効率化を行っています。
- cv [型from] [型to]
  - [こちらのレポジトリ](https://github.com/okaponta/rust-kyopro-type-converter)にあるRustの型変換コマンドを読み込んでいます
  - ローカルにクローンしないと動かないです
- join
  - vscodeのデバッグ用configと現在のディレクトリの内容をgit commitする
- test
  - `test a`などでテストケースを実行します
- run
  - `run a`などで実際に動かせます
- submit
  - `submit a`などで提出できます
- commit
  - `commit a`などでコメントつきでgit commitします
- problem
  - `problem abcXXX`などでA-F問題を開きます
- qiita
  - qiita解説記事投稿用のテンプレを出力します。

# 自分用コマンドメモ

- コンテストごと
  - procon-support でエイリアス読み込んだ状態で problem abcXXX で A-F 問題を開いておく
  - cargo compete new abcXXX
  - Cmd+Shift+N で新規 VSCode のウィンドウ開く
  - Cmd+O で abcXXX を開く
  - procon-support でエイリアス読み込み
  - join でコミット
  - cargo compete open
- 問題ごと
  - test a
  - submit a
  - commit a
  - run a
- Rust のバージョンが 1.49.0 以降になったら以下でたぶんバージョン固定できる
  - rustup toolchain add 1.49.0
  - echo '1.49.0' > rust-toolchain
