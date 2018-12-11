extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

#[derive(Clone,PartialEq,Debug)]
enum Worker {
    Free,
    Busy(char, isize),
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut sons: HashMap<char, Vec<char>> = HashMap::new();
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.")
        .unwrap();

    for line in lines {
        let l = &line.unwrap();
        let caps = re.captures(l).unwrap();
        let before = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let after = caps.get(2).unwrap().as_str().chars().next().unwrap();
        sons.entry(before).or_insert(vec![]);

        let mut befores = sons.entry(after).or_insert(vec![]);
        befores.push(before);
    }
    println!("{:?}", sons);
    for (_, l) in sons.iter_mut() {
        l.sort_unstable();
    }

    let mut workers = vec![Worker::Free; 5];
    let mut time = 0;

    while sons.len() > 0 {
        for worker in workers.iter_mut() {
            if let Worker::Busy(task, end_time) = worker {
                if *end_time >= time {
                    continue;
                }

                for (_, l) in sons.iter_mut() {
                    match l.binary_search(&task) {
                        Ok(idx) =>  {l.remove(idx);},
                        Err(_) => (),
                    };
                }
            }
            // We won't free busy worker because of the break
            *worker = Worker::Free;
        }

        let free = get_free_worker(&workers);
        let next_task = get_valid(&sons);


        if let (Some(idx), Some(task)) = (free, next_task) {
            sons.remove(&task);
            let work_time = get_time_for_task(task);
            workers[idx] = Worker::Busy(task, time + work_time);
        } else {
            time = get_next_free_time(&workers) + 1;
        }
    }
    time = get_end_time(&workers);
    println!("{}", time + 1);

    Ok(())
}

fn get_valid(sons: &HashMap<char, Vec<char>>) -> Option<char> {
    let mut valids: Vec<_> = sons.iter().filter(|(_k, s)| s.len() == 0).collect();
    valids.sort_unstable_by(|a, b| a.0.cmp(b.0));
    if valids.len() > 0 {
        Some(*valids[0].0)
    } else {
        None
    }
}

fn get_free_worker(workers: &Vec<Worker>) -> Option<usize> {
    workers.iter().position(|w| w == &Worker::Free)
}

fn get_next_free_time(workers: &Vec<Worker>) -> isize {
    let busy = workers.iter().filter_map(
        |w| match w {
           &Worker::Free => None,
           &Worker::Busy(_, time) => Some(time),
        }
    );
    busy.min().unwrap()
}

fn get_end_time(workers: &Vec<Worker>) -> isize {
    let busy = workers.iter().filter_map(
        |w| match w {
           &Worker::Free => None,
           &Worker::Busy(_, time) => Some(time),
        }
    );
    busy.max().unwrap()
}

fn get_time_for_task(task: char) -> isize {
    60 + task as isize - 'A' as isize
}
