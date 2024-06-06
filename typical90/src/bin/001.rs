use std::io::{stdin, stdout, BufReader, Write};
use proconio::{input, source::line::LineSource};

fn main(){
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
    from &mut source,
    n: usize,
    l: usize,
    k: usize,
    mut a: [usize; n],
  }
    a.insert(0, 0);
    a.push(l);

    let diffs: Vec<usize> = a.windows(2).map(|x| x[1] - x[0]).collect();

    let mut low = diffs.iter().min().unwrap().clone();
    let mut high = l;
    let mut mid = l+2;

    while high > low+1 {
        mid = (high + low) / 2;
        let i = num_divide(mid, &diffs);
        if i < k { high = mid; }
        else { low = mid; }
    }

    if num_divide(high, &diffs) < k { high -= 1;}
    println!("{:?}", high);
}


fn num_divide(score: usize, v: &Vec<usize>) -> usize {
    let mut current_num = 0;
    let mut num_div = 0;
    for i in v {
        current_num += i;
        if current_num >= score {
            num_div += 1;
            current_num = 0;
        }
    }
    if current_num < score { num_div -= 1; }
    num_div
}
