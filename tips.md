# Rustの書き方備忘録

## println
- 0埋め
  - println!("{:<03}", { n });

## 演算
- BitXOR
  - ^
- BitAND
  - &

## String
- 部分文字列が存在する
  - t.contains(&s)
- 部分文字列
  - &s[0..k]
- charを追加
  - str.insert(index,char)
- 末尾を先頭に
  - format!("{}{}", x % 10, x / 10)
- Chars
  - 文字が'a'の場合にとりつづける(先頭&末尾)
  - let la = s.iter().take_while(|&&si| si == 'a').count();
  - let ra = s.iter().rev().take_while(|&&si| si == 'a').count();

## Vec
- 前後と一緒にイテレーション
  - windows()
- 要素入れ替え
  - swap(a,b)
- 要素の範囲削除(lからrまで)
  - drain((l-1)..r)
- 初期値いりで初期化
  - (0..n).collect_vec()
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
