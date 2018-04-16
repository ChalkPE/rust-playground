// Given a list of integers, use a vector and return

// the mean (average),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut list: Vec<i32> = vec![-15, 30, 7, -5, -12, 7, 51, -17, 0, 43, 7, -6, -38];

    list.sort();
    for &i in &list {
        sum += i;
        *(map.entry(i).or_insert(0)) += 1;
    }

    println!("mean: {:?}", sum / list.len() as i32);
    println!("median: {:?}", list[list.len() / 2 as usize]);

    match map.iter().max_by(|x, y| x.1.cmp(y.1)) {
        None => println!("mode: unknown"),
        Some(mode) => println!("mode: {:?}", mode.0)
    }
}
