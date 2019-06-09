const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone)]
enum Side {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
enum Piece {
    Pawn(Side),
    Knight(Side),
    Bishop(Side),
    Rook(Side),
    Queen(Side),
    King(Side),
}

#[derive(Debug)]
struct Board {
    squares: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    fn back_row_of_side(side: Side) -> [Option<Piece>; BOARD_SIZE] {
        [
            Some(Piece::Rook(side)),
            Some(Piece::Knight(side)),
            Some(Piece::Bishop(side)),
            Some(Piece::Queen(side)),
            Some(Piece::King(side)),
            Some(Piece::Bishop(side)),
            Some(Piece::Knight(side)),
            Some(Piece::Rook(side)),
        ]
    }

    fn pawn_row_of_side(side: Side) -> [Option<Piece>; BOARD_SIZE] {
        [Some(Piece::Pawn(side)); BOARD_SIZE]
    }

    fn empty_row() -> [Option<Piece>; BOARD_SIZE] {
        [None::<Piece>; BOARD_SIZE]
    }

    fn starting_board() -> Board {
        Board {
            squares: [
                Board::back_row_of_side(Side::White),
                Board::pawn_row_of_side(Side::White),
                Board::empty_row(),
                Board::empty_row(),
                Board::empty_row(),
                Board::empty_row(),
                Board::pawn_row_of_side(Side::Black),
                Board::back_row_of_side(Side::Black),
            ]
        }
    }
}

fn main() {
    let starting_board = Board::starting_board();

    println!("Starting board: {:#?}", starting_board);
}
