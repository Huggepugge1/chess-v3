use crate::board::*;
use crate::types::*;
use crate::consts::*;
use crate::piece_square_tables;

use std::cmp::{min, max};

use stoppable_thread;
use tokio;

type Stop = stoppable_thread::SimpleAtomicBool;

pub fn evaluate(board: &mut Board) -> i32 {
    let mut result = 0;
    let mut queens = 0;
    let mut minors = 0;

    for square in 0..64 {
        let piece = board.get_piece(square);

        result += match piece.typ {
            PieceType::Pawn  => 100 + match piece.color {
                Color::White => piece_square_tables::WHITE_PAWN[square],
                Color::Black => piece_square_tables::BLACK_PAWN[square],
                Color::Empty => 0,
            },
            PieceType::Rook  => 500 + match piece.color {
                Color::White => piece_square_tables::WHITE_ROOK[square],
                Color::Black => piece_square_tables::BLACK_ROOK[square],
                Color::Empty => 0,
            },
            PieceType::Knight => 320 + match piece.color {
                Color::White => piece_square_tables::WHITE_KNIGHT[square],
                Color::Black => piece_square_tables::BLACK_KNIGHT[square],
                Color::Empty => 0,
            },
            PieceType::Bishop => 330 + match piece.color {
                Color::White => piece_square_tables::WHITE_BISHOP[square],
                Color::Black => piece_square_tables::BLACK_BISHOP[square],
                Color::Empty => 0,
            },
            PieceType::Queen  => 900 + match piece.color {
                Color::White => piece_square_tables::WHITE_QUEEN[square],
                Color::Black => piece_square_tables::BLACK_QUEEN[square],
                Color::Empty => 0,
            },
            _  => 0,
        } * match piece.color {
            Color::White => 1,
            Color::Black => -1,
            Color::Empty => 0,
        };

        match piece.typ {
            PieceType::Rook   => minors += 1,
            PieceType::Knight => minors += 1,
            PieceType::Bishop => minors += 1,
            PieceType::Queen  => queens += 1,
            _  => (),
        }
    }
    
    for square in 0..64 {
        let piece = board.get_piece(square);
        if queens == 0 || (queens <= 2 && minors <= 2) {
            result += match piece.typ {
                PieceType::King  => 900 + match piece.color {
                    Color::White => piece_square_tables::WHITE_KING_END_GAME[square],
                    Color::Black => piece_square_tables::BLACK_KING_END_GAME[square],
                    Color::Empty => 0,
                },
                _  => 0,
            } * match piece.color {
                Color::White => 1,
                Color::Black => -1,
                Color::Empty => 0,
            };
        } else {
            result += match piece.typ {
                PieceType::King  => 900 + match piece.color {
                    Color::White => piece_square_tables::WHITE_KING[square],
                    Color::Black => piece_square_tables::BLACK_KING[square],
                    Color::Empty => 0,
                },
                _  => 0,
            } * match piece.color {
                Color::White => 1,
                Color::Black => -1,
                Color::Empty => 0,
            };
        }
    }

    result
}

fn is_check(board: &mut Board, check_bitboard: u64) -> bool {
    let check_moves = board.generate_moves();
    for check_move in check_moves {
        if check_bitboard & (1 << check_move.end_square) > 0 {
            return false;
        }
    }

    true
}

fn is_legal_position(board: &mut Board, castling_bitboard: u64) -> bool {
    let own_pieces = if board.turn == Color::White {
        board.black_pieces
    } else {
        board.white_pieces
    };

    is_check(board, (board.kings & own_pieces) | castling_bitboard)
}

