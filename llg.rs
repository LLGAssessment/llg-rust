use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::iter;

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

    {
    struct Env<'a> {
        graph: &'a Vec<Vec<usize>>,
        visited: &'a mut Vec<bool>,
        toppath: &'a mut Vec<usize>,
        stack: &'a mut Vec<usize>
    }
    let mut env = Env { graph: graph, visited: &mut visited, toppath: &mut toppath, stack: &mut stack };

    fn traverse_graph(env: &mut Env, pos: usize) {
        env.visited[pos] = true;
        env.stack.push(pos);
        if env.toppath.len() < env.stack.len() {
            env.toppath.clear();
            env.toppath.extend(env.stack.iter());
        }
        for link in &env.graph[pos] {
            if !env.visited[*link] {
                traverse_graph(env, pos);
            }
        }
        env.visited[pos] = false;
        env.stack.pop();
    };

    for i in 0..node_count {
        traverse_graph(&mut env, i);
    }
    }

    toppath
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

    let wordlist: Vec<String> = words
        .drain()
        .collect();

    let graph = build_graph(&wordlist);
    for word in longest_path(&graph).iter().map(|idx| &wordlist[*idx]) {
        println!("{}", word);
    }
}
