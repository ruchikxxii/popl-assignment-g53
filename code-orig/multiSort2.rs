use std::thread;
 
fn merge_sort(slice: &[u32]) -> Vec<u32> {
    let len = slice.len();
    if len <= 1 {
        return slice.to_vec();
    }
 
    let mid = len / 2;
    let left = slice[..mid].to_vec();
    let right = slice[mid..].to_vec();
 
    let left_result;
    let right_result;

    if left.len()>100{
        let left_handle = thread::spawn(move || merge_sort(&left));
        let right_handle = thread::spawn(move || merge_sort(&right));
        left_result = left_handle.join().unwrap();
        right_result = right_handle.join().unwrap();
    }else {
        left_result = merge_sort(&left);
        right_result = merge_sort(&right);
    }
 
    merge(&left_result, &right_result)
}
 
fn merge(left: &[u32], right: &[u32]) -> Vec<u32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }
    merged
}
 
fn main() {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    let path = Path::new("data.txt");
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut unsorted = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let number: u32 = line.trim().parse().unwrap();
        unsorted.push(number);
    }
    use std::time::Instant;
    let now = Instant::now();
    let sorted = merge_sort(&unsorted);
    let elapsed = now.elapsed();
    // println!("{:?}", sorted);
    println!("Elapsed: {:.2?}", elapsed);
}