use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n:usize,
    }
    if n <= 10 {
        println!("{}", n - 1);
        return;
    }
    n -= 10;
    if n <= 9 {
        println!("{}{}", n, n);
        return;
    }
    let mut digit = 2;
    let mut tmp = 9;
    let mut ten = 1;
    while tmp < n {
        n -= tmp;
        if digit % 2 == 0 {
            tmp *= 10;
            ten *= 10;
        }
        digit += 1;
    }
    ten -= 1;
    n += ten;
    let mut ans = vec!['0'; digit];
    let nn = n.to_string().chars().collect::<Vec<_>>();
    for i in 0..nn.len() {
        ans[i] = nn[i];
        ans[digit - 1 - i] = nn[i];
    }
    println!("{}", ans.iter().join(""));
}
