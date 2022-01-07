# Rustの書き方備忘録

## println
- 0埋め
  - println!("{:<03}", { n });

## 演算
- BitXOR
  - ^
- BitAND
  - &

# char
- nこめのアルファベット
  - (b'a' + ((n - 1) % 26) as u8) as char
- 数値に変換
  - c.to_digit(RADIX)
- `Vec<char>` -> String
  - `c.iter().collect::<String>();`
- chars -> String
  - `chars.collect::<String>();`

## String
- 部分文字列が存在する
  - t.contains(&s)
- 部分文字列
  - &s[0..k]
- charを追加
  - str.insert(char)
- String -> int
  - `s.parse::<i32>().unwrap();`

## Vec
- 前後と一緒にイテレーション
  - windows()
- 要素入れ替え
  - swap(a,b)
- 前半後半いれかえ
  - rotate_left(n)
  - rotate_right(n)
  - `s[n..].iter().collect::<String>(), s[..n].iter().collect::<String>())`

## iter
- 要素を一緒にまわす
  - zip
  - s.iter().zip(t.iter()).map(|(a,b)| a + b)
- 二重ループをスッと書く
  - iproduct!
  - for (i, j) in iproduct!(0..n, 0..n) { }

## Map
- 新規
  - let mut map = HashMap::new();
- 要素を取得、なかったら0
  - map.get(&key).map_or(0, |v| *v);
- 要素がなければ1を挿入、あれば+1
  - *map.entry(sum).or_insert(0) += 1;
