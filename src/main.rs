use std::time::Instant;

mod board;
mod search;

fn main() {
    let board = &mut board::Board::new();
    let mut position_counter = PositionCounter::new();
    let mut line = String::new();
    println!("\nPlease input a depth: ");
    std::io::stdin().read_line(&mut line).expect("failed to read from stdin");
    let depth = match line.trim().parse::<usize>() {
        Ok(i) => i,
        Err(_) => {println!("Invalid depth"); 0},
    };

    let start = Instant::now();
    println!("{}", search::search(board, depth, true, &mut position_counter));

    println!("Elapsed time: {:.2?}", start.elapsed());
}

pub struct PositionCounter {
    positions: usize,
}

impl PositionCounter {
    pub fn new() -> Self {
        PositionCounter { positions: 0 }
    }

    pub fn add(&mut self) {
        self.positions += 1;
    }
}
