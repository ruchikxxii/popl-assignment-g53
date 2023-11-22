use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;
fn main() {
    let input = fs::read_to_string("citiesOrders.txt")
        .expect("Could not read file");
    
    let words = input.lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    
    use std::time::Instant;
    let now = Instant::now();
    let result = map_reduce(words, 4);
    let elapsed = now.elapsed();
    for (word, count) in result {
        println!("{}: {}", word, count);
    }
    println!("Elapsed: {:.2?}", elapsed);
}

fn map_reduce(words: Vec<String>, num_threads: usize) -> HashMap<String, u32> {
    let word_counts: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));
let mut handles = vec![];

for chunk in words.chunks(words.len() / num_threads) {
    let word_counts = Arc::clone(&word_counts);
    let chunk = chunk.to_vec();
    let handle = thread::spawn(move || {
        // let mut local_counts = HashMap::new();
        let mut local_counts = HashMap::with_capacity(chunk.len());
        for word in &chunk {
            *local_counts.entry(word.to_string()).or_insert(0) += 1;
        }
        let mut word_counts = word_counts.lock().unwrap();
        for (word, count) in local_counts {
            *word_counts.entry(word).or_insert(0) += count;
        }
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

Arc::try_unwrap(word_counts).unwrap().into_inner().unwrap()
}
