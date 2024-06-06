use proconio::input;

fn main() {
    input! { n: usize, mut aa: [u64; n] }
    aa.sort_unstable();
    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            ans += aa[j] / aa[i];
        }
    }
    println!("{ans}");
}
