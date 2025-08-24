mod pawn;

use crate::game::GameBoard;
use ratatui::style::Color;
use std::fmt::Debug;

macro_rules! piece_structs {
    ($($name:ident),*) => {
        $(
            #[derive(Debug)]
            pub struct $name {
                pub color: PieceColor,
            }

            impl $name {
                pub fn white() -> Self {
                    Self {
                        color: PieceColor::White,
                    }
                }

                pub fn black() -> Self {
                    Self {
                        color: PieceColor::Black,
                    }
                }
            }

            impl Colored for $name {
                fn ratatui_color(&self) -> Color {
                    self.color.ratatui_color()
                }
                fn piece_color(&self) -> PieceColor {
                    self.color
                }
            }
        )*
    };
}

piece_structs!(Rook, Knight, Bishop, Queen, King, Pawn);

pub trait Colored {
    fn ratatui_color(&self) -> Color;
    fn piece_color(&self) -> PieceColor;
}
pub trait Piece: Debug + Colored {
    fn acceptable_moves(&self, coordinates: &Coord, game_board: &GameBoard) -> Vec<Coord>;

    fn to_string(&self) -> &'static str;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}

impl Coord {
    pub fn some(row: usize, col: usize) -> Option<Self> {
        Some(Self::from((row, col)))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn ratatui_color(&self) -> Color {
        match self {
            PieceColor::White => Color::White,
            PieceColor::Black => Color::Black,
        }
    }
}
