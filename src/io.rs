use crate::generator::Block;
use std::{fs::File, io::Read};

pub fn load_gameplay(fp: String) -> Vec<[Block; 10]> {
    let mut file = File::open(fp).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // println!("{contents}");
    let mut gameplay = Vec::new();
    for line in contents.lines() {
        let mut tokens = line.split_whitespace();
        let block = Block::from_char(tokens.next().unwrap().chars().next().unwrap());
        let x = usize::from_str_radix(tokens.next().unwrap_or("0"), 10).unwrap();
        let y = usize::from_str_radix(tokens.next().unwrap_or("0"), 10).unwrap();

        if gameplay.len() <= x { gameplay.resize(x + 1, [Block::Empty; 10]); }
        gameplay[x][9 - y] = block;
    }
    gameplay
}