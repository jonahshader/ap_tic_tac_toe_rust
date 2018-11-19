extern crate rand;

use std::io;
//use rand::Rng;
//use std::cmp::Ordering;

enum MoveResult {
    Fail,
    Success,
    Win,
}

struct Board {
    board: [[i8; 3]; 3],
    winner: i8,
    moves: u8,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board: [[0i8; 3]; 3],
            winner: 0,
            moves: 0,
        }
    }
}

impl Board {
    /// returns true if a move at the x y location is legal
    pub fn check_can_move(&self, x: u8, y: u8) -> bool {
        self.board[x as usize][y as usize] == 0
    }

    /// attempts to make a move at the x y location. returns MoveResult
    pub fn try_move(&mut self, x: u8, y: u8, tile_value: i8) -> MoveResult {
        if self.check_can_move(x, y) {              // if we can move here, then
            self.force_move(x, y, tile_value);      // move here
            if self.check_win(x, y, tile_value) {   // check if that was a winning move
                self.winner = tile_value;           // if it was, write down who won in winner value
                return MoveResult::Win;             // return Win
            } else {
                return MoveResult::Success;         // if that wasn't a winning move, the move was only successful
            }
        }
        MoveResult::Fail                            // if we can't move here, return Fail
    }

    /// prints the board to the console
    pub fn print_board(&self) {
        for y in 0..3 {
            for x in 0..3 {
                // print out each cell on the board.
                // if the number is less than zero, we print. if its greater, leave a space for a negative sign
                // to keep print alligned
                if self.board[x][y] < 0 {
                    print!("{}", self.board[x][y]);
                } else {
                    print!(" {}", self.board[x][y]);
                }
            }
            println!();
        }
    }

    /// sets board value at x y to tile_value
    fn force_move(&mut self, x: u8, y: u8, tile_value: i8) {
        self.board[x as usize][y as usize] = tile_value;            // set the value at x y to tile_value
        self.moves += 1;                                            // increment move counter
    }

    /// checks if the board is in a winning state given x y was the last move
    fn check_win(&self, x: u8, y: u8, tile_value: i8) -> bool {
        if self.moves >= 3 {                                        // must have 3 or more moves for a win to occur
            if self.check_horizontal_vertical(x, y, tile_value) {   // check horizontal vertical right away
                return true;
            }
            if (x == 0 && y == 0) || (x == 2 && y == 2) {           // if xy is top left or bottom right, check corresponding diagonal
                if self.check_diagonal_top_left(tile_value) {
                    return true;
                }
            } else if (x == 0 && y == 2) || (x == 2 && y == 0) {    // else if xy is top right or bottom left, check corresponding diagonal
                if self.check_diagonal_top_right(tile_value) {
                    return true;
                }
            } else if x == 1 && y == 1 {                            // else if xy is the center, check both diagonals
                if self.check_diagonal_top_left(tile_value) {
                    return true;
                }
                if self.check_diagonal_top_right(tile_value) {
                    return true;
                }
            }
        }

        false                                                       // none of the previous methods found a win
    }

    fn check_horizontal_vertical(&self, x: u8, y: u8, tile_value: i8) -> bool {
        let mut success = true;
        //check horizontal
        for i in 0..3 {
            if self.board[i as usize][y as usize] != tile_value {
                success = false;
            }
        }


        if success {
            return true;        // return true if the horizontal check was successful
        } else {                // else, set success to true again for the vertical check
            success = true;
        }

        //check vertical
        for i in 0..3 {
            if self.board[x as usize][i as usize] != tile_value {
                success = false;
            }
        }

        if success {
            return true;        // return true if the vertical check was successful
        }
        false                   // neither checks were successful. return false
    }

    /// checks diagonal from top right to bottom left for win
    fn check_diagonal_top_right(&self, tile_value: i8) -> bool {
        for i in 0..3 {
            if self.board[2 - i][i] != tile_value {
                return false;
            }
        }
        true
    }

    /// checks diagonal from top left to bottom right for win
    fn check_diagonal_top_left(&self, tile_value: i8) -> bool {
        for i in 0..3 {
            if self.board[i][i] != tile_value {
                return false;
            }
        }
        true
    }
}



fn main() {

    let mut temp_board: Board = Default::default();

    loop {
        temp_board.print_board();

        println!("Enter X coordinate to play: ");

        let mut pos_x = String::new();

        io::stdin().read_line(&mut pos_x)
            .expect("Failed to read line");

        let pos_x: u8 = pos_x.trim().parse()
            .expect("Please type a number!");

        println!("Enter Y coordinate to play: ");

        let mut pos_y = String::new();

        io::stdin().read_line(&mut pos_y)
            .expect("Failed to read line");

        let pos_y: u8 = pos_y.trim().parse()
            .expect("Please type a number!");

        println!("Enter player: ");

        let mut player = String::new();

        io::stdin().read_line(&mut player)
            .expect("Failed to read line");

        let player: i8 = player.trim().parse()
            .expect("Please type a number!");

        match temp_board.try_move(pos_x, pos_y, player) {
            MoveResult::Fail => println!("Failed to move. Spot already taken!"),
            MoveResult::Success => println!("Moved."),
            MoveResult::Win => println!("Moved. Game over!"),
        }

    }

//    println!("Please input your guess.");
//
//    let mut guess = String::new();
//
//    io::stdin().read_line(&mut guess)
//        .expect("Failed to read line");
//
//    let guess: u32 = guess.trim().parse()
//        .expect("Please type a number!");
//
//    println!("You guessed: {}", guess);

}

//fn print_board(board_print: &[[[[i8; 3]; 3]; 3]; 3]) {
//    for y in 0..3 {
//        for k in 0..3 {
//            for x in 0..3 {
//                for j in 0..3 {
//                    if board_print[x][y][j][k] >= 0 {
//                        print!(" {}", board_print[x][y][j][k])
//                    } else {
//                        print!("{}", board_print[x][y][j][k])
//                    }
//
//                }
//            }
//            println!();
//        }
//    }
//
//}
//
//fn ask_player_move(player_name: String) {
//    println!("{}, pick your move.", player_name);
//}