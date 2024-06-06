use proconio::input;


fn main() {
    input! { n: usize, k: usize, ab: [(i64, i64); n] }
    let mut points: Vec<_> = ab.into_iter().flat_map(|(a, b)| [a-b, b]).collect();
    points.sort_unstable();
    // println!("{:?}", points);
    let ans: i64 = points.into_iter().skip(2*n-k).sum();
    println!("{}", ans);
}
