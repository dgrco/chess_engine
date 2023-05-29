mod board;

fn main() {
    let mut board = board::Board::new();
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    board.init_board_from_fen(fen);
    board.print_board();
}
