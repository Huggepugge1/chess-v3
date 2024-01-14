use crate::board::*;
use crate::types::*;
use crate::attack_bitboards::*;

impl Board {
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();

        result.extend(self.generate_pawn_moves());
        result.extend(self.generate_rook_moves());
        result.extend(self.generate_knight_moves());
        result.extend(self.generate_bishop_moves());
        result.extend(self.generate_queen_moves());
        result.extend(self.generate_king_moves());
        
        println!("{self:?}");
        println!("{result:?}");
        result
    }

    pub fn generate_positive_ray_moves(&self, rays: [u64; 64], square: Square, flipped_own_pieces: u64) -> u64 {
        let occupied = self.white_pieces | self.black_pieces;
        let intersection = occupied & rays[square];
        if intersection == 0 {
            rays[square]
        } else {
            (rays[square] ^ rays[intersection.trailing_zeros() as Square]) & flipped_own_pieces
        }
    }

    pub fn generate_negative_ray_moves(&self, rays: [u64; 64], square: Square, flipped_own_pieces: u64) -> u64 {
        let occupied = self.white_pieces | self.black_pieces;
        let intersection = occupied & rays[square];
        if intersection == 0 {
            rays[square]
        } else {
            (rays[square] ^ rays[intersection.leading_zeros() as Square ^ 63]) & flipped_own_pieces
        }
    }

    pub fn generate_pawn_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let own_pieces = if self.turn == Color::White {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let pawns = own_pieces & self.pawns;
            
        for start_square in 0..64 {
            if pawns & (1 << start_square) > 0 {
                let push_bitboard = if self.turn == Color::White {
                    WHITE_PAWN_PUSHES[start_square]
                } else {
                    BLACK_PAWN_PUSHES[start_square]
                } & !(self.white_pieces | self.black_pieces);
 
                let en_passant_bitboard = if self.en_passant < 64 {
                    1 << self.en_passant
                } else {
                    0
                };

                let attack_bitboard = if self.turn == Color::White {
                    WHITE_PAWN_ATTACKS[start_square] & (self.black_pieces | en_passant_bitboard)
                } else {
                    BLACK_PAWN_ATTACKS[start_square] & (self.white_pieces | en_passant_bitboard)
                }; 

                for end_square in 0..64 {
                    let promote_bitboard: u64 = if self.turn == Color::White {
                        0xff
                    } else {
                        0xff << 56
                    };
                    if push_bitboard & (1 << end_square) > 0 {
                        if 1 << end_square & promote_bitboard > 0 {
                            for piece in [
                                PieceType::Rook,
                                PieceType::Knight,
                                PieceType::Bishop,
                                PieceType::Queen,
                            ] {
                                moves.push(Move::new(start_square, end_square, piece));
                            }
                        } else {
                            moves.push(Move::new(start_square, end_square, PieceType::Empty));
                        }
                    } else if attack_bitboard & (1 << end_square) > 0 {
                        if 1 << end_square & promote_bitboard > 0 {
                            for piece in [
                                PieceType::Rook,
                                PieceType::Knight,
                                PieceType::Bishop,
                                PieceType::Queen,
                            ] {
                                moves.push(Move::new(start_square, end_square, piece));
                            }
                        } else {
                            moves.push(Move::new(start_square, end_square, PieceType::Empty));
                        }
                    }
                }
            }
        }
        moves
    }
    
    pub fn generate_rook_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let own_pieces = if self.turn == Color::White {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let rooks = own_pieces & self.rooks;
            
        for start_square in 0..64 {
            if rooks & (1 << start_square) > 0 {
                let attack_bitboard = 
                    self.generate_positive_ray_moves(EAST_RAYS, start_square, !own_pieces)
                    | self.generate_positive_ray_moves(NORTH_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(WEST_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(SOUTH_RAYS, start_square, !own_pieces);
                for end_square in 0..64 {
                    if attack_bitboard & (1 << end_square) > 0 {
                        moves.push(Move::new(start_square, end_square, PieceType::Empty));
                    }
                }
            }
        }
        if moves.len() >= 1 {
            println!("{self:?}\n{moves:?}");
            self.print_board();
        }
        moves
    }

    pub fn generate_knight_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let own_pieces = if self.turn == Color::White {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let knights = own_pieces & self.knights;
            
        for start_square in 0..64 {
            if knights & (1 << start_square) > 0 {
                let attack_bitboard = KNIGHT_ATTACK_BITBOARDS[start_square] & !own_pieces;
                for end_square in 0..64 {
                    if attack_bitboard & (1 << end_square) > 0 {
                        moves.push(Move::new(start_square, end_square, PieceType::Empty));
                    }
                }
            }
        }
        moves
    }

    pub fn generate_bishop_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let own_pieces = if self.turn == Color::White {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let bishops = own_pieces & self.bishops;
            
        for start_square in 0..64 {
            if bishops & (1 << start_square) > 0 {
                let attack_bitboard = 
                    self.generate_positive_ray_moves(NORTH_EAST_RAYS, start_square, !own_pieces)
                    | self.generate_positive_ray_moves(NORTH_WEST_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(SOUTH_WEST_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(SOUTH_EAST_RAYS, start_square, !own_pieces);
                for end_square in 0..64 {
                    if attack_bitboard & (1 << end_square) > 0 {
                        moves.push(Move::new(start_square, end_square, PieceType::Empty));
                    }
                }
            }
        }
        moves
    }

    pub fn generate_queen_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let own_pieces = if self.turn == Color::White {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let queens = own_pieces & self.queens;
            
        for start_square in 0..64 {
            if queens & (1 << start_square) > 0 {
                let attack_bitboard = 
                    self.generate_positive_ray_moves(EAST_RAYS, start_square, !own_pieces)
                    | self.generate_positive_ray_moves(NORTH_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(WEST_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(SOUTH_RAYS, start_square, !own_pieces)
                    | self.generate_positive_ray_moves(NORTH_EAST_RAYS, start_square, !own_pieces)
                    | self.generate_positive_ray_moves(NORTH_WEST_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(SOUTH_WEST_RAYS, start_square, !own_pieces)
                    | self.generate_negative_ray_moves(SOUTH_EAST_RAYS, start_square, !own_pieces);
                for end_square in 0..64 {
                    if attack_bitboard & (1 << end_square) > 0 {
                        moves.push(Move::new(start_square, end_square, PieceType::Empty));
                    }
                }
            }
        }
        moves
    }

    pub fn generate_king_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let own_pieces = if self.turn == Color::White {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let kings = own_pieces & self.kings;
            
        for start_square in 0..64 {
            if kings & (1 << start_square) > 0 {
                let attack_bitboard = KING_ATTACK_BITBOARDS[start_square] & !own_pieces;
                for end_square in 0..64 {
                    if attack_bitboard & (1 << end_square) > 0 {
                        moves.push(Move::new(start_square, end_square, PieceType::Empty));
                    }
                }
            }
        }

        if self.turn == Color::White {
            if self.castling_rights.0.0
                && 0b01100000 & (self.white_pieces | self.black_pieces) == 0 {
                moves.push(Move::new(4, 6, PieceType::Empty));
            }
            
            if self.castling_rights.0.1
                && 0b00001110 & (self.white_pieces | self.black_pieces) == 0 {
                moves.push(Move::new(4, 2, PieceType::Empty));
            }
        } else {
            if self.castling_rights.1.0
                && (0b01100000 << 56) & (self.white_pieces | self.black_pieces) == 0 {
                moves.push(Move::new(60, 62, PieceType::Empty));
            }
            
            if self.castling_rights.1.1
                && (0b00001110 << 56) & (self.white_pieces | self.black_pieces) == 0 {
                moves.push(Move::new(60, 58, PieceType::Empty));
            }
        }
        moves
    }
}
