use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        mut a:[Usize1;n],
    }
    let mut b = vec![0; n];
    for i in 0..n {
        b[a[i]] = i;
    }
    let mut ans = vec![];
    for i in 0..n {
        if a[i] == i {
            continue;
        }
        let j = b[i];
        let k = a[j];
        b.swap(a[i], k);
        a.swap(i, j);
        ans.push((i + 1, j + 1));
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{} {}", ans[i].0, ans[i].1);
    }
}
