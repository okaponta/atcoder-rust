use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
    }
    let mut rem = a.iter().sum::<i64>();
    let mut rem_num = n as i64;
    a.sort();
    let mut ans = 0;
    for ai in a {
        rem_num -= 1;
        rem -= ai;
        ans += rem - ai * rem_num;
    }
    println!("{}", ans);
}
