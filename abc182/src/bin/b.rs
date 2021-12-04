use proconio::input;

fn main() {
    input! {
       n:i32, a:[i32;n]
    }
    let mut no_prime = vec![];
    let mut prime = vec![];
    for i in 2..1000 {
        if no_prime.contains(&i) {
            continue;
        } else {
            prime.push(i);
            let mut num = i;
            while num < 1000 {
                num += i;
                no_prime.push(num);
            }
        }
    }
    let mut max_count = 0;
    let mut ans = 0;
    for e in prime {
        let mut count = 0;
        for ai in &a {
            if ai % e == 0 {
                count += 1;
            }
        }
        if max_count < count {
            max_count = count;
            ans = e;
        }
    }
    println!("{}", ans);
}
