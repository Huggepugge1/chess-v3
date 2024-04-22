use crate::board::Board;

use std::collections::HashMap;

pub type Square = usize;
pub type Castling = ((bool, bool), (bool, bool));
pub type Clock = u8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Empty,
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    pub start_square: Square,
    pub end_square:   Square,
    pub promotion:    PieceType,
}

impl Move {
    pub fn new(start_square: Square, end_square: Square, promotion: PieceType) -> Self {
        Move {
            start_square: start_square,
            end_square: end_square,
            promotion: promotion,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    pub typ: PieceType,
    pub color: Color,
}

pub struct TranspositionTableContent {
    pub result: Vec<(Move, i32)>,
    pub depth: i32,
}

pub type TranspositionTable = HashMap<Board, TranspositionTableContent>;