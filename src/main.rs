use std::{time, thread};
use gameoflife::Board;

const HEIGHT:usize=50;
const WIDTH:usize=50;

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);
    loop {
        board.random_state();
        board.render();
        thread::sleep(time::Duration::from_millis(300));
    }
}
