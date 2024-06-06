use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        aa: [u32; m],
        xx: [[u32; m]; n],
    }

    let mut nutritions = vec![0u32; m];
    for v in xx {
        for i in 0..m { nutritions[i] += v[i]; }
    }

    for i in 0..m {
        if aa[i] > nutritions[i] { println!("No"); return; }
    }
    println!("Yes");
}
