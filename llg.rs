use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut words: HashSet<String> = stdin.lock()
        .lines()
        .map(
            |word| String::from(word.unwrap().trim())
            )
        .collect();
    words.remove("");
    for word in &words {
        println!("{}", word);
    }
}
