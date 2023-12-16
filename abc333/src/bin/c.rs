use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut rep = vec![1usize];
    for i in 0..15 {
        rep.push(rep[i] * 10 + 1);
    }
    let mut ans = vec![];
    for i in 0..15 {
        for j in 0..15 {
            for k in 0..15 {
                ans.push(rep[i] + rep[j] + rep[k]);
            }
        }
    }
    ans.sort();
    ans.dedup();
    println!("{}", ans[n - 1]);
}
