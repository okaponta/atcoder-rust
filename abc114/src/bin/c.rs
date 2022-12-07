use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 0;
    for mut i in 0..1 << 20 {
        let mut tmp = 0;
        let mut ten = 1;
        let mut flg = 0;
        while 0 < i {
            flg |= 1 << (i % 4);
            tmp += ((i % 4) * 2 + 1) * ten;
            i /= 4;
            ten *= 10;
        }
        if flg != 14 {
            continue;
        }
        if n < tmp {
            println!("{}", ans);
            return;
        }
        ans += 1;
    }
}
