const BOARD_SIZE: usize = 128; // Size of 0x88 board

#[derive(Copy, Clone)]
pub enum Piece {
    Empty,
    WPawn,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
}

pub struct Board {
   squares: [Piece; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [Piece::Empty; BOARD_SIZE],
        }
    }

    pub fn get_piece(&self, index: usize) -> Piece {
        self.squares[index]
    }

    pub fn set_piece(&mut self, index: usize, piece: Piece) {
        self.squares[index] = piece;
    }

    pub fn print_board(&self) {
        for rank in 0..8 {
            for file in 0..8 {
                let index = rank * 16 + file;
                match self.squares[index] {
                    Piece::Empty => print!("- "),
                    piece => print!("{} ", get_piece_symbol(piece)),
                }
            }
            println!("");
        }
    }

    pub fn init_board_from_fen(&mut self, fen: &str) {
        let mut rank = 0;
        let mut file = 0;

        for c in fen.chars() {
            match c {
                '/' => {
                    rank += 1;
                    file = 0;
                }
                '1'..='8' => {
                    let count = c.to_digit(10).unwrap() as usize;
                    for _ in 0..count {
                        self.set_piece(rank * 16 + file, Piece::Empty);
                        file += 1;
                    }
                }
                'r' => {
                    self.set_piece(rank * 16 + file, Piece::BRook);
                    file += 1;
                }
                'n' => {
                    self.set_piece(rank * 16 + file, Piece::BKnight);
                    file += 1;
                }
                'b' => {
                    self.set_piece(rank * 16 + file, Piece::BBishop);
                    file += 1;
                }
                'q' => {
                    self.set_piece(rank * 16 + file, Piece::BQueen);
                    file += 1;
                }
                'k' => {
                    self.set_piece(rank * 16 + file, Piece::BKing);
                    file += 1;
                }
                'p' => {
                    self.set_piece(rank * 16 + file, Piece::BPawn);
                    file += 1;
                }
                'R' => {
                    self.set_piece(rank * 16 + file, Piece::WRook);
                    file += 1;
                }
                'N' => {
                    self.set_piece(rank * 16 + file, Piece::WKnight);
                    file += 1;
                }
                'B' => {
                    self.set_piece(rank * 16 + file, Piece::WBishop);
                    file += 1;
                }
                'Q' => {
                    self.set_piece(rank * 16 + file, Piece::WQueen);
                    file += 1;
                }
                'K' => {
                    self.set_piece(rank * 16 + file, Piece::WKing);
                    file += 1;
                }
                'P' => {
                    self.set_piece(rank * 16 + file, Piece::WPawn);
                    file += 1;
                }
                _ => {}
            }
        }
    }
}

fn get_piece_symbol(piece: Piece) -> String {
    match piece {
        Piece::Empty => "-".to_string(),
        Piece::WPawn => "♙".to_string(),
        Piece::WKnight => "♘".to_string(),
        Piece::WBishop => "♗".to_string(),
        Piece::WRook => "♖".to_string(),
        Piece::WQueen => "♕".to_string(),
        Piece::WKing => "♔".to_string(),
        Piece::BPawn => '\u{265F}'.to_string(),
        Piece::BKnight => "♞".to_string(),
        Piece::BBishop => "♝".to_string(),
        Piece::BRook => "♜".to_string(),
        Piece::BQueen => "♛".to_string(),
        Piece::BKing => "♚".to_string(),
    }
}
