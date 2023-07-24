mod board;

fn main() {
   let board = board::BoardState::from_fen(board::DEFAULT_FEN_STRING).unwrap();
   board.pretty_print();
}
