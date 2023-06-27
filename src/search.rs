use crate::PositionCounter;
use crate::board::Board;
use crate::board::PieceType;

pub fn search(board: &Board, depth: usize,  maximizing_player: bool,
        position_counter: &mut PositionCounter) -> isize {
    if depth == 0 {
        position_counter.add();
        println!("{}", position_counter.positions);
        return evaluate(board);
    }

    if maximizing_player {
        let mut max = isize::MIN;
        for (original_position, potential_positions) in board.get_board_legal_moves(true) {
            for potential_position in potential_positions {
                let simulated_move_board = board.simulate_move(
                    original_position, potential_position);
                let score = search(&simulated_move_board, depth - 1, 
                                   !maximizing_player, position_counter);
                if score > max {
                    max = score;
                }
            }
        }
        return max;
    } else {
        let mut min = isize::MAX;
        for (original_position, potential_positions) in board.get_board_legal_moves(false) {
            for potential_position in potential_positions {
                let simulated_move_board = board.simulate_move(
                    original_position, potential_position);
                let score = search(&simulated_move_board, depth - 1, 
                                   !maximizing_player, position_counter);
                if score < min {
                    min = score;
                }
            }
        }

        return min;
    }
}

fn evaluate(board: &Board) -> isize {
    let mut white_weight = 0;
    let mut black_weight = 0;

    for white_piece in board.get_white_pieces() {
        white_weight += match white_piece.get_type() {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Queen => 9,
            _ => 0,
        };
    }

    for black_piece in board.get_black_pieces() {
        black_weight -= match black_piece.get_type() {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Queen => 9,
            _ => 0,
        };
    }

    white_weight + black_weight
}
