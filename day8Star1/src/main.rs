use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
enum Action {
    Node(usize),
    Meta(usize),
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
    let mut meta = 0;
    let mut stack = vec![Action::Node(1)];
    while let Some(todo) = stack.pop() {

        if let Action::Node(n) = todo {
            let node_count = it.next().unwrap();
            let meta_count = it.next().unwrap();
            if n > 1 {
                stack.push(Action::Node(n-1));
            }
            if meta_count > 0 {
                stack.push(Action::Meta(meta_count));
            }
            if node_count > 0 {
                stack.push(Action::Node(node_count));
            }
        }

        if let Action::Meta(n) = todo {
            for _i in 0..n {
                meta += it.next().unwrap();
            }
        }
    }
    meta
}
