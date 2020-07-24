#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: usize,
    }
    let mut ans = vec![s; k];
    if s < 1_000_000_000 {
        let mut add = vec![s + 1; n - k];
        ans.append(&mut add);
    } else {
        let mut add = vec![1; n - k];
        ans.append(&mut add);
    }
    for i in 0..n {
        print!("{} ", ans[i]);
    }
}
