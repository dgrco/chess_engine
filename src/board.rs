use std::collections::HashMap;

const BOARD_SIZE: usize = 128; // Size of 0x88 board

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Copy, Clone, PartialEq)]
pub struct DynamicPiece {
    piece_type: PieceType,
    piece_color: PieceColor,
    position: usize,
}

impl DynamicPiece {
    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn get_color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn get_type_and_color(&self) -> (PieceType, PieceColor) {
        (self.piece_type, self.piece_color)
    }

    pub fn get_position(&self) -> usize {
        self.position
    }
}

pub struct Board {
    squares: [DynamicPiece; BOARD_SIZE],
    white_set: Vec<DynamicPiece>,
    black_set: Vec<DynamicPiece>,
    fen: String,
}

impl Board {
    // construct an empty board upon instantiation
    pub fn new() -> Self {
        let default_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut board = Board {
            squares: [DynamicPiece {piece_type: PieceType::Empty, 
                piece_color: PieceColor::None, position: 0}; BOARD_SIZE],
                white_set: vec![],
                black_set: vec![],
                fen: default_fen.clone(),
        };

        board.init_board_from_fen(default_fen);
        board
    }

    // performs a deep copy of the board to create a new board
    pub fn copy(&self) -> Board {
        let mut board = Board::new();

        board.init_board_from_fen(self.fen.clone());

        board
    }

    pub fn get_fen(&self) -> String {
        self.fen.clone()
    }

    // get piece (type) from an index 
    pub fn get_piece(&self, index: usize) -> DynamicPiece {
        self.squares[index]
    }

    // set piece (type) from a index
    pub fn set_piece(&mut self, index: usize, piece: DynamicPiece) {
        self.squares[index] = piece;
        self.squares[index].position = index;

        self.update_fen();
    }

    // get all white pieces
    pub fn get_white_pieces(&self) -> Vec<DynamicPiece> {
        self.white_set.clone()
    }

    // get all black pieces
    pub fn get_black_pieces(&self) -> Vec<DynamicPiece> {
        self.black_set.clone()
    }

