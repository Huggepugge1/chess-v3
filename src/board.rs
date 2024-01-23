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

    //              EP      Castling  HM     Captured
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

    pub fn print_board(&self) {
        println!(" --- --- --- --- --- --- --- ---");
        for i in 0..8 {
            print!("|");
            for j in 0..8 {
                print!(" {} |", Self::converter(self.get_piece(63 - ((i * 8) + (7 - j)))));
            }
            println!("");
            println!(" --- --- --- --- --- --- --- ---");
        }
    }

    pub fn converter(piece: Piece) -> char {
        let chr: char = match piece.typ {
            PieceType::Pawn   => 'p',
            PieceType::Rook   => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen  => 'q',
            PieceType::King   => 'k',
            PieceType::Empty  => '_',
        };

        match piece.color {
            Color::White => chr.to_ascii_uppercase(),
            _ => chr
        }
    }

    pub fn print_move(mov: &Move) -> String {
        format!(
            "{}{}",
            Self::square_to_string(mov.start_square),
            Self::square_to_string(mov.end_square)
        )
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
            } else if self.knights & (1 << square) > 0 {
                PieceType::Knight
            } else if self.bishops & (1 << square) > 0 {
                PieceType::Bishop
            } else if self.queens & (1 << square) > 0 {
                PieceType::Queen
            } else {
                PieceType::King
            };

        return Piece {
            typ: typ,
            color: color,
        }
    }

    fn change_turn(&mut self) {
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }

    pub fn make_move(&mut self, mov: Move) {
        let last_en_passant = self.en_passant;
        let last_castling_rights = self.castling_rights;
        let last_halfmove_clock = self.halfmove_clock;

        let Move {start_square, end_square, promotion} = mov;
        
        let start_piece = self.get_piece(start_square);
        let end_piece   = self.get_piece(end_square);

        let from_to_bb = (1 << start_square) ^ (1 << end_square);
        match start_piece.color {
            Color::White => self.white_pieces ^= from_to_bb,
            Color::Black => self.black_pieces ^= from_to_bb,
            Color::Empty => panic!("Tried to move an empty piece!"),
        }
        
        match start_piece.typ {
            PieceType::Pawn   => self.pawns   ^= from_to_bb,
            PieceType::Rook   => self.rooks   ^= from_to_bb,
            PieceType::Knight => self.knights ^= from_to_bb,
            PieceType::Bishop => self.bishops ^= from_to_bb,
            PieceType::Queen  => self.queens  ^= from_to_bb,
            PieceType::King   => self.kings   ^= from_to_bb,
            PieceType::Empty  => panic!("Tried to move an empty piece!"),
        }
        
        if end_piece != EMPTY_PIECE {
            match end_piece.color {
                Color::White => self.white_pieces ^= 1 << end_square,
                Color::Black => self.black_pieces ^= 1 << end_square,
                Color::Empty => panic!("Tried to capture an empty piece!"),
            }
            
            match end_piece.typ {
                PieceType::Pawn   => self.pawns   ^= 1 << end_square,
                PieceType::Rook   => self.rooks   ^= 1 << end_square,
                PieceType::Knight => self.knights ^= 1 << end_square,
                PieceType::Bishop => self.bishops ^= 1 << end_square,
                PieceType::Queen  => self.queens  ^= 1 << end_square,
                PieceType::King   => self.kings   ^= 1 << end_square,
                PieceType::Empty  => panic!("Tried to capture an empty piece!"),
            }
        }

        if promotion != PieceType::Empty {
            self.pawns ^= 1 << end_square;
            match promotion {
                PieceType::Rook   => self.rooks   ^= 1 << end_square,
                PieceType::Knight => self.knights ^= 1 << end_square,
                PieceType::Bishop => self.bishops ^= 1 << end_square,
                PieceType::Queen  => self.queens  ^= 1 << end_square,
                _  => panic!("Tried to promote to an invalid piece!"),
            }
        }

        // En passant capture
        if start_piece.typ == PieceType::Pawn && end_piece == EMPTY_PIECE 
            && (start_square % 8) as i32 - (end_square % 8) as i32 != 0 {
            let enemy_pos = 1 << (start_square + end_square % 8 - 8);
            self.pawns &= !enemy_pos;
            
            match start_piece.color {
                Color::White => self.black_pieces ^= enemy_pos,
                Color::Black => self.white_pieces ^= enemy_pos,
                Color::Empty => panic!("Tried to capture an empty piece!"),
            }
        }

        // En passant detection
        if start_piece.typ == PieceType::Pawn && i32::abs(start_square as i32 - end_square as i32) == 16 {
            self.en_passant = (start_square + end_square) / 2;
        } else {
            self.en_passant = 64;
        }

        // Castling
        if start_piece.typ == PieceType::King && i32::abs(start_square as i32 - end_square as i32) == 2 {
            if end_square == 2 {
                self.make_move(Move::new(0, 3, PieceType::Empty));
            } else if end_square == 6 {
                self.make_move(Move::new(7, 5, PieceType::Empty));
            } else if end_square == 62 {
                self.make_move(Move::new(63, 61, PieceType::Empty));
            } else {
                self.make_move(Move::new(56, 59, PieceType::Empty));
            }

            self.change_turn();
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
                if start_square == 7 {
                    self.castling_rights.0.0 = false;
                }
                if start_square == 0 {
                    self.castling_rights.0.1 = false;
                }
            } else {
                if start_square == 63 {
                    self.castling_rights.1.0 = false;
                }
                if start_square == 56 {
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

        self.change_turn();
        self.moves.push((last_en_passant, last_castling_rights, last_halfmove_clock, end_piece));
    }

    pub fn unmake_move(&mut self, mov: Move) {
        let start_kings = self.kings;
        let captured_piece;
        (
            self.en_passant,
            self.castling_rights,
            self.halfmove_clock,
            captured_piece
        ) = self.moves.pop().unwrap();
        let Move {start_square, end_square, promotion} = mov;
        
        let piece = self.get_piece(end_square);

        let from_to_bb = (1 << start_square) ^ (1 << end_square);

        match piece.color {
            Color::White => self.white_pieces ^= from_to_bb,
            Color::Black => self.black_pieces ^= from_to_bb,
            Color::Empty => panic!("Tried to move an empty piece!"),
        }

        // Piece did not promote
        if promotion == PieceType::Empty {
            match piece.typ {
                PieceType::Pawn   => self.pawns   ^= from_to_bb,
                PieceType::Rook   => self.rooks   ^= from_to_bb,
                PieceType::Knight => self.knights ^= from_to_bb,
                PieceType::Bishop => self.bishops ^= from_to_bb,
                PieceType::Queen  => self.queens  ^= from_to_bb,
                PieceType::King   => self.kings   ^= from_to_bb,
                PieceType::Empty  => panic!("Tried to move an empty piece!"),
            }
        // Piece has promoted
        } else {
            self.pawns ^= 1 << start_square;
            match piece.typ {
                PieceType::Rook   => self.rooks   ^= 1 << end_square,
                PieceType::Knight => self.knights ^= 1 << end_square,
                PieceType::Bishop => self.bishops ^= 1 << end_square,
                PieceType::Queen  => self.queens  ^= 1 << end_square,
                _ => panic!("Tried to remove an empty piece!"),
            }
        }
         
        if captured_piece != EMPTY_PIECE {
            match captured_piece.color {
                Color::White => self.white_pieces ^= 1 << end_square,
                Color::Black => self.black_pieces ^= 1 << end_square,
                Color::Empty => panic!("Tried to restore an empty piece!"),
            }
            
            match captured_piece.typ {
                PieceType::Pawn   => self.pawns   ^= 1 << end_square,
                PieceType::Rook   => self.rooks   ^= 1 << end_square,
                PieceType::Knight => self.knights ^= 1 << end_square,
                PieceType::Bishop => self.bishops ^= 1 << end_square,
                PieceType::Queen  => self.queens  ^= 1 << end_square,
                PieceType::King   => self.kings   ^= 1 << end_square,
                PieceType::Empty  => panic!("Tried to restore an empty piece!"),
            }
        }
        if start_kings != self.kings {
            println!("{:?}", mov);
        }
        self.change_turn();
    }
}
