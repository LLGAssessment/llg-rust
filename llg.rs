use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn build_graph(wordlist: &Vec<String>) -> Vec<Vec<usize>> {
    let words_count = wordlist.len();
    let mut graph: Vec<Vec<usize>> = Vec::with_capacity(words_count);

    let first_letters: Vec<char> = wordlist
        .iter()
        .map(|word| word.chars().next().unwrap())
        .collect();

    for i in 0..words_count {
        let mut links: Vec<usize> = Vec::with_capacity(words_count);
        let last_letter = wordlist[i]
            .chars()
            .last()
            .unwrap();
        for j in 0..words_count {
            if i != j && last_letter == first_letters[j] {
                links.push(j);
            }
        }
        graph[i] = links;
    }
    graph
}

fn main() {
    let stdin = io::stdin();
    let mut words: HashSet<String> = stdin.lock()
        .lines()
        .map(
            |word| String::from(word.unwrap().trim())
            )
        .collect();
    words.remove("");

    let mut wordlist: Vec<String> = words
        .drain()
        .collect();
    wordlist.sort();

    for word in &wordlist {
        println!("{}", word);
    }
    build_graph(wordlist);
}
