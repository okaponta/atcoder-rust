use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        c:usize,
        ta:[(usize,usize);n],
    }
    let n = 30;
    let mut ans = c;
    let mut zero = 0;
    let mut one = (1 << 30) - 1;
    for (t, a) in ta {
        if t == 1 {
            zero &= a;
            one &= a;
        }
        if t == 2 {
            zero |= a;
            one |= a;
        }
        if t == 3 {
            zero ^= a;
            one ^= a;
        }
        let mut next = 0;
        for i in 0..n {
            if ans >> i & 1 == 0 {
                next += (zero >> i & 1) * (1 << i);
            } else {
                next += (one >> i & 1) * (1 << i);
            }
        }
        println!("{}", next);
        ans = next;
    }
}
