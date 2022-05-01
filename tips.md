# Rust の書き方備忘録

## println

- 0 埋め
  - println!("{:<03}", { n });

## 演算

- BitXOR
  - ^
- BitAND
  - &
- 割り算(切り上げ)
  - (a + b - 1) / b
- bitが1の数
  - count_ones()

## String

- 部分文字列が存在する
  - t.contains(&s)
- 部分文字列
  - &s[0..k]
- char を追加
  - str.insert(index,char)
- 末尾を先頭に
  - format!("{}{}", x % 10, x / 10)
- n 進数に変換
  - 2
    - format!("{:b}", i);
  - 8
    - format!("{:o}", i);
  - 16
    - format!("{:x}", i);
- 文字が'a'の場合にとりつづける(先頭&末尾)
  - let first_a_count = s.len() - s.trim_start_matches('a').len();
  - let last_a_count = s.len() - s.trim_end_matches('a').len();
  - Chars
    - let la = s.iter().take_while(|&&si| si == 'a').count();
    - let ra = s.iter().rev().take_while(|&&si| si == 'a').count();

## Vec

- 前後と一緒にイテレーション
  - windows()
- 要素入れ替え
  - swap(a,b)
- 要素の範囲削除(l から r まで)
  - drain((l-1)..r)
- 初期値いりで初期化
  - (0..n).collect_vec()
- 前半後半いれかえ
  - rotate_left(n)
  - rotate_right(n)
  - `s[n..].iter().collect::<String>(), s[..n].iter().collect::<String>())`
- 3 つで sort(全部逆順)

```
vec.sort_by(|a, b| match b.0.cmp(&a.0) {
  std::cmp::Ordering::Equal => match b.1.cmp(&a.1) {
      std::cmp::Ordering::Equal => b.2.cmp(&a.2),
      other => other,
  },
  other => other,
});
```

## iter

- 要素を一緒にまわす
  - zip
  - s.iter().zip(t.iter()).map(|(a,b)| a + b)
- 二重ループをスッと書く
  - iproduct!
  - for (i, j) in iproduct!(0..n, 0..n) { }
- itertools
  - (0..3).permutations(2)
    - [0,1],[0,2],[1,0],[1,2],[2,0],[2,1]
  - (0..3).combinations(2)
    - [0,1],[0,2],[1,2]
  - (0..3).combinations_with_replacement(2)
    - [0,0],[0,1],[0,2],[1,1],[1,2],[2,2]

## Map

- 新規
  - let mut map = HashMap::new();
- 要素を取得、なかったら 0
  - map.get(&key).map_or(0, |v| *v);
- 要素がなければ 1 を挿入、あれば+1
  - *map.entry(sum).or_insert(0) += 1;
- 要素のあるなしによって出力を切り替え

```
match map.get(&ans) {
    Some(res) => println!("{}", res),
    None => println!("-1"),
}
```

## memswap
- swap(&mut x, &mut y);
