use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.reverse();
    let mut cand = vec![];
    cand.push(a[0].to_string() + &a[1].to_string() + &a[2].to_string());
    cand.push(a[0].to_string() + &a[2].to_string() + &a[1].to_string());
    cand.push(a[1].to_string() + &a[0].to_string() + &a[2].to_string());
    cand.push(a[1].to_string() + &a[2].to_string() + &a[0].to_string());
    cand.push(a[2].to_string() + &a[0].to_string() + &a[1].to_string());
    cand.push(a[2].to_string() + &a[1].to_string() + &a[0].to_string());
    let mut ans = vec![];
    for s in cand {
        ans.push(s.parse::<usize>().unwrap());
    }
    ans.sort();
    ans.reverse();
    println!("{}", ans[0]);
}
