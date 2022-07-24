use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        c:usize,
        ta:[(usize,usize);n],
    }
    let max = (1 << 30) - 1;
    let mut ans = c;
    let mut zero = 0;
    let mut one = max;
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
        let next = ((ans ^ max) & zero) + (ans & one);
        println!("{}", next);
        ans = next;
    }
}
