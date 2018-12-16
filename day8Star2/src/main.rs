use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
enum Action {
    Node(usize, Option<usize>),
    Meta(usize, usize),
}

#[derive(Debug)]
struct Node {
    childs: Vec<usize>,
    metadata: Vec<usize>,
}

impl Node {
    fn get_sum(&self, nodes: &Vec<Node>) -> usize {
        if self.childs.len() == 0 {
            return self.metadata.iter().sum()
        }
        let mut sum = 0;
        for metadata in self.metadata.iter() {
            if metadata > &self.childs.len() {
                continue;
            }
            let idx = self.childs[*metadata-1];
            sum += nodes[idx].get_sum(nodes)
        }
        return sum
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let numbers = buf_reader.split(' ' as u8).filter_map(|l| String::from_utf8(l.unwrap()).unwrap().trim().parse::<usize>().ok());
    println!("{}", metadata_sum(numbers));
    Ok(())
}

fn metadata_sum<I>(numbers: I) -> usize
    where I: IntoIterator<Item=usize>
{
    let mut it = numbers.into_iter();
    let mut stack = vec![Action::Node(1, None)];
    let mut nodes = vec![];
    while let Some(todo) = stack.pop() {

        if let Action::Node(n, parent_id) = todo {
            let node_id = nodes.len();
            let node = Node{childs: vec![], metadata: vec![]};
            nodes.push(node);
            if let Some(p_id) = parent_id {
                nodes[p_id].childs.push(node_id)
            }
            let node_count = it.next().unwrap();
            let meta_count = it.next().unwrap();
            if n > 1 {
                stack.push(Action::Node(n-1, parent_id));
            }
            if meta_count > 0 {
                stack.push(Action::Meta(meta_count, node_id));
            }
            if node_count > 0 {
                stack.push(Action::Node(node_count, Some(node_id)));
            }
        }

        if let Action::Meta(n, parent_id) = todo {
            for _i in 0..n {
                nodes[parent_id].metadata.push(it.next().unwrap());
            }
        }
    }
    nodes[0].get_sum(&nodes)
}
