use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        c:usize,
    }
    let mut ans = 10000;
    for i in 0..10000 {
        let tmpa = a * i;
        if n < tmpa {
            break;
        }
        for j in 0..10000 - i {
            let tmpb = tmpa + b * j;
            if n < tmpb {
                break;
            }
            if (n - tmpb) % c == 0 {
                ans = ans.min(i + j + (n - tmpb) / c);
            }
        }
    }
    println!("{}", ans);
}
