use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().map(|l| l.unwrap());
    let boxes: Vec<_> = lines.collect();
    for (i, id1) in boxes.iter().enumerate() {
        let mut found = false;
        for id2 in boxes[i..].iter() {
            if quasi_equal(id1, id2) {
                found = true;
                println!("{}", common(id1, id2));
                break;
            }
        }
        if found {
            break;
        }

    }
    Ok(())
}

fn quasi_equal(s1 :&str, s2 :&str) -> bool {
    let bytes = s1.bytes().zip(s2.bytes());
    let diffs = bytes.map(|(b1, b2)| if b1 == b2 { 0 } else { 1 });
    return 1 == diffs.sum()
}

fn common(s1: &str, s2: &str) -> std::string::String {
    let bytes = s1.bytes().zip(s2.bytes());
    let common = bytes.filter(|(b1, b2)| b1 == b2).map(|(b1, _b2)| b1);
    let res: Vec<_> = common.collect();
    return String::from_utf8(res).unwrap();
}
