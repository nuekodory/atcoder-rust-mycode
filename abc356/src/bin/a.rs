use itertools::Itertools;
use proconio::input;

fn main() {
    input! { n: usize, l: usize, r:usize }
    let mut v = vec![];
    if l != r {
        for i in 1..l { v.push(i); }
        for i in l..=r { v.push(r-i+l); }
        for i in r + 1..=n { v.push(i); }
    } else {
        for i in 1..=n { v.push(i); }
    }
    let v = v.iter().map(|i| i.to_string()).collect_vec();
    let v = v.join(" ");
    println!("{v}");
}
