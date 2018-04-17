// Given a list of integers, use a vector and return

// the mean (average),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

fn median(list: &mut Vec<i32>) -> i32 {
    list.sort();
    list[list.len() / 2]
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &n in list {
        *(map.entry(n).or_insert(0)) += 1;
    }

    let mode = map
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1));

    match mode {
        Some(m) => m.0,
        None => panic!("list is empty")
    }
}

fn main() {
    let list = vec![-1, 7, -5, 7, -13, 0, 7, -6, 12];

    println!("mean: {:?}", mean(&list));
    println!("median: {:?}", median(&mut list.clone()));
    println!("mode: {:?}", mode(&list));
}
