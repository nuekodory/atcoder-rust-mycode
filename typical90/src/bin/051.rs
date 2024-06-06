use itertools::Itertools;
use proconio::{input, fastout};


#[fastout]
fn main() {
    input! { n: usize, k: usize, p: u64, aa: [u64; n] }
    if n == 1 { println!("{}", (aa[0] <= p) as i8 ); return; }

}
