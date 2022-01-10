use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u32 {
    let lines = data.split("\n");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        let node: Vec<&str> = line.split('-').collect();
        if map.contains_key(node[0]) {
            map.get_mut(node[0]).unwrap().push(node[1]);
        }
        else {
            map.insert(node[0], vec![node[1]]);
        }
        if map.contains_key(node[1]) {
            map.get_mut(node[1]).unwrap().push(node[0]);
        }
        else {
            map.insert(node[1], vec![node[0]]);
        }
    }
    let mut count = 0;
    let mut stack = vec![String::from("start")];
    while stack.len() != 0 {
        let last_item = stack.pop().unwrap();
        let visited_nodes: Vec<&str> = last_item.split('-').collect();
        let last_node = visited_nodes[visited_nodes.len() - 1];
        if last_node == "end" {
            count += 1;
        }
        else {
            for item in map.get(last_node).unwrap() {
                if item.chars().nth(0).unwrap().is_uppercase() {
                    stack.push(visited_nodes.join("-") + "-" + item.to_owned());
                }
                else if item != &"start" {
                    if !visited_nodes.contains(item) {
                        stack.push(visited_nodes.join("-") + "-" + item.to_owned());
                    }
                    else {
                        let mut repeated = false;
                        let mut visited = HashSet::new();
                        for nodes in &visited_nodes {
                            if nodes.chars().nth(0).unwrap().is_lowercase() {
                                if visited.contains(nodes) {
                                    repeated = true;
                                    break;
                                }
                                else {
                                    visited.insert(nodes);
                                }
                            }
                        }
                        if !repeated {
                            stack.push(visited_nodes.join("-") + "-" + item.to_owned());
                        }
                    }
                }
            }

        }
    }
    count   
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
