use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut nums = vec![];
    for _ in 0..n {
        input! {
            l:usize,
            a:[usize;l],
        }
        nums.push(a);
    }
    for _ in 0..q {
        input! {
            s: Usize1,
            t: Usize1
        }
        println!("{}", nums[s][t]);
    }
}
