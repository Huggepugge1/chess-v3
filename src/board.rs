use crate::types::*;
use crate::consts::*;

#[derive(Clone, Debug)]
pub struct Board {
    pub white_pieces: u64,
    pub black_pieces: u64,
    pub pawns: u64,
    pub rooks: u64,
    pub knights: u64,
    pub bishops: u64,
    pub queens: u64,
    pub kings: u64,

    pub turn: Color,
    pub castling_rights: Castling,
    pub en_passant: Square,
    pub halfmove_clock: Clock,
    pub fullmove_clock: Clock,

    pub moves: Vec<(Square, Castling, Clock, Piece)>,
}

impl Board {
    pub fn square_to_string(square: Square) -> String {
        (('a' as Square + (square % 8)) as u8 as char).to_string()
            + &(('1' as Square + (square / 8)) as u8 as char).to_string()
    }

    pub fn string_to_square(string: String) -> Square {
        (string.chars().nth(0).unwrap() as Square - 'a' as Square)
            + (string.chars().nth(1).unwrap() as Square - '1' as Square) * 8
    }

    pub fn get_piece(&self, square: Square) -> Piece {
        let color: Color =
            if self.white_pieces & (1 << square) > 0 {
                Color::White
            } else if self.black_pieces & (1 << square) > 0 {
                Color::Black
            } else {
                Color::Empty
            };

        let typ: PieceType =
            if color == Color::Empty {
                PieceType::Empty
            } else if self.pawns & (1 << square) > 0 {
                PieceType::Pawn
            } else if self.rooks & (1 << square) > 0 {
                PieceType::Rook
            } else if self.pawns & (1 << square) > 0 {
                PieceType::Knight
            } else if self.pawns & (1 << square) > 0 {
                PieceType::Bishop
            } else if self.pawns & (1 << square) > 0 {
                PieceType::Queen
            } else {
                PieceType::King
            };

        return Piece {
            typ: typ,
            color: color,
        }
    }

    pub fn make_move(&mut self, mov: Move, promotion: PieceType) {
        let (start, end) = mov;
        
        let start_piece = self.get_piece(start);
        let end_piece   = self.get_piece(end);

        let from_to_bb = (1 << start) ^ (1 << end);
        match start_piece.color {
            Color::White => self.white_pieces ^= from_to_bb,
            Color::Black => self.black_pieces ^= from_to_bb,
            Color::Empty => panic!("Tried to move a Empty piece!"),
        }
        
        match start_piece.typ {
            PieceType::Pawn   => self.pawns   ^= from_to_bb,
            PieceType::Rook   => self.rooks   ^= from_to_bb,
            PieceType::Knight => self.knights ^= from_to_bb,
            PieceType::Bishop => self.bishops ^= from_to_bb,
            PieceType::Queen  => self.queens  ^= from_to_bb,
            PieceType::King   => self.kings   ^= from_to_bb,
            PieceType::Empty  => panic!("Tried to move a Empty piece!"),
        }
        
        if end_piece != EMPTY_PIECE {
            match end_piece.color {
                Color::White => self.white_pieces ^= 1 << end,
                Color::Black => self.black_pieces ^= 1 << end,
                Color::Empty => panic!("Tried to capture a Empty piece!"),
            }
            
            match end_piece.typ {
                PieceType::Pawn   => self.pawns   ^= 1 << end,
                PieceType::Rook   => self.rooks   ^= 1 << end,
                PieceType::Knight => self.knights ^= 1 << end,
                PieceType::Bishop => self.bishops ^= 1 << end,
                PieceType::Queen  => self.queens  ^= 1 << end,
                PieceType::King   => self.kings   ^= 1 << end,
                PieceType::Empty  => panic!("Tried to capture a Empty piece!"),
            }
        }

        if promotion != PieceType::Empty {
            self.pawns &= !(1 << end);
            match promotion {
                PieceType::Rook   => self.rooks   ^= 1 << end,
                PieceType::Knight => self.knights ^= 1 << end,
                PieceType::Bishop => self.bishops ^= 1 << end,
                PieceType::Queen  => self.queens  ^= 1 << end,
                _  => panic!("Tried to promote to an invalid piece!"),
            }
        }

        // En passant capture
        if start_piece.typ == PieceType::Pawn && end_piece == EMPTY_PIECE && (start + end) % 8 != 0 {
            let enemy_pos = 1 << (start + end % 8 - 8);
            self.pawns &= !enemy_pos;
            
            match start_piece.color {
                Color::White => self.black_pieces ^= enemy_pos,
                Color::Black => self.white_pieces ^= enemy_pos,
                Color::Empty => panic!("Tried to capture a Empty piece!"),
            }
        }

        // En passant detection
        if start_piece.typ == PieceType::Pawn && i32::abs(start as i32 - end as i32) == 16 {
            self.en_passant = (start + end) / 2;
        }

        // Castling
        if start_piece.typ == PieceType::King && i32::abs(start as i32 - end as i32) == 2 {
            if end == 2 {
                self.make_move((0, 3), PieceType::Empty);
            } else if end == 6 {
                self.make_move((7, 5), PieceType::Empty);
            } else if end == 62 {
                self.make_move((63, 61), PieceType::Empty);
            } else {
                self.make_move((56, 59), PieceType::Empty);
            }
        }

        // Removing castling rights: Moving king
        if start_piece.typ == PieceType::King {
            if start_piece.color == Color::White {
                self.castling_rights.0.0 = false;
                self.castling_rights.0.1 = false;
            } else {
                self.castling_rights.1.0 = false;
                self.castling_rights.1.1 = false;
            }
        }

        // Removing castling rights: Moving a rook
        if start_piece.typ == PieceType::Rook {
            if start_piece.color == Color::White {
                if start == 7 {
                    self.castling_rights.0.0 = false;
                }
                if start == 0 {
                    self.castling_rights.0.1 = false;
                }
            } else {
                if start == 63 {
                    self.castling_rights.1.0 = false;
                }
                if start == 56 {
                    self.castling_rights.1.1 = false;
                }
            }
        }

        // Halfmove-clock
        if start_piece.typ == PieceType::Pawn || end_piece != EMPTY_PIECE {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }
}
