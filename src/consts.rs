use crate::types::*;
use crate::board::Board;

pub const EMPTY_MOVE: Move = Move {
    start_square: 64,
    end_square: 64,
    promotion: PieceType::Empty,
};

pub const EMPTY_PIECE: Piece = Piece {
    typ: PieceType::Empty,
    color: Color::Empty,
};

pub const EMPTY_BOARD: Board = Board {
    white_pieces: 0,
    black_pieces: 0,
    pawns: 0,
    rooks: 0,
    knights: 0,
    bishops: 0,
    queens: 0,
    kings: 0,

    turn: Color::White,
    castling_rights: ((false, false), (false, false)),
    en_passant: 0,
    halfmove_clock: 0,
    fullmove_clock: 0,
    
    moves: Vec::new(),
};
