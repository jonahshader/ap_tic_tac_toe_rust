use std::io;
mod game_match;
use game::game_match::board;
use game::game_match::board::MoveResult;

pub struct Game {
    game_match: game_match::GameMatch,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game_match: Default::default(),
        }
    }
}

impl Game {
    pub fn play(&mut self) {
        // print welcome info
        println!("Welcome to Meta Tic Tac Toe!");

        println!("{} plays.", self.game_match.get_current_player());
        println!("Enter meta X: ");
        let mut pos_meta_x = String::new();
        io::stdin().read_line(&mut pos_meta_x);
        let pos_meta_x: usize = pos_meta_x.trim().parse()
            .expect("Please type a number!");

        println!("Enter meta Y: ");
        let mut pos_meta_y = String::new();
        io::stdin().read_line(&mut pos_meta_y);
        let pos_meta_y: usize = pos_meta_y.trim().parse()
            .expect("Please type a number!");

        println!("Enter X: ");
        let mut pos_x = String::new();
        io::stdin().read_line(&mut pos_x);
        let pos_x: usize = pos_x.trim().parse()
            .expect("Please type a number!");

        println!("Enter Y: ");
        let mut pos_y = String::new();
        io::stdin().read_line(&mut pos_y);
        let pos_y: usize = pos_y.trim().parse()
            .expect("Please type a number!");

        self.game_match.first_move((pos_meta_x, pos_meta_y), (pos_x, pos_y));

        while !self.game_match.is_game_done() {
            self.game_match.print_board();

            println!("{} plays in {:?} section.", self.game_match.get_current_player(), self.game_match.get_last_move());
            println!("Enter X: ");
            let mut pos_x = String::new();
            io::stdin().read_line(&mut pos_x);
            let pos_x: usize = pos_x.trim().parse()
                .expect("Please type a number!");

            println!("Enter Y: ");
            let mut pos_y = String::new();
            io::stdin().read_line(&mut pos_y);
            let pos_y: usize = pos_y.trim().parse()
                .expect("Please type a number!");

            match self.game_match.try_move((pos_x, pos_y)) {
                MoveResult::Fail => {println!("Move failed. Someone already played there!");},
                MoveResult::Success => {println!("Move successful.");},
                MoveResult::Win => {println!("{} wins!", self.game_match.get_current_player());},
            }
        }
    }
}