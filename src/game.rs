use crate::pieces::{Colored, Pawn, Piece};

// repeat an expression 8 times in an array
macro_rules! row {
    ($expr:expr) => {
        [$expr, $expr, $expr, $expr, $expr, $expr, $expr, $expr]
    };
}

#[derive(Default, Debug)]
pub struct Game {
    pub game_board: GameBoard,
}

#[derive(Debug)]
pub struct GameBoard {
    pub board: [[Option<Box<dyn Piece>>; 8]; 8],
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            board: [
                row!(Some(Box::new(Pawn::black()))),
                row!(Some(Box::new(Pawn::black()))),
                row!(None),
                row!(None),
                row!(None),
                row!(None),
                row!(Some(Box::new(Pawn::white()))),
                row!(Some(Box::new(Pawn::white()))),
            ],
        }
    }
}
