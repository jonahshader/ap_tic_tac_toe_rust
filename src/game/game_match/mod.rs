use game::game_match::board::MoveResult;
use game::game_match::board::MetaBoard;

pub mod board;

pub struct GameMatch {
    meta_board: MetaBoard,
    current_player: i8,
    game_done: bool,
    last_sub_move: (usize, usize),
    move_count: u8,
}

impl Default for GameMatch {
    fn default() -> Self {
        GameMatch {
            meta_board: Default::default(),
            current_player: 1,
            game_done: false,
            last_sub_move: (0, 0),
            move_count: 0,
        }
    }
}

impl GameMatch {
    pub fn get_current_player(&self) -> i8 {
        self.current_player
    }

    pub fn print_board(&self) {
        self.meta_board.print_board();
    }

    pub fn get_last_move(&self) -> (usize, usize) {
        self.last_sub_move
    }

    pub fn is_game_done(&self) -> bool {
        self.game_done
    }

    pub fn first_move(&mut self, meta_pos: (usize, usize), pos: (usize, usize)) {
        if self.move_count == 0 {
            self.meta_board.try_move(meta_pos, pos, self.current_player);
        }
    }

    pub fn try_move(&mut self, pos: (usize, usize)) -> MoveResult {
        let move_result = self.meta_board.try_move(self.last_sub_move, pos, self.current_player);

        match move_result {
            MoveResult::Fail => {}

            MoveResult::Success => {
                if self.current_player == -1 {
                    self.current_player = 1;
                } else {
                    self.current_player = -1;
                }

                self.last_sub_move = pos;
            }

            MoveResult::Win => {
                self.game_done = true;
            }
        }

        move_result
    }
}