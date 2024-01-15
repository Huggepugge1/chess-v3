use crate::types::*;
use crate::consts::*;
use crate::board::Board;

impl Board {
    pub fn load_fen(&mut self, fen: String) {
        *self = EMPTY_BOARD;
        let mut parts = fen.split(" ");
        let pieces = parts.next().unwrap();
        let turn = parts.next().unwrap();
        let castling = parts.next().unwrap();
        let en_passant = parts.next().unwrap();
        let halfmove_clock = parts.next().unwrap();
        let fullmove_clock = parts.next().unwrap();
        let mut pos: Square = 56;

        for piece in pieces.chars() {
            if piece == '/' {
                continue;
            } else if piece.is_digit(10) {
                pos += piece as Square - '0' as Square;
            } else {
                if piece.is_uppercase() {
                    self.white_pieces |= 1 << pos;
                    match piece {
                        'P' => self.pawns   |= 1 << pos,
                        'R' => self.rooks   |= 1 << pos,
                        'N' => self.knights |= 1 << pos,
                        'B' => self.bishops |= 1 << pos,
                        'Q' => self.queens  |= 1 << pos,
                        'K' => self.kings   |= 1 << pos,
                        _ => (),
                    }
                } else {
                    self.black_pieces |= 1 << pos;
                    match piece {
                        'p' => self.pawns   |= 1 << pos,
                        'r' => self.rooks   |= 1 << pos,
                        'n' => self.knights |= 1 << pos,
                        'b' => self.bishops |= 1 << pos,
                        'q' => self.queens  |= 1 << pos,
                        'k' => self.kings   |= 1 << pos,
                        _ => (),
                    }
                }
                pos += 1;
            }
            if pos > 8 && pos % 8 == 0 {
                pos -= 16;
            }
        }

        match turn {
            "w" => self.turn = Color::White,
            _ => self.turn = Color::Black,
        }

        for castling_right in castling.chars() {
            match castling_right {
                'K' => self.castling_rights.0.0 = true,
                'Q' => self.castling_rights.0.1 = true,
                'k' => self.castling_rights.1.0 = true,
                'q' => self.castling_rights.1.1 = true,
                _ => (),
            }
        }

        if en_passant != "-" {
            self.en_passant = Board::string_to_square(en_passant.to_string());
        } else {
            self.en_passant = 64;
        }

        self.halfmove_clock = halfmove_clock.parse::<Clock>().unwrap();
        self.fullmove_clock = fullmove_clock.parse::<Clock>().unwrap();
    }
}
