use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut odd = vec![];
    let mut even = vec![];
    for a in a {
        if a % 2 == 0 {
            even.push(a);
        } else {
            odd.push(a);
        }
    }
    if odd.len() < 2 && even.len() < 2 {
        println!("-1");
        return;
    }
    odd.sort();
    odd.reverse();
    even.sort();
    even.reverse();
    let o = if odd.len() < 2 { 0 } else { odd[0] + odd[1] };
    let e = if even.len() < 2 { 0 } else { even[0] + even[1] };
    println!("{}", o.max(e));
}
