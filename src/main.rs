use std::{time, thread};
use gameoflife::Board;

const HEIGHT:usize = 25;
const WIDTH:usize = 40;

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);
    let mut gen:u32 = 0;
    loop {
        //resets after every 100 generations
        if gen%100==0 {board.random_state()} else {board.next_state()}
        board.render(gen);
        gen+=1;
        thread::sleep(time::Duration::from_millis(300));
    }
}
