use proconio::input;

fn main() {
    input! {
       n:i32,
       s:[String;n]
    }
    let ac = s.iter().filter(|&s| s == "AC").count();
    let wa = s.iter().filter(|&s| s == "WA").count();
    let tle = s.iter().filter(|&s| s == "TLE").count();
    let re = s.iter().filter(|&s| s == "RE").count();
    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}
