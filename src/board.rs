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
    // construct an empty board upon instantiation
    pub fn new() -> Self {
        Board {
            squares: [Piece::Empty; BOARD_SIZE],
        }
    }

    // get piece (type) from an index 
    pub fn get_piece(&self, index: usize) -> Piece {
        self.squares[index]
    }

    // set piece (type) from a index
    pub fn set_piece(&mut self, index: usize, piece: Piece) {
        self.squares[index] = piece;
    }

    // print visual board in terminal
    pub fn print_board(&self) {
        for rank in (0..=7).rev() {
            for file in (0..=7).rev() {
                let index = rank * 16 + file;
                match self.squares[index] {
                    Piece::Empty => print!("- "),
                    piece => print!("{} ", get_piece_symbol(piece)),
                }
            }
            println!("");
        }
    }

    // initialize a board given a FEN string
    pub fn init_board_from_fen(&mut self, fen: &str) {
        let mut rank = 7;
        let mut file = 0;

        for c in fen.chars() {
            match c {
                '/' => {
                    rank -= 1;
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
   
    pub fn get_legal_moves(&self, position: usize) -> Vec<usize> {
        let mut legal_moves = Vec::new();

        let piece_type = self.get_piece(position);
        let file = position & 0xF; 
        let rank = (position & 0xF0) >> 4; // gets rank bits with bit manipulation

        // legal moves by piece type
        match piece_type {
            // pawns
            Piece::WPawn => {
                if rank < 7 {
                    legal_moves.push(position + 0x10);
                }
                if rank < 6 {
                    legal_moves.push(position + 0x20);
                }
            }
            Piece::BPawn => {
                if rank > 0 {
                    legal_moves.push(position - 0x10);
                }
                if rank > 1 {
                    legal_moves.push(position - 0x20);
                }
            }
            _ => {}
        }
        
        legal_moves
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

pub fn hex_to_chess_notation(position: usize) -> String {
    let rank = ((position & 0xF0) >> 4) + 1;

    let chess_rank = rank.to_string();
    file_to_chess_notation(position) + &chess_rank
}

fn file_to_chess_notation(position: usize) -> String {
    let file = position & 0xF; 
    
    match file {
        0 => "a".to_string(),
        1 => "b".to_string(),
        2 => "c".to_string(),
        3 => "d".to_string(),
        4 => "e".to_string(),
        5 => "f".to_string(),
        6 => "g".to_string(),
        7 => "h".to_string(),
        _ => panic!("Invalid file"),
    }
}
