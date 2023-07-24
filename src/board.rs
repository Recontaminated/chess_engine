pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug)]
enum PieceColor {
    White,
    Black,
}

pub struct BoardState {
    bitboards: BoardBitboards,
    side_to_move: PieceColor,
    white_king_castle: bool,
    white_queen_castle: bool,
    black_king_castle: bool,
    black_queen_castle: bool,
    en_passant_square: Option<u8>,
}
struct BoardBitboards {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
    white_pieces: u64,
    black_pieces: u64,
}

impl BoardBitboards {
    pub fn get_square(&self, pos: u8) -> Option<PieceType> {
        // go through each bitboard and check if the square is set
        // if it is, return the piece type
        // if not, return None
        if self.white_pawns & (1 << pos) != 0 {
            return Some(PieceType::Pawn);
        }
        if self.white_knights & (1 << pos) != 0 {
            return Some(PieceType::Knight);
        }
        if self.white_bishops & (1 << pos) != 0 {
            return Some(PieceType::Bishop);
        }
        if self.white_rooks & (1 << pos) != 0 {
            return Some(PieceType::Rook);
        }
        if self.white_queens & (1 << pos) != 0 {
            return Some(PieceType::Queen);
        }
        if self.white_king & (1 << pos) != 0 {
            return Some(PieceType::King);
        }
        if self.black_pawns & (1 << pos) != 0 {
            return Some(PieceType::Pawn);
        }
        if self.black_knights & (1 << pos) != 0 {
            return Some(PieceType::Knight);
        }
        if self.black_bishops & (1 << pos) != 0 {
            return Some(PieceType::Bishop);
        }
        if self.black_rooks & (1 << pos) != 0 {
            return Some(PieceType::Rook);
        }
        if self.black_queens & (1 << pos) != 0 {
            return Some(PieceType::Queen);
        }
        if self.black_king & (1 << pos) != 0 {
            return Some(PieceType::King);
        }

        return None;
    }
}
impl BoardState {
    pub fn from_fen(fen: &str) -> Result<BoardState, &str> {
        let mut board = BoardBitboards {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0,
            white_pieces: 0,
            black_pieces: 0,
        };
        let mut board_state = BoardState {
            bitboards: board,
            en_passant_square: None,
            side_to_move: PieceColor::White,
            white_king_castle: false,
            white_queen_castle: false,
            black_king_castle: false,
            black_queen_castle: false,
        };

        let fen_parts: Vec<&str> = fen.split(" ").collect();
        if fen_parts.len() != 6 {
            return Err("FEN string must have 6 parts");
        }

        board_state.side_to_move = match fen_parts[1] {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => return Err("Invalid side to move"),
        };

        for letter in fen_parts[2].chars() {
            match letter.to_string().as_str() {
                "K" => board_state.white_king_castle = true,
                "Q" => board_state.white_queen_castle = true,
                "k" => board_state.black_king_castle = true,
                "q" => board_state.black_queen_castle = true,
                "-" => (),
                _ => return Err("Invalid castling rights"),
            }
        }
        if fen_parts[3] != "-" {
            let first_char = fen_parts[3].chars().next().unwrap();
            let second_char = fen_parts[3].chars().nth(1).unwrap();
            let en_passant_square = match (first_char.to_digit(10), second_char.to_digit(10)) {
                (Some(file), Some(rank)) => {
                    if file < 1 || file > 8 || rank < 1 || rank > 8 {
                        return Err("Invalid en passant square");
                    }
                    (file - 1) + (rank - 1) * 8
                }
                _ => return Err("Invalid en passant square"),
            };

            board_state.en_passant_square = Some(en_passant_square as u8);
        }

        // parse the position
        let rows: Vec<&str> = fen_parts[0].split("/").collect();
        if rows.len() != 8 {
            return Err("Invalid number of rows");
        }
        let mut cur_row = 0;
        let mut curCol = 0;
        for row in rows {
            for square in row.chars() {
                print!("current row is {}, current col is {}\n", cur_row, curCol);
                match square.to_digit(10) {
                    Some(num) => {
                        curCol += num as usize;
                    }
                    None => {
                        match square {
                            'P' => board_state.bitboards.white_pawns |= 1 << (cur_row * 8 + curCol),
                            'N' => {
                                board_state.bitboards.white_knights |= 1 << (cur_row * 8 + curCol)
                            }
                            'B' => {
                                board_state.bitboards.white_bishops |= 1 << (cur_row * 8 + curCol)
                            }
                            'R' => board_state.bitboards.white_rooks |= 1 << (cur_row * 8 + curCol),
                            'Q' => {
                                board_state.bitboards.white_queens |= 1 << (cur_row * 8 + curCol)
                            }
                            'K' => board_state.bitboards.white_king |= 1 << (cur_row * 8 + curCol),
                            'p' => board_state.bitboards.black_pawns |= 1 << (cur_row * 8 + curCol),
                            'n' => {
                                board_state.bitboards.black_knights |= 1 << (cur_row * 8 + curCol)
                            }
                            'b' => {
                                board_state.bitboards.black_bishops |= 1 << (cur_row * 8 + curCol)
                            }
                            'r' => board_state.bitboards.black_rooks |= 1 << (cur_row * 8 + curCol),
                            'q' => {
                                board_state.bitboards.black_queens |= 1 << (cur_row * 8 + curCol)
                            }
                            'k' => board_state.bitboards.black_king |= 1 << (cur_row * 8 + curCol),
                            _ => return Err("Invalid piece type"),
                        }
                        curCol += 1;
                    }
                }
            }

            cur_row += 1;
            curCol = 0;
        }

        return Ok(board_state);
    }
    pub fn pretty_print(self) {
        println!("Side to move: {:?}", self.side_to_move);
        println!("{:#064b}", self.bitboards.white_pawns);
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = self.bitboards.get_square(file + rank * 8);
                match square {
                    Some(PieceType::Pawn) => print!("P"),
                    Some(PieceType::Knight) => print!("N"),
                    Some(PieceType::Bishop) => print!("B"),
                    Some(PieceType::Rook) => print!("R"),
                    Some(PieceType::Queen) => print!("Q"),
                    Some(PieceType::King) => print!("K"),
                    None => print!("-"),
                }
            }
            println!("");
        }
    }
}
