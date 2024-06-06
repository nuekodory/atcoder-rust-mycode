use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! { n: usize, m: usize, k: u32, }
    let mut success = vec![];
    let mut failure = vec![];
    for _ in 0..m {
        input! { c: usize, aa: [Usize1; c], r: char }
        if r == 'o' { success.push(aa) }
        else { failure.push(aa) }
    }
    // println!("{:?} | {:?}", success, failure);
    let all_failure = success.len() == 0;
    let mut ans = 0;
    'bit: for b in 0u32..(1<<n) {
        if !all_failure && b.count_ones() < k { continue 'bit }
        // println!("{b:b}");
        for suc in success.iter() {
            let mut count = 0;
            for &i in suc {
                if b & (1<<i) > 0 { count += 1; }
            }
            if count < k { continue 'bit }
        }
        for fai in failure.iter() {
            let mut count = 0;
            for &i in fai {
                if b & (1<<i) > 0 { count += 1; }
            }
            if count >= k { continue 'bit }
        }
        ans += 1;
    }
    println!("{ans}");
}