pub async fn search(max_depth: i32, mut time: u64, board: &mut Board, alpha_beta: bool) -> Move {
    let best =
        if board.turn == Color::White {
            i32::MAX
        } else {
            i32::MIN
        };
    if time == 0 {
        time = u64::MAX;
    }
    
    let mut board = board.clone();
    let mut transposition_table: TranspositionTable = TranspositionTable::new();
    let handle = stoppable_thread::spawn(move |stopped| {
        let mut depth = 1;
        let mut moves = board.generate_moves();
        let mut result = Vec::new();
        if max_depth == -1 {
            loop {
                let new_result = min_max(depth, &mut board, moves, best, alpha_beta, &mut transposition_table, stopped);
                moves = new_result.iter().map(|(mov, _)| mov.clone()).collect::<Vec<Move>>();
                if stopped.get() {
                    return result;
                } else {
                    result = new_result;
                }
                println!("info depth {} {}", depth, result.iter().map(|(mov, _)| Board::print_move(mov)).collect::<Vec<String>>().join(" ").replace("  ", " "));
                depth += 1;
            }
        } else {
            while depth < max_depth {
                let new_result = min_max(depth, &mut board, moves, best, alpha_beta, &mut transposition_table, stopped);
                moves = new_result.iter().map(|(mov, _)| mov.clone()).collect::<Vec<Move>>();
                if stopped.get() {
                    return result;
                } else {
                    result = new_result;
                }
                println!("info depth {} {:?}", depth, result.iter().map(|(mov, _)| Board::print_move(mov)).collect::<Vec<String>>());
                depth += 1;
            }
            result
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(time)).await;

    let result = handle.stop().join().unwrap();
    result[0].0.clone()
}

pub async fn alpha_beta_test(board: &mut Board) {
    let depth = 1;
    println!("{}", Board::print_move(&search(depth, 0, board, false).await));
    println!("{}", Board::print_move(&search(depth, 0, board, true).await));
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

fn min_max(depth: i32, board: &mut Board, mut moves: Vec<Move>, parent_score: i32, alpha_beta: bool, transposition_table: &mut TranspositionTable, stopped: &Stop) -> Vec<(Move, i32)> {
    if stopped.get() {
        return Vec::new();
    }
    if depth == 0 {
        vec![(EMPTY_MOVE, evaluate(board))]
    } else {
        let previous_search = transposition_table.get(board);
        match previous_search {
            Some(value) => {
                if value.depth >= depth {
                    return value.result.clone();
                }
            },
            None => (),
        }
        let mut result = Vec::new();
        let mut best =
            if board.turn == Color::White {
                i32::MIN
            } else {
                i32::MAX
            };

        let turn = board.turn;

        moves = moves
            .iter()
            .filter(|mov| {
                board.make_move((*mov).clone());
                let mut castling_bitboard = 0;
                if board.get_piece(mov.start_square).typ == PieceType::King 
                    && i32::abs((mov.end_square % 8) as i32 - (mov.start_square % 8) as i32) == 2 {
                    for square in min(mov.start_square, mov.end_square)..max(mov.start_square, mov.end_square) {
                        castling_bitboard |= 1 << square;
                    }
                }
                let legal = is_legal_position(board, castling_bitboard);
                board.unmake_move((*mov).clone());
                legal
            })
            .map(|mov| mov.clone())
            .collect::<Vec<Move>>();
        
        if moves.len() == 0 {
            let evaluation = i32::MAX * match board.turn {
                Color::White => -1,
                Color::Black => 1,
                Color::Empty => 0,
            };
            return vec![(EMPTY_MOVE, evaluation)];
        }

        for mov in moves.clone() {
            board.make_move(mov.clone());
            
            let moves = board.generate_moves();
            let min_max_result = min_max(depth - 1, board, moves, best, alpha_beta, transposition_table, stopped);

            if min_max_result.len() > 0 {
                result.push((mov.clone(), min_max_result[0].1));
                
                if alpha_beta {
                    if turn == Color::White {
                        best = i32::max(best, min_max_result[0].1);
                        if parent_score < best {
                            board.unmake_move(mov);
                            break;
                        }
                    } else {
                        best = i32::min(best, min_max_result[0].1);
                        if parent_score > best {
                            board.unmake_move(mov);
                            break;
                        }
                    }
                }
            }

            board.unmake_move(mov);
        }
        
        result.sort_by(|(_, score1), (_, score2)| 
            if turn == Color::White {
                score2.partial_cmp(score1).unwrap()
            } else {
                score1.partial_cmp(score2).unwrap()
            }
        );

        transposition_table.insert(board.clone(), TranspositionTableContent {
            result: result.clone(),
            depth,
        });
        result
    }
}
