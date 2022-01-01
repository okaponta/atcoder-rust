use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
       n:i32,
       a:[i64;n],
       q:i32,
       bc:[(i64,i64);q],
    }
    let mut ans = a.iter().sum::<i64>();
    let mut map: HashMap<i64, i64> = HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }
    for bci in bc {
        let num = map.remove(&bci.0).unwrap_or(0);
        ans += (bci.1 - bci.0) * num;
        println!("{}", ans);
        *map.entry(bci.1).or_insert(0) += num;
    }
}
