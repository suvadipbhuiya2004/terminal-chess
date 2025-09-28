// positive means white
// negative means black
//
// 1 means pawn
// 2 means king
// 3 means queen
// 4 means bishop
// 5 means knight
// 6 means rook

fn main() {
    let mut board = [[0i8; 8]; 8];

    // setup pawns
    board[1].fill(-1);
    board[6].fill(1);

    // setup rooks
    board[0][0] = -6;
    board[0][7] = -6;
    board[7][0] = 6;
    board[7][7] = 6;

    // setup knights
    board[0][1] = -5;
    board[0][6] = -5;
    board[7][1] = 5;
    board[7][6] = 5;

    // setup bishops
    board[0][2] = -4;
    board[0][5] = -4;
    board[7][2] = 4;
    board[7][5] = 4;

    // setup kings
    board[0][4] = -2;
    board[7][4] = 2;

    // setup queens
    board[0][3] = -3;
    board[7][3] = 3;
    
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
