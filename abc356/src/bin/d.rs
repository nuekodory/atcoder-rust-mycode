use proconio::input;

fn main() {
    input! { n: u64, m: u64 }
    let mut small = 0;
    if n > m { small = m; } else { small = n; }

    let ones = small.count_ones();
    let mut ans = 2u64.pow(ones) % 998244353;
    if ones == 0 { ans = 0; }
    println!("{ans}");
}
