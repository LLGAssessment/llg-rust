use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;
use std::iter;
use std::fs::File;

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
        graph.push(links);
    }
    graph
}

fn longest_path(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let node_count = graph.len();
    let mut toppath: Vec<usize> = Vec::with_capacity(node_count);
    let mut stack: Vec<usize> = Vec::with_capacity(node_count);
    let mut visited: Vec<bool> = iter::repeat(false)
        .take(node_count)
        .collect();

    fn traverse_graph(
        graph: &Vec<Vec<usize>>,
        pos: usize,
        visited: &mut Vec<bool>,
        toppath: &mut Vec<usize>,
        stack: &mut Vec<usize>
    ) {
        visited[pos] = true;
        stack.push(pos);
        if toppath.len() < stack.len() {
            toppath.clear();
            toppath.extend(stack.iter());
        }
        for link in &graph[pos] {
            if !visited[*link] {
                traverse_graph(graph, *link, visited, toppath, stack);
            }
        }
        visited[pos] = false;
        stack.pop();
    };

    for i in 0..node_count {
        traverse_graph(graph, i, &mut visited, &mut toppath, &mut stack);
    }

    toppath
}

fn main() {
    let stdin = File::open("llg-dataset/70pokemons.txt").unwrap();
    let mut words: HashSet<String> = BufReader::new(&stdin)
        .lines()
        .map(
            |word| String::from(word.unwrap().trim())
            )
        .collect();
    words.remove("");

    let wordlist: Vec<String> = words
        .drain()
        .collect();

    let graph = build_graph(&wordlist);
    for word in longest_path(&graph).iter().map(|idx| &wordlist[*idx]) {
        println!("{}", word);
    }
}
