use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:Chars,
    }
    s.sort();
    let mut ans = 0;
    let mut root = 0;
    while root * root < 10usize.pow(n as u32) {
        let mut cs = vec![];
        let mut tmp = root * root;
        while 0 < tmp {
            cs.push(std::char::from_digit((tmp % 10) as u32, 10).unwrap());
            tmp /= 10;
        }
        while cs.len() < s.len() {
            cs.push('0');
        }
        cs.sort();
        if cs == s {
            ans += 1;
        }
        root += 1;
    }
    println!("{}", ans);
}
