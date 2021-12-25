# Rustの書き方備忘録

## println
- 0埋め
  - println!("{:<03}", { n });

## String
- 部分文字列が存在する
  - t.contains(&s)

## Vec
- 要素入れ替え
  - swap(a,b)
- 前半後半いれかえ
  - rotate_left(n)
  - rotate_right(n)
  - s[n..].iter().collect::<String>(), s[..n].iter().collect::<String>())

## Map
- 要素を取得、なかったら0
  - map.get(&key).map_or(0, |p| *p);