use crate::board::*;
use crate::types::*;
use crate::consts::*;
use std::cmp::{min, max};

pub fn evaluate(board: &Board) -> i32 {
    let mut result = 0;
    for square in 0..64 {
        let piece = board.get_piece(square);

        result += match piece.typ {
            PieceType::Pawn   => 1008,
            PieceType::Rook   => 4961,
            PieceType::Knight => 3191,
            PieceType::Bishop => 3266,
            PieceType::Queen  => 9848,
            PieceType::King   => 200000,
            PieceType::Empty  => 0,
        } * match piece.color {
            Color::White => 1,
            Color::Black => -1,
            Color::Empty => 0,
        }
    }

    result
}

fn is_legal_position(board: &mut Board, castling_bitboard: u64) -> bool {
    let own_pieces = if board.turn == Color::White {
        board.black_pieces
    } else {
        board.white_pieces
    };

    let check_moves = board.generate_moves();
    for check_move in check_moves {
        if ((board.kings & own_pieces) | castling_bitboard) & (1 << check_move.end_square) > 0 {
            return false;
        }
    }

    true
}

pub fn search(depth: i32, board: &mut Board) -> Move {
    let best =
        if board.turn == Color::White {
            i32::MIN
        } else {
            i32::MAX
        };
    if depth == -1 {
        min_max(4, board, best)[0].clone().0
    } else {
        min_max(depth, board, best)[0].clone().0
    }
}

pub fn perft(start_depth: i32, depth: i32, board: &mut Board) -> i32 {
    if depth == 0 {
        return 1
    }

    let mut result = 0;

    for mov in board.generate_moves() {
        let mut castling_bitboard = 0;
        if board.get_piece(mov.start_square).typ == PieceType::King 
            && i32::abs((mov.end_square % 8) as i32 - (mov.start_square % 8) as i32) == 2 {
            for square in min(mov.start_square, mov.end_square)..max(mov.start_square, mov.end_square) {
                castling_bitboard |= 1 << square;
            }
        }

        board.make_move(mov.clone());
        if is_legal_position(board, castling_bitboard) {
            let current_move = perft(start_depth, depth - 1, board);
            result += current_move;

            if start_depth == depth {
                println!("{}: {}", Board::print_move(&mov), current_move);
            }
        }
        board.unmake_move(mov);
    }

    result
}

fn min_max(depth: i32, board: &mut Board, parent_score: i32) -> Vec<(Move, i32)> {
    if depth == 0 {
        vec![(EMPTY_MOVE, evaluate(board))]
    } else {
        let moves = board.generate_moves();
        let mut result = Vec::new();
        let mut best =
            if board.turn == Color::White {
                i32::MIN
            } else {
                i32::MAX
            };

        for mov in moves.clone() {
            let mut castling_bitboard = 0;
            if board.get_piece(mov.start_square).typ == PieceType::King 
                && i32::abs((mov.end_square % 8) as i32 - (mov.start_square % 8) as i32) == 2 {
                for square in min(mov.start_square, mov.end_square)..max(mov.start_square, mov.end_square) {
                    castling_bitboard |= 1 << square;
                }
            }

            board.make_move(mov.clone());
            
            if is_legal_position(board, castling_bitboard) {
                let min_max_result = min_max(depth - 1, board, best);

                if min_max_result.len() > 0 {
                    result.push((mov.clone(), min_max_result[0].1));
                    
                    if ()
                    if board.turn == Color::White {
                        best = i32::min(best, min_max_result[0].1);
                        if parent_score > best {
                            board.unmake_move(mov);
                            break;
                        }
                    } else {
                        best = i32::max(best, min_max_result[0].1);
                        if parent_score < best {
                            board.unmake_move(mov);
                            break;
                        }
                    }
                }
            }

            board.unmake_move(mov);
        }

        result.sort_by(|(_, score1), (_, score2)| 
            if board.turn == Color::White {
                score2.partial_cmp(score1).unwrap()
            } else {
                score1.partial_cmp(score2).unwrap()
            }
        );

        result
    }
}
