use crate::io::load_gameplay;

#[derive(Clone, Copy, Debug)]
pub enum Block {
    Empty,
    Full,
    RampShort(u8), // orientation (left to right) 0 = up, 1 = down
    RampTall(u8), // same thing
}

impl Block {
    pub fn to_char(&self) -> char {
        match self {
            Block::Empty => ' ',
            Block::Full => 'O',
            Block::RampShort(dir) => if *dir == 1 { 'u' } else { 'd' },
            Block::RampTall(dir) => if *dir == 1 { 'U' } else { 'D' },
        }
    }
    pub fn from(num: u8) -> Self {
        match num {
            0 => Block::Empty,
            1 => Block::Full,
            2 => Block::RampShort(0),
            3 => Block::RampTall(0),
            _ => unreachable!("there aren't enough blocks in this town for the 4 of us."),
        }
    }
    pub fn from_char(c: char) -> Self {
        match c {
            ' ' => Block::Empty,
            'o' => Block::Full,
            'd' => Block::RampShort(0),
            'u' => Block::RampShort(1),
            'D' => Block::RampTall(0),
            'U' => Block::RampTall(1),
            _ => unreachable!("there aren't enough blocks in this town for the 4 of us."),
        }
    }
}

pub struct Generator {
    gameplay: Vec<[Block; 10]>
}

impl Generator {
    pub fn new() -> Self {
        Self {
            gameplay: Vec::new(),
        }
    }
    pub fn load_gameplay(&mut self, fp: &str) {
        self.gameplay = load_gameplay(fp.to_string());
    }
    pub fn run(&mut self) {
        for _ in 0..10 {
            self.next_row();
        }
    }
    fn next_row(&mut self) {
        let mut column = [Block::Empty; 10];
        for i in 0..10 {
            column[i] = Block::from(0);
        }
        self.gameplay.push(column);
    }
    pub fn print_gameplay(&self) {
        for i in 0..10 {
            println!("{}", self.gameplay.iter().map(|x| x[i].to_char()).collect::<String>());
        }
    }
}
