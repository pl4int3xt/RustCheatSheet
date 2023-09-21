use std::collections::{HashMap, HashSet};

fn main(){
  println!("{:?}", recover_secret(
    vec![ 
  ['t','u','p'],
  ['w','h','i'],
  ['t','s','u'],
  ['a','t','s'],
  ['h','a','p'],
  ['t','i','s'],
  ['w','h','s']]));
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
    let mut indegree: HashMap<char, usize> = HashMap::new();
    let mut result: Vec<char> = Vec::new();

    // Initialize the graph and indegree map
    for triplet in &triplets {
        for &c in triplet.iter() {
            graph.entry(c).or_insert(HashSet::new());
            indegree.entry(c).or_insert(0);
        }
    }

    // Build the graph and calculate indegree
    for triplet in &triplets {
        graph.get_mut(&triplet[0]).unwrap().insert(triplet[1]);
        graph.get_mut(&triplet[1]).unwrap().insert(triplet[2]);
        indegree.entry(triplet[1]).and_modify(|e| *e += 1);
        indegree.entry(triplet[2]).and_modify(|e| *e += 1);
    }

    // Perform topological sorting
    let mut stack: Vec<char> = indegree
        .iter()
        .filter(|&(_, &indeg)| indeg == 0)
        .map(|(c, _)| *c)
        .collect();

    while let Some(node) = stack.pop() {
        result.push(node);
        for &neighbor in graph.get(&node).unwrap() {
            indegree.entry(neighbor).and_modify(|e| *e -= 1);
            if *indegree.get(&neighbor).unwrap() == 0 {
                stack.push(neighbor);
            }
        }
    }

    result.iter().rev().collect()
    
  }