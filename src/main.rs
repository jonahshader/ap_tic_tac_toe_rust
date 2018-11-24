extern crate rand;
mod board;

use std::io;
use board::MoveResult;
use board::Board;
use board::MetaBoard;
//use rand::Rng;
//use std::cmp::Ordering;

fn main() {

    let mut temp_board: Board = Default::default();

    let mut temp_meta_board: MetaBoard = Default::default();

    temp_meta_board.print_board();

//    loop {
//        temp_board.print_board();
//
//        println!("Enter X coordinate to play: ");
//
//        let mut pos_x = String::new();
//
//        io::stdin().read_line(&mut pos_x)
//            .expect("Failed to read line");
//
//        let pos_x: u8 = pos_x.trim().parse()
//            .expect("Please type a number!");
//
//        println!("Enter Y coordinate to play: ");
//
//        let mut pos_y = String::new();
//
//        io::stdin().read_line(&mut pos_y)
//            .expect("Failed to read line");
//
//        let pos_y: u8 = pos_y.trim().parse()
//            .expect("Please type a number!");
//
//        println!("Enter player: ");
//
//        let mut player = String::new();
//
//        io::stdin().read_line(&mut player)
//            .expect("Failed to read line");
//
//        let player: i8 = player.trim().parse()
//            .expect("Please type a number!");
//
//        match temp_board.try_move((pos_x as usize, pos_y as usize), player) {
//            MoveResult::Fail => println!("Failed to move. Spot already taken!"),
//            MoveResult::Success => println!("Moved."),
//            MoveResult::Win => println!("Moved. Game over!"),
//        }
//
//    }

}
