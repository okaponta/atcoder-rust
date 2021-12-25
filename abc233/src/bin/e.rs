use proconio::{input, marker::Chars};

fn main() {
    input! {
       x:Chars,
    }
    let mut sum = x.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
    let mut ans = vec![];
    let mut up = 0;
    for i in 0..x.len() {
        let mut _tmp = up + sum;
        ans.push(_tmp % 10);
        up = _tmp / 10;
        let digit = x[x.len() - 1 - i].to_digit(10).unwrap();
        sum = sum - digit;
    }
    if up != 0 {
        ans.push(up);
    }
    ans.reverse();
    let str = ans.iter().map(|i| i.to_string()).collect::<String>();
    println!("{}", str);
}
