
/// the result of a move on a 3x3 tic tac toe board
pub enum MoveResult {
    Fail,       // move could not be complete
    Success,    // move successful, but not a win
    Win,        // move successful and was a winning move
}

/// a single 3x3 tic tac toe board
#[derive(Copy, Clone)]
pub struct Board {
    board: [[i8; 3]; 3],
    winner: i8,
    moves: u8,
}

/// default board implementation
impl Default for Board {
    fn default() -> Self {
        Board {
            board: [[0i8; 3]; 3],
            winner: 0,
            moves: 0,
        }
    }
}

/// methods for Board
impl Board {
    /// returns a reference to the board
//    pub fn get_board(&self) -> &[[i8; 3]; 3] {
//        &self.board
//    }

    /// returns winner
//    pub fn get_winner(&self) -> i8 {
//        self.winner
//    }

    /// returns true if a move at pos is legal
    pub fn check_can_move(&self, pos: (usize, usize)) -> bool {
        self.board[pos.0][pos.1] == 0
    }

    /// attempts to make a move at pos. returns MoveResult
    pub fn try_move(&mut self, pos: (usize, usize), tile_value: i8) -> MoveResult {
        if self.check_can_move(pos) {
            self.force_move(pos, tile_value);
            if self.check_win(pos, tile_value) {
                self.winner = tile_value;
                return MoveResult::Win;
            } else {
                return MoveResult::Success;
            }
        }
        MoveResult::Fail
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

    /// prints a single spot on the board
    pub fn print_tile(&self, pos: (usize, usize)) {
        if self.board[pos.0][pos.1] < 0 {
            print!("{}", self.board[pos.0][pos.1]);
        } else {
            print!(" {}", self.board[pos.0][pos.1]);
        }
    }

    /// sets board value at pos to tile_value
    fn force_move(&mut self, pos: (usize, usize), tile_value: i8) {
        self.board[pos.0][pos.1] = tile_value;
        self.moves += 1;
    }

    /// checks if the board is in a winning state given pos was the last move
    fn check_win(&self, pos: (usize, usize), tile_value: i8) -> bool {
        let (x, y) = pos;
        if self.moves >= 3 {                                        // must have 3 or more moves for a win to occur
            if self.check_horizontal_vertical(pos, tile_value) {    // check horizontal vertical right away
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

        false
    }

    fn check_horizontal_vertical(&self, pos: (usize, usize), tile_value: i8) -> bool {
        let mut success = true;
        //check horizontal
        for i in 0..3 {
            if self.board[i as usize][pos.1] != tile_value {
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
            if self.board[pos.0][i as usize] != tile_value {
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

pub struct MetaBoard {
    board: [[Board; 3]; 3],
    winner: i8,
    moves: u8,
}

impl Default for MetaBoard {
    fn default() -> Self {
        MetaBoard {
            board: [[Default::default(); 3]; 3],
            winner: 0,
            moves: 0
        }
    }
}

impl MetaBoard {
    /// returns true if a move at pos is legal
    pub fn check_can_move(&self, meta_pos: (usize, usize)) -> bool {
        self.board[meta_pos.0][meta_pos.1].winner == 0
    }

    pub fn try_move(&mut self, meta_pos: (usize, usize), pos: (usize, usize), tile_value: i8) -> MoveResult {
        MoveResult::Fail
    }

    pub fn print_board(&self) {
        // top horizontal line
        self.print_horizontal_line();
        println!();
        for y1 in 0..3 {
            for y2 in 0..3 {
                for x1 in 0..3 {
                    print!(" |");
                    for x2 in 0..3 {
                        self.board[x1][y1].print_tile((x2, y2));
                    }
                }
                print!(" |");
                println!(); // new line
            }


            // horizontal line under every 3 3x3 boards
            self.print_horizontal_line();
            println!();
        }
    }

    fn print_horizontal_line(&self) {
        print!("  ");
        for i in 0..3 {

            for j in 0..3 {
                print!("--");
            }
            print!("  ");
        }
    }

    fn force_move(&mut self, meta_pos: (usize, usize), pos: (usize, usize), tile_value: i8) {
        self.board[meta_pos.0][meta_pos.1].force_move(pos, tile_value);
        self.moves += 1;
    }

    fn check_win(&self, meta_pos: (usize, usize), tile_value: i8) -> bool {
        let (x, y) = meta_pos;
        if self.moves >= 3 {                                        // must have 3 or more moves for a win to occur
            if self.check_horizontal_vertical(meta_pos, tile_value) {    // check horizontal vertical right away
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

        false
    }

    fn check_horizontal_vertical(&self, meta_pos: (usize, usize), tile_value: i8) -> bool {
        let mut success = true;
        //check horizontal
        for i in 0..3 {
            if self.board[i as usize][meta_pos.1 as usize].winner != tile_value {
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
            if self.board[meta_pos.0 as usize][i as usize].winner != tile_value {
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
            if self.board[2 - i][i].winner != tile_value {
                return false;
            }
        }
        true
    }

    /// checks diagonal from top left to bottom right for win
    fn check_diagonal_top_left(&self, tile_value: i8) -> bool {
        for i in 0..3 {
            if self.board[i][i].winner != tile_value {
                return false;
            }
        }
        true
    }

}