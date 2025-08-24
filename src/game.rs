use crate::pieces::{Colored, Coord, Pawn, Piece};

// repeat an expression 8 times in an array
macro_rules! row {
    ($expr:expr) => {
        [$expr, $expr, $expr, $expr, $expr, $expr, $expr, $expr]
    };
}

#[derive(Default, Debug)]
pub struct Game {
    game_board: GameBoard,
}

impl Game {
    pub fn make_move(&mut self, from: Coord, to: Coord) {
        let piece = self.game_board.board[from.row][from.col].take();
        self.game_board.board[to.row][to.col] = piece;
    }

    pub fn get_cell(&self, coord: Coord) -> Option<&Box<dyn Piece>> {
        self.game_board.board[coord.row][coord.col].as_ref()
    }
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
