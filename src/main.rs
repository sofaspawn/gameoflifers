use std::{time, thread};
use gameoflife::Board;

const HEIGHT:usize = 25;
const WIDTH:usize = 40;

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);
    let mut i = 0;
    loop {
        if i%10==0 {board.random_state()} else {board.next_state()}
        board.render();
        i+=1;
        thread::sleep(time::Duration::from_millis(500));
    }
}
