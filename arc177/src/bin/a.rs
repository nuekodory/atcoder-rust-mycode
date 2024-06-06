use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut a: u64, mut b: u64, mut c: u64,
        mut d: u64, mut e: u64, mut f: u64,
        n: usize,
        xx: [u64; n],
    }

    let vv = xx.into_iter().map(|x| total_to_required(x)).collect_vec();
    let mut req = vec![0u64; 6];


    for v in vv {
        req[0] += v[5];
        req[1] += v[4];
        req[2] += v[3];
        req[3] += v[2];
        req[4] += v[1];
        req[5] += v[0];
    }
    // println!("{:?}", req);

    let flag = {
        if req[0] > a { false } else {
            b += (a - req[0]) / 5;
            if req[1] > b { false } else {
                c += (b - req[1] ) / 2;
                if req[2] > c { false } else {
                    d += (c - req[2]) / 5;
                    if req[3] > d { false } else {
                        e += (d - req[3]) / 2;
                        if req[4] > e { false } else {
                            f += (e - req[4]) / 5;
                            if req[5] > f { false } else { true }
                        }
                    }
                }
            }
        }
    };

    if flag { println!("Yes"); } else { println!("No"); }
    return;
}

fn total_to_required(mut s: u64) -> Vec<u64> {
    let mut v = vec![];
    for m in vec![500, 100, 50, 10, 5, 1] {
        v.push(s/m);
        s -= (s/m)*m;
    }
    v
}
