use proconio::input;

fn main() {
    input! {
        n:usize,
        lr:[(usize,usize);n],
    }
    let mut q = vec![];
    for (l, r) in lr {
        q.push((l, 0));
        q.push((r, 1));
    }
    q.sort();
    let mut ans = 0i64;
    let mut count = 0;
    for (_i, q) in q {
        if q == 0 {
            ans += count;
            count += 1;
        } else {
            count -= 1;
        }
    }
    println!("{}", ans);
}
