use proconio::input;

fn main() {
    input! {
        mut n:[i64;3]
    }
    let mut count = 0;
    n.sort();
    loop {
        let a = n[0];
        let b = n[1];
        let c = n[2];
        let big = c - a;
        let small = b - a;
        let rem = big - small;
        let subb = rem / 2;
        let suba = rem - subb;
        n = vec![a - suba, a - subb, a];
        count += big;
        if n[0] == n[1] && n[0] == n[2] {
            break;
        }
    }
    if n[0] < 0 {
        println!("-1");
    } else {
        println!("{}", count + n[0]);
    }
}
