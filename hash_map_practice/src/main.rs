use std::collections::HashMap;

fn averages(sequence: &[i32]) -> (i32, i32){
    let mut sequence_vec: Vec<i32> = sequence.to_vec();
    sequence_vec.sort();
    let mut median = 0;
    if sequence_vec.len() % 2 == 0 {
        median = sequence_vec[sequence_vec.len() / 2];
    } else {
        median = sequence_vec[sequence_vec.len() / 2 - 1];
    };
    let mut counts = HashMap::new();
    for num in &sequence_vec {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mut max_freq = 0;
    let mut mode = 0;
    for (&key, &value) in &counts {
        if value > max_freq {
            max_freq = value;
            mode = *key;
        }
    }
    (median, mode)
}



fn main() {
    let results = averages(&[2, 4, 5, 6,7, 3, 3, 3, 5, 6, 5]);
    println!("Median: {}\nMode: {}", results.0, results.1);
}
