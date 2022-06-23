use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        lr:[(usize,usize);n],
    }
    let max = 200_010;
    let mut kukan = vec![0; max];
    for (l, r) in lr {
        kukan[l] += 1;
        kukan[r] -= 1;
    }
    let mut s = 0;
    let mut is_ans = false;
    let mut l = 0;
    for i in 0..max {
        s += kukan[i];
        if 0 < s && !is_ans {
            l = i;
            is_ans = !is_ans;
        }
        if 0 == s && is_ans {
            println!("{} {}", l, i);
            is_ans = !is_ans;
        }
    }
}
