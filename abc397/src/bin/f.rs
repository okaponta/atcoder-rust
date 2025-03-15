use ac_library::*;
#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

struct M;
impl Monoid for M {
    type S = i64;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
    fn identity() -> Self::S {
        0
    }
}
impl MapMonoid for M {
    type F = i64;
    type M = M;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }
}

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(a[i]).or_insert(BTreeSet::new()).insert(i);
    }
    let mut seg = LazySegtree::<M>::new(n);
    for (_k, v) in &map {
        if 2 <= v.len() {
            seg.apply_range(*v.iter().next().unwrap()..*v.iter().last().unwrap(), 1);
        }
    }
    let mut ans = 0;
    let mut set = HashSet::new();
    for i in 0..n - 1 {
        set.insert(a[i]);
        map.get_mut(&a[i]).unwrap().remove(&i);
        if map[&a[i]].len() == 0 {
            map.remove(&a[i]);
        } else {
            seg.apply_range(0..*map[&a[i]].iter().next().unwrap(), -1);
        }
        let tmp = set.len() + map.len() + seg.prod(i..n) as usize;
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
