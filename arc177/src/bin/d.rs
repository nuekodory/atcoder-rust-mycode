use std::collections::BinaryHeap;
use im_rc::HashSet;
use proconio::input;
use proconio::marker::Chars;
const N: usize = 500+2;

fn main() {
    input! { n: usize, cc: [Chars; n] }
    let mut mp = [[None; N]; N];
    for x in (0..n) {
        for y in (0..n) {
            mp[x+1][y+1] = Some(cc[y][x] == 'R');
        }
    }

    let mut red = [[u32::MAX; N]; N];
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    for x in (0..=n+1) {
        visited.insert((x, 0));
        visited.insert((x, n+1));
    }
    for y in (0..=n+1) {
        visited.insert((0, y));
        visited.insert((n+1, y));
    }
    queue.push((0u32, (1usize, 1usize)));

    while !queue.is_empty() {
        let (cost, (x, y)) = queue.pop().unwrap();
        if visited.contains(&(x, y)) { continue }
        visited.insert((x, y));
        red[x][y] = cost;
        // move

    }
}
