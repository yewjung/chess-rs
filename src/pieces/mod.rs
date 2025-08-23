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
            }
        )*
    };
}

piece_structs!(Rook, Knight, Bishop, Queen, King, Pawn);

pub trait Colored {
    fn ratatui_color(&self) -> Color;
}
pub trait Piece: Debug + Colored {
    fn acceptable_moves(&self, coordinates: &Coord, game_board: &GameBoard) -> Vec<Coord>;

    fn to_string(&self) -> &'static str;
}

pub struct Coord {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
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
