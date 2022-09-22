use num::integer::Roots;
use proconio::input;

fn main() {
    input! {
       x:f64,
       y:f64,
       r:f64,
    }
    let (x, y, r) = (t(x), t(y), t(r));
    println!(
        "{}",
        ((x - r + 9999).div_euclid(10000)..=(x + r).div_euclid(10000))
            .into_iter()
            .fold(0, |ans, x2| {
                let y2 = (r * r - (x - x2 * 10000).pow(2)).sqrt();
                return ans + 1 + (y + y2).div_euclid(10000) - (y - y2 + 9999).div_euclid(10000);
            })
    );
}

fn t(a: f64) -> i64 {
    (a * 10000.0).round() as i64
}
