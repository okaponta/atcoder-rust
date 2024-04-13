use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut l:usize,
        r:usize,
    }
    let mut ans = vec![];
    if l == 0 {
        let mut tmp = 1;
        while tmp * 2 <= r {
            tmp *= 2;
        }
        ans.push((0, tmp));
        l = tmp;
    }
    while l < r {
        // max
        for i in 0..62 {
            if l >> i & 1 == 1 {
                // min
                let mut j = 0;
                while j < i {
                    if r < (l + (1 << (j + 1))) {
                        break;
                    }
                    j += 1;
                }
                ans.push((l, l + (1 << j)));
                l += 1 << j;
                break;
            }
        }
    }
    println!("{}", ans.len());
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
