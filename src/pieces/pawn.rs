use crate::game::GameBoard;
use crate::pieces::{Coord, Pawn, Piece};

impl Piece for Pawn {
    fn acceptable_moves(&self, coordinates: &Coord, game_board: &GameBoard) -> Vec<Coord> {
        Vec::new()
    }

    fn to_string(&self) -> &'static str {
        "\
        \n\
        \n\
      ▟█▙\n\
      ▜█▛\n\
     ▟███▙\n\
    "
    }
}
