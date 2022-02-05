use proconio::input;

fn main() {
    input! {
        t:usize,
        andsum:[(usize,usize);t]
    }
    for (a, s) in andsum {
        if s < 2 * a {
            println!("No");
            continue;
        }
        println!("{}", if (s - 2 * a) & a == 0 { "Yes" } else { "No" });
    }
}
