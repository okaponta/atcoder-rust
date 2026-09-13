# atcoder-rust

AtCoderの回答を貯めておくリポジトリです  
[acrust](https://github.com/okaponta/acrust)を使用しております。

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
  - 現在のディレクトリの内容をgit commitする
  - vscodeのデバッグ用config(`.vscode/launch.json`)は`acrust new`が`.acrust/template/copy/`から配置してくれるので、joinはコミットするだけです
- test
  - `test a`などでテストケースを実行します
- run
  - `run a`などで実際に動かせます
- submit
  - `submit a`などで提出できます
- copy
  - `copy a`などで解答をクリップボードにコピーします
  - 終了したコンテストには提出できないので、練習提出はこれでコピーしてブラウザに貼ります
- commit
  - `commit a`などでコメントつきでgit commitします
- problem
  - 引数なしでそのコンテストの全問題を開きます。`problem c`などで1問だけ開くこともできます
  - 問題IDはCargo.tomlから読むので、ABC/ARC併催回(abc042のcはarc058_a)も正しく開きます
  - コンテストのディレクトリの中で実行してください

# 自分用コマンドメモ

- コンテストごと
  - acrust new abcXXX
  - Cmd+Shift+N で新規 VSCode のウィンドウ開く
  - Cmd+O で abcXXX を開く
  - procon-support でエイリアス読み込み
  - problem で全問題を開く
  - join でコミット
- 問題ごと
  - test a
  - submit a
  - commit a
  - run a
- Rust のバージョンは `rust-toolchain.toml` でジャッジと同じものに固定してあります
  - `acrust env update` でジャッジ環境(rustc・edition・クレート)に追従します
  - `acrust status` で今の状態を確認できます
- ログイン求められた時(Submission rejected)
  - acrust login
