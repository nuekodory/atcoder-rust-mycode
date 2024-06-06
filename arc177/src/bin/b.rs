use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! { n: usize, mut s: Chars };
    let mut is_cont = false;
    let mut start = usize::MAX;
    let mut conts = vec![];
    s.push('0');

    for (i, c) in s.into_iter().enumerate() {
        match c {
            '0' =>
            if is_cont { is_cont = false; conts.push((start, i-1)) }
            '1' =>
            if !is_cont { is_cont = true; start = i; }
            _ => panic!("unexpected char")
        }
    }
    let mut ans = vec![];
    // println!("{:?}", conts);

    for (start, end) in conts.into_iter().rev() {
        for _ in 0..=end { ans.push('A'); }
        for _ in 0..start { ans.push('B'); }
    }

    let ans: String = ans.into_iter().collect();
    println!("{}", ans.len());
    println!("{ans}");
}
