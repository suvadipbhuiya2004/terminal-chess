#[derive(PartialEq)]
enum PieceType {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

#[derive(PartialEq)]
enum Color {
    White,
    Black,
}

struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    fn piece_new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }
    
    fn piece_get_value(&self) -> i8 {
        if self.color == Color::White {
            match self.piece_type {
                PieceType::Pawn => 1,
                PieceType::King => 2,
                PieceType::Queen => 3,
                PieceType::Bishop => 4,
                PieceType::Knight => 5,
                PieceType::Rook => 6,
            }
        } else {
            match self.piece_type {
                PieceType::Pawn => -1,
                PieceType::King => -2,
                PieceType::Queen => -3,
                PieceType::Bishop => -4,
                PieceType::Knight => -5,
                PieceType::Rook => -6,
            }
        }
    }
}

fn main() {
    
    let pawn_white = Piece::piece_new(PieceType::Pawn, Color::White);
    let knight_white = Piece::piece_new(PieceType::Knight, Color::White);
    let rook_white = Piece::piece_new(PieceType::Rook, Color::White);
    let bishop_white = Piece::piece_new(PieceType::Bishop, Color::White);
    let queen_white = Piece::piece_new(PieceType::Queen, Color::White);
    let king_white = Piece::piece_new(PieceType::King, Color::White);
    
    let pawn_black = Piece::piece_new(PieceType::Pawn, Color::Black);
    let knight_black = Piece::piece_new(PieceType::Knight, Color::Black);
    let rook_black = Piece::piece_new(PieceType::Rook, Color::Black);
    let bishop_black = Piece::piece_new(PieceType::Bishop, Color::Black);
    let queen_black = Piece::piece_new(PieceType::Queen, Color::Black);
    let king_black = Piece::piece_new(PieceType::King, Color::Black);
    
    let mut board = [[0i8; 8]; 8];

    // setup pawns
    board[1].fill(pawn_black.piece_get_value());
    board[6].fill(pawn_white.piece_get_value());
    
    // setup rooks
    board[0][0] = rook_black.piece_get_value();
    board[0][7] = rook_black.piece_get_value();
    board[7][0] = rook_white.piece_get_value();
    board[7][7] = rook_white.piece_get_value();

    // setup knights
    board[0][1] = knight_black.piece_get_value();
    board[0][6] = knight_black.piece_get_value();
    board[7][1] = knight_white.piece_get_value();
    board[7][6] = knight_white.piece_get_value();

    // setup bishops
    board[0][2] = bishop_black.piece_get_value();
    board[0][5] = bishop_black.piece_get_value();
    board[7][2] = bishop_white.piece_get_value();
    board[7][5] = bishop_white.piece_get_value();

    // setup kings
    board[0][4] = king_black.piece_get_value();
    board[7][4] = king_white.piece_get_value();

    // setup queens
    board[0][3] = queen_black.piece_get_value();
    board[7][3] = queen_white.piece_get_value();
    
    println!();
    println!("Initial board setup:");
    board_print(board);
}

fn board_print(board: [[i8; 8]; 8]) {
    println!();
    println!("       A     B     C     D     E     F     G     H");
    println!("    * --- * --- * --- * --- * --- * --- * --- * --- *");
    for i in 0..8 {
        println!("    |     |     |     |     |     |     |     |     |");
        print!(" {}  |", 8 - i);
        for j in 0..8 {
            if board[i][j] < 0 {
                print!(" {}  |", board[i][j]);
            } 
            else if board[i][j] > 0 {
                print!(" +{}  |", board[i][j]);
            }
            else {
                print!("     |");
            }
        }
        println!();
        println!("    |     |     |     |     |     |     |     |     |");
        println!("    * --- * --- * --- * --- * --- * --- * --- * --- *");
    }
    println!();
}