    // update the fen string upon a change
    pub fn update_fen(&mut self) {
        let mut new_fen = String::from("");
        let mut space_count = 0;

        for rank in (0..=7).rev() {
            for file in 0..=7 {
                match self.squares[file_rank_to_hex(file, rank)].get_type_and_color() {
                    (PieceType::Empty, PieceColor::None) => {
                        space_count += 1;
                    }
                    (PieceType::Pawn, PieceColor::White) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "P";
                    }
                    (PieceType::Rook, PieceColor::White) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "R";
                    }
                    (PieceType::Knight, PieceColor::White) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "N";
                    }
                    (PieceType::King, PieceColor::White) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "K";
                    }
                    (PieceType::Queen, PieceColor::White) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "Q";
                    }
                    (PieceType::Bishop, PieceColor::White) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "B";
                    }
                    (PieceType::Pawn, PieceColor::Black) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "p";
                    }
                    (PieceType::Rook, PieceColor::Black) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "r";
                    }
                    (PieceType::Knight, PieceColor::Black) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "n";
                    }
                    (PieceType::King, PieceColor::Black) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "k";
                    }
                    (PieceType::Queen, PieceColor::Black) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "q";
                    }
                    (PieceType::Bishop, PieceColor::Black) => {
                        if space_count > 0 {
                            new_fen += &space_count.to_string();
                            space_count = 0;
                        }
                        new_fen += "b";
                    }
                    _ => {},
                }
            }
            if space_count > 0 {
                new_fen += &space_count.to_string();
                space_count = 0;
            }
            if rank > 0 {
                new_fen += "/";
            }
        }
        // will be changed
        new_fen += " w KQkq - 0 1";
        self.fen = new_fen;
    }

    // print visual board in terminal
    pub fn print_board(&self) {
        for rank in (0..=7).rev() {
            print!("\t{} | ", rank + 1);
            for file in 0..=7 {
                let index = rank * 16 + file;
                match (self.squares[index].piece_type, self.squares[index].piece_color) {
                    (PieceType::Empty, PieceColor::None) => print!("- "),
                    (piece_type, piece_color) => print!("{} ", 
                                                        get_piece_symbol(piece_type, piece_color)),
                }
            }
            println!("");
        }
        print!("\t   -----------------\n\t    a b c d e f g h\n\n")
    }

    // initialize a board given a FEN string
    pub fn init_board_from_fen(&mut self, fen: String) {
        let mut rank = 7;
        let mut file = 0;

        // wipe white/black set
        self.white_set.clear();
        self.black_set.clear();

        for c in fen.chars() {
            match c {
                ' ' => {
                    break;
                }
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    let count = c.to_digit(10).unwrap() as usize;
                    for _ in 0..count {
                        self.set_piece(rank * 16 + file, 
                                       DynamicPiece { 
                                           piece_type: PieceType::Empty, 
                                           piece_color: PieceColor::None,
                                           position: file_rank_to_hex(file, rank),
                                       });
                        file += 1;
                    }
                }
                'r' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Rook, 
                                       piece_color: PieceColor::Black,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.black_set.push(DynamicPiece { 
                        piece_type: PieceType::Rook, 
                        piece_color: PieceColor::Black,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'n' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Knight, 
                                       piece_color: PieceColor::Black,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.black_set.push(DynamicPiece { 
                        piece_type: PieceType::Knight, 
                        piece_color: PieceColor::Black,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'b' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Bishop, 
                                       piece_color: PieceColor::Black,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.black_set.push(DynamicPiece { 
                        piece_type: PieceType::Bishop, 
                        piece_color: PieceColor::Black,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'q' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Queen, 
                                       piece_color: PieceColor::Black,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.black_set.push(DynamicPiece { 
                        piece_type: PieceType::Queen, 
                        piece_color: PieceColor::Black,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'k' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::King, 
                                       piece_color: PieceColor::Black,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.black_set.push(DynamicPiece { 
                        piece_type: PieceType::King, 
                        piece_color: PieceColor::Black,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'p' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Pawn, 
                                       piece_color: PieceColor::Black,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.black_set.push(DynamicPiece { 
                        piece_type: PieceType::Pawn, 
                        piece_color: PieceColor::Black,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'R' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Rook, 
                                       piece_color: PieceColor::White,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.white_set.push(DynamicPiece { 
                        piece_type: PieceType::Rook, 
                        piece_color: PieceColor::White,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'N' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Knight, 
                                       piece_color: PieceColor::White,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.white_set.push(DynamicPiece { 
                        piece_type: PieceType::Knight, 
                        piece_color: PieceColor::White,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'B' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Bishop, 
                                       piece_color: PieceColor::White,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.white_set.push(DynamicPiece { 
                        piece_type: PieceType::Bishop, 
                        piece_color: PieceColor::White,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'Q' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Queen, 
                                       piece_color: PieceColor::White,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.white_set.push(DynamicPiece { 
                        piece_type: PieceType::Queen, 
                        piece_color: PieceColor::White,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'K' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::King, 
                                       piece_color: PieceColor::White,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.white_set.push(DynamicPiece { 
                        piece_type: PieceType::King, 
                        piece_color: PieceColor::White,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                'P' => {
                    self.set_piece(rank * 16 + file, 
                                   DynamicPiece { 
                                       piece_type: PieceType::Pawn, 
                                       piece_color: PieceColor::White,
                                       position: file_rank_to_hex(file, rank),
                                   });
                    self.white_set.push(DynamicPiece { 
                        piece_type: PieceType::Pawn, 
                        piece_color: PieceColor::White,
                        position: file_rank_to_hex(file, rank),
                    });
                    file += 1;
                }
                _ => {}
            }
        }
    }

    pub fn get_legal_moves(&self, position: usize) -> Vec<usize> {
        let mut legal_moves = Vec::new();

        let piece_type = self.get_piece(position).piece_type;
        let piece_color = self.get_piece(position).piece_color;
        let file = position & 0xF; 
        let rank = (position & 0xF0) >> 4; // gets rank bits with bit manipulation

        // legal moves by piece type
        match piece_type {
            // pawns
            PieceType::Pawn => {
                match piece_color {
                    PieceColor::White => {
                        if self.square_empty(file, rank + 1) {
                            if rank < 7 {
                                legal_moves.push(file_rank_to_hex(file, rank + 1));
                            }
                            if rank == 1 && self.square_empty(file, rank + 2) {
                                legal_moves.push(file_rank_to_hex(file, rank + 2));
                            }
                        }
                        if file > 0 && rank < 7 && 
                            self.is_capturable(file_rank_to_hex(file, rank), 
                                               file_rank_to_hex(file - 1, rank + 1)) {
                                legal_moves.push(file_rank_to_hex(file - 1, rank + 1))
                            }
                        if file < 7 && rank < 7 && 
                            self.is_capturable(file_rank_to_hex(file, rank), 
                                               file_rank_to_hex(file + 1, rank + 1)) {
                                legal_moves.push(file_rank_to_hex(file + 1, rank + 1))
                            }
                    }
                    PieceColor::Black => {
                        if self.square_empty(file, rank - 1) {
                            if rank > 0 {
                                legal_moves.push(file_rank_to_hex(file, rank - 1));
                            }
                            if rank == 6 && self.square_empty(file, rank - 2) {
                                legal_moves.push(file_rank_to_hex(file, rank - 2));
                            }
                        }
                        if file > 0 && rank > 0 && 
                            self.is_capturable(file_rank_to_hex(file, rank), 
                                              file_rank_to_hex(file - 1, rank - 1)) {
                            legal_moves.push(file_rank_to_hex(file - 1, rank - 1))
                        }
                        if file < 7 && rank > 0 && 
                            self.is_capturable(file_rank_to_hex(file, rank), 
                                              file_rank_to_hex(file + 1, rank - 1)) {
                            legal_moves.push(file_rank_to_hex(file + 1, rank - 1))
                        }
                    }
                    _ => {}
                }
            }
            // Rooks
            PieceType::Rook => {
                for straight_move in self.get_straights(rank, file) {
                    legal_moves.push(straight_move);
                }
            }
            // Bishops
            PieceType::Bishop => {
                for diag_move in self.get_diagonals(rank, file) {
                    legal_moves.push(diag_move);
                }
            }

            // Queen
            PieceType::Queen => {
                for straight_move in self.get_straights(rank, file) {
                    legal_moves.push(straight_move);
                }

                for diag_move in self.get_diagonals(rank, file) {
                    legal_moves.push(diag_move);
                }
            }

            // King
            PieceType::King => {
                // up
                if rank < 7 && self.square_empty(file, rank + 1) {
                    legal_moves.push(file_rank_to_hex(file, rank + 1));
                }
                // top-left
                if rank < 7 && file > 0 && self.square_empty(file - 1, rank + 1) {
                    legal_moves.push(file_rank_to_hex(file - 1, rank + 1));
                }
                // top-right
                if rank < 7 && file < 7 && self.square_empty(file + 1, rank + 1) {
                    legal_moves.push(file_rank_to_hex(file + 1, rank + 1));
                }
                // left
                if file > 0 && self.square_empty(file - 1, rank) {
                    legal_moves.push(file_rank_to_hex(file - 1, rank));
                }
                // right
                if file < 7 && self.square_empty(file + 1, rank) {
                    legal_moves.push(file_rank_to_hex(file + 1, rank));
                }
                // bottom-left
                if file > 0 && rank > 0 && self.square_empty(file - 1, rank - 1) {
                    legal_moves.push(file_rank_to_hex(file - 1, rank - 1));
                }
                // bottom
                if rank > 0 && self.square_empty(file, rank - 1) {
                    legal_moves.push(file_rank_to_hex(file, rank - 1));
                }
                // bottom-right
                if rank > 0 && file < 7 && self.square_empty(file + 1, rank - 1) {
                    legal_moves.push(file_rank_to_hex(file + 1, rank - 1));
                }
            }

            // Knights
            PieceType::Knight => {
                // up-up-right
                if rank < 6 && file < 7 && (self.square_empty(file + 1, rank + 2)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file + 1, rank + 2))){
                    legal_moves.push(file_rank_to_hex(file + 1, rank + 2));
                }

                // up-up-left
                if rank < 6 && file > 0 && (self.square_empty(file - 1, rank + 2)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file - 1, rank + 2))) {
                    legal_moves.push(file_rank_to_hex(file - 1, rank + 2));
                }

                // up-right-right
                if rank < 7 && file < 6 && (self.square_empty(file + 2, rank + 1)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file + 2, rank + 1))) {
                    legal_moves.push(file_rank_to_hex(file + 2, rank + 1));
                }

                // up-left-left
                if rank < 7 && file > 1 && (self.square_empty(file - 2, rank + 1)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file - 2, rank + 1))) {
                    legal_moves.push(file_rank_to_hex(file - 2, rank + 1));
                }

                // down-down-right
                if rank > 1 && file < 7 && (self.square_empty(file + 1, rank - 2)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file + 1, rank - 2))) {
                    legal_moves.push(file_rank_to_hex(file + 1, rank - 2));
                }

                // down-down-left
                if rank > 1 && file > 0 && (self.square_empty(file - 1, rank - 2)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file - 1, rank - 2))) {
                    legal_moves.push(file_rank_to_hex(file - 1, rank - 2));
                }

                // down-right-right
                if rank > 0 && file < 6 && (self.square_empty(file + 2, rank - 1)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file + 2, rank - 1))) {
                    legal_moves.push(file_rank_to_hex(file + 2, rank - 1));
                }

                // down-left-left
                if rank > 0 && file > 1 && (self.square_empty(file - 2, rank - 1)
                                            || self.is_capturable(file_rank_to_hex(file, rank), 
                                                                  file_rank_to_hex(file - 2, rank - 1))) {
                    legal_moves.push(file_rank_to_hex(file - 2, rank - 1));
                }
            }
            _ => {}
        }

        legal_moves
    }

    // gets every legal move for every piece on the board
    pub fn get_board_legal_moves(&self, white_to_move: bool) 
        -> HashMap<usize, Vec<usize>> {
            let mut all_moves: HashMap<usize, Vec<usize>> = HashMap::new();
            if white_to_move {
                for piece in self.white_set.clone() {
                    let available_moves = self.get_legal_moves(piece.get_position());
                    if available_moves.len() > 0 {
                        all_moves.insert(piece.get_position(), available_moves);
                    }
                }
            } else {
                for piece in self.black_set.clone() {
                    let available_moves = self.get_legal_moves(piece.get_position());
                    if available_moves.len() > 0 {
                        all_moves.insert(piece.get_position(), available_moves);
                    }
                }
            }

            all_moves
        }

    // helpers for get_straights and get_diagonals
    fn add_position_if_valid(&self, list: &mut Vec<usize>, original_file: usize,
                             original_rank: usize, current_file: usize, current_rank: usize) -> bool {
        if self.square_empty(current_file, current_rank) {
            list.push(file_rank_to_hex(current_file, current_rank));
            true
        } else {
            if self.is_capturable(file_rank_to_hex(original_file, original_rank),
            file_rank_to_hex(current_file, current_rank)) {
                list.push(file_rank_to_hex(current_file, current_rank));
            }
            false
        }
    }

    pub fn get_straights(&self, rank: usize, file: usize) -> Vec<usize> {
        let mut straights = Vec::new();
        let mut current_file = file;
        let mut current_rank = rank;

        // up
        while current_rank < 7 {
            current_rank += 1;
            if !self.add_position_if_valid(&mut straights, file, rank, current_file, current_rank) {
                break;
            }
        }
        // down
        current_file = file;
        current_rank = rank;
        while current_rank > 0 {
            current_rank -= 1;
            if !self.add_position_if_valid(&mut straights, file, rank, current_file, current_rank) {
                break;
            }
        }
        // left
        current_file = file;
        current_rank = rank;
        while current_file > 0 {
            current_file -= 1;
            if !self.add_position_if_valid(&mut straights, file, rank, current_file, current_rank) {
                break;
            }
        }
        // right
        current_file = file;
        current_rank = rank;
        while current_file < 7 {
            current_file += 1;
            if !self.add_position_if_valid(&mut straights, file, rank, current_file, current_rank) {
                break;
            }
        }

        straights
    }

    pub fn get_diagonals(&self, rank: usize, file: usize) -> Vec<usize> {
        let mut diagonals = Vec::new();
        let mut current_file = file;
        let mut current_rank = rank;

        // down-left
        while current_rank > 0 && current_file > 0 {
            current_file -= 1;
            current_rank -= 1;
            if !self.add_position_if_valid(&mut diagonals, file, rank, current_file, current_rank) {
                break;
            }
        }

        // down-right
        current_file = file;
        current_rank = rank;
        while current_rank > 0 && current_file < 7 {
            current_file += 1;
            current_rank -= 1;
            if !self.add_position_if_valid(&mut diagonals, file, rank, current_file, current_rank) {
                break;
            }
        }

        // up-left
        current_file = file;
        current_rank = rank;
        while current_rank < 7 && current_file > 0 {
            current_file -= 1;
            current_rank += 1;
            if !self.add_position_if_valid(&mut diagonals, file, rank, current_file, current_rank) {
                break;
            }
        }

        // up-right
        current_file = file;
        current_rank = rank;
        while current_rank < 7 && current_file < 7 {
            current_file += 1;
            current_rank += 1;
            if !self.add_position_if_valid(&mut diagonals, file, rank, current_file, current_rank) {
                break;
            }
        }       

        diagonals
    }

    fn square_empty(&self, file: usize, rank: usize) -> bool {
        self.get_piece(file_rank_to_hex(file, rank)).piece_type == PieceType::Empty
    }

    pub fn simulate_move(&self, old_position: usize, new_position: usize) -> Board {
        let mut simulated_board = self.copy();
        let original_piece = simulated_board.get_piece(old_position);
        let new_piece = simulated_board.get_piece(new_position);

        // handle capture in white/black set
        if new_piece.piece_type != PieceType::Empty {
            match new_piece.piece_color {
                PieceColor::White => {
                    for (i, white_piece) in simulated_board.white_set.clone()
                        .iter().enumerate() {
                            if white_piece.get_position() == new_position {
                                simulated_board.white_set.remove(i);
                            }
                    }
                }
                PieceColor::Black => {
                    for (i, black_piece) in simulated_board.black_set.clone()
                        .iter().enumerate() {
                            if black_piece.get_position() == new_position {
                                simulated_board.black_set.remove(i);
                            }
                        }
                }
                PieceColor::None => {},
            }
        }

        // move piece
        simulated_board.set_piece(old_position, 
                                  DynamicPiece { 
                                      piece_type: PieceType::Empty, 
                                      piece_color: PieceColor::None,
                                      position: old_position 
                                  });
        simulated_board.set_piece(new_position, original_piece);

        simulated_board
    }

    // checks if the move is a capture move
    pub fn is_capturable(&self, old_position: usize, new_position: usize) -> bool {
        let old_position_piece: DynamicPiece = self.get_piece(old_position);
        let new_position_piece: DynamicPiece = self.get_piece(new_position);

        if old_position_piece.piece_color == new_position_piece.piece_color {
            return false;
        }

        let (old_file, old_rank) = file_rank_from_hex(old_position);
        let (new_file, new_rank) = file_rank_from_hex(new_position);

        if self.square_empty(new_file, new_rank) {
            return false;
        }

        match old_position_piece.get_type() {
            PieceType::Pawn => {
                if old_position_piece.piece_color == PieceColor::White {
                    if (old_file > 0 && old_rank < 7 
                        && new_position == file_rank_to_hex(old_file - 1, old_rank + 1))
                        || (old_file < 7 && old_rank < 7 && 
                            new_position == file_rank_to_hex(old_file + 1, old_rank + 1)) {
                            return true;
                        }
                    return false;
                } else {
                    if (old_file > 0 && old_rank > 0 
                        && new_position == file_rank_to_hex(old_file - 1, old_rank - 1))
                        || (old_file < 7 && old_rank > 0 && 
                            new_position == file_rank_to_hex(old_file + 1, old_rank - 1)) {
                            return true;
                        }
                    return false;
                }
            }
            PieceType::Rook | PieceType::King | PieceType::Queen | 
                PieceType::Bishop | PieceType::Knight => {
                    return true;
                }
            _ => {false}
        }
    }

}


fn get_piece_symbol(piece_type: PieceType, piece_color: PieceColor) -> String {
    match piece_color {
        PieceColor::None => "-".to_string(),
        PieceColor::White => {
            match piece_type {
                PieceType::Pawn => "♙".to_string(),
                PieceType::Knight => "♘".to_string(),
                PieceType::Bishop => "♗".to_string(),
                PieceType::Rook => "♖".to_string(),
                PieceType::Queen => "♕".to_string(),
                PieceType::King => "♔".to_string(),
                PieceType::Empty => "-".to_string(),
            }
        }
        PieceColor::Black => {
            match piece_type {
                PieceType::Pawn => '\u{265F}'.to_string(),
                PieceType::Knight => "♞".to_string(),
                PieceType::Bishop => "♝".to_string(),
                PieceType::Rook => "♜".to_string(),
                PieceType::Queen => "♛".to_string(),
                PieceType::King => "♚".to_string(),
                PieceType::Empty => "-".to_string(),
            }
        }
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

// gets the (file, rank) tuple from hex position
pub fn file_rank_from_hex(position: usize) -> (usize, usize) {
    let file = position & 0xF;
    let rank = position >> 4;

    (file, rank)
}

pub fn file_rank_to_hex(file: usize, rank: usize) -> usize {
    rank * 16 + file
}
