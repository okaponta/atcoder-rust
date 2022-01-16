use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {
        nchar : Chars,
        m : usize,
        c : [usize; m]
    }

    // BitOrで使うよう
    let mut idx = vec![0; 10];
    for i in 0..m {
        idx[c[i]] = 1 << i;
    }

    // tupleのひとつめがsum、ふたつめがcount
    // prev[a][b][bit]
    // a すべて0ではないとき、a=1。0のときa=0
    // b nより小さいのが確定しているとき、b=1。確定していないときb=0
    // bit cが含まれているかを[1011]みたいな感じでフラグ管理して、2進法で整数で表現したもの
    // bitを0-9までの数を管理するようにすれば、aがなくせるはず。
    let mut prev = vec![vec![vec![(0, 0); 1 << m]; 2]; 2];

    prev[0][0][0] = (0, 1);

    for x in nchar.into_iter().map(|c| c.to_digit(10).unwrap() as usize) {
        // 桁でloop

        let mut next = vec![vec![vec![(0, 0); 1 << m]; 2]; 2];
        for bit in 0..1 << m {
            for a in 0..2 {
                for b in 0..2 {
                    if prev[a][b][bit].1 == 0 {
                        // 処理対象がないのでスキップ
                        continue;
                    }
                    // 次の数字
                    for i in 0..10 {
                        if b == 0 && i > x {
                            // nより大きい数字になるため、処理を飛ばす
                            break;
                        }
                        // 0は含むけど、0だけの場合を弾く
                        let nextbit = if i == 0 && a == 0 { bit } else { bit | idx[i] };
                        let nexta = if i > 0 || a == 1 { 1 } else { 0 };
                        let nextb = if i < x || b == 1 { 1 } else { 0 };

                        next[nexta][nextb][nextbit].0 +=
                            prev[a][b][bit].0 * 10 + prev[a][b][bit].1 * i as usize;
                        next[nexta][nextb][nextbit].1 += prev[a][b][bit].1;
                        next[nexta][nextb][nextbit].0 %= MOD;
                        next[nexta][nextb][nextbit].1 %= MOD;
                    }
                }
            }
        }
        prev = next;
    }
    let mut ans = 0;
    for a in 0..2 {
        for b in 0..2 {
            ans += prev[a][b][(1 << m) - 1].0;
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
