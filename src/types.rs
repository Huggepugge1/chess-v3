pub type Square = usize;
pub type Castling = ((bool, bool), (bool, bool));
pub type Move = (Square, Square);
pub type Clock = u8;

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Empty,
    White,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PieceType {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub typ: PieceType,
    pub color: Color,
}
