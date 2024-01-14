use crate::board::*;
use crate::types::*;
use crate::consts::*;
use std::cmp::{min, max};

pub fn evaluate(board: &Board) -> f32 {
    let mut result: f32 = 0.0;
    for square in 0..64 {
        let piece = board.get_piece(square);

        result += match piece.typ {
            PieceType::Pawn   => 1.008,
            PieceType::Rook   => 4.961,
            PieceType::Knight => 3.191,
            PieceType::Bishop => 3.266,
            PieceType::Queen  => 9.848,
            PieceType::King   => 200.0,
            PieceType::Empty  => 0.0,
        } * match piece.color {
            Color::White => 1.0,
            Color::Black => -1.0,
            Color::Empty => 1.0,
        }
    }

    result
}

pub fn search(depth: i32, board: &mut Board) -> Move {
    if depth == -1 {
        min_max(4, board)[0].clone().0
    } else {
        min_max(depth, board)[0].clone().0
    }
}

pub fn perft(depth: i32, board: &mut Board) -> i32 {
    if depth == 0 {
        return 1
    }

    let mut result = 0;

    let own_pieces = if board.turn == Color::White {
        board.white_pieces
    } else {
        board.black_pieces
    };
    for mov in board.generate_moves() {
        let mut castling_bitboard = 0;
        if board.get_piece(mov.start_square).typ == PieceType::King 
            && i32::abs((mov.end_square % 8) as i32 - (mov.start_square % 8) as i32) == 2 {
            for square in min(mov.start_square, mov.end_square)..max(mov.start_square, mov.end_square) {
                castling_bitboard |= 1 << square;
            }
        }

        board.make_move(mov.clone());
        if is_legal_position(board, own_pieces, castling_bitboard) {
            result += perft(depth - 1, board);
        }
        board.unmake_move(mov);
    }

    board.print_board();

    result
}

fn is_legal_position(board: &mut Board, own_pieces: u64, castling_bitboard: u64) -> bool {
    let check_moves = board.generate_moves();
    for check_mov in check_moves {
        if ((board.kings & own_pieces) | castling_bitboard) & (1 << check_mov.end_square) > 0 {
            return false;
        }
    }

    true
}

fn min_max(depth: i32, board: &mut Board) -> Vec<(Move, f32)> {
    if depth == 0 {
        vec![(EMPTY_MOVE, evaluate(board))]
    } else {
        let moves = board.generate_moves();
        let mut result: Vec<(Move, f32)> = Vec::new();
        let own_pieces = if board.turn == Color::White {
            board.white_pieces
        } else {
            board.black_pieces
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
            
            if is_legal_position(board, own_pieces, castling_bitboard) {
                let min_max_result = min_max(depth - 1, board);

                if min_max_result.len() > 0 {
                    result.push((mov.clone(), min_max_result[0].1));
                }
            }

            board.unmake_move(mov.clone());
        }

        result.sort_by(|(_, score1), (_, score2)| 
            if board.turn == Color::White {
                score1.partial_cmp(score2).unwrap()
            } else {
                score2.partial_cmp(score1).unwrap()
            }
        );

        result
    }
}
