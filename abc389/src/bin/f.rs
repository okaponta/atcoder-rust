use ac_library::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        lr:[(usize,usize);n],
        q:usize,
        x:[usize;q],
    }
    let init = (0..500010).into_iter().collect::<Vec<_>>();
    let mut lazy = LazySegtree::<F>::from(init);
    for (l, r) in lr {
        let l = lazy.max_right(0, |x| x < l);
        let r = lazy.max_right(0, |x| x <= r);
        lazy.apply_range(l..r, 1);
    }
    for x in x {
        println!("{}", lazy.get(x));
    }
}

struct M;
impl Monoid for M {
    type S = usize;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}

struct F;
impl MapMonoid for F {
    type M = M;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        x + f
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}
